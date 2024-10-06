use darling::{FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(display))]
struct DisplayOpts {
    prefix: Option<String>,
    replace: Option<Replace>,
}

#[derive(FromMeta, Default)]
struct Replace {
    from: String,
    to: String,
}

pub fn impl_display(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = DisplayOpts::from_derive_input(&input).expect("Invalid display options");

    let name = &input.ident;
    let implementation = match &input.data {
        Data::Struct(data) => generate_struct_impl(&data.fields),
        Data::Enum(data) => generate_enum_impl(data, &opts),
        _ => panic!("Display can only be derived for structs and enums"),
    };

    let body = match opts.prefix {
        Some(prefix) => quote! { format!("{}-{}", #prefix, #implementation) },
        None => implementation,
    };

    quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", #body)
            }
        }
    }
    .into()
}

fn generate_struct_impl(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Unnamed(fields_unnamed) if fields_unnamed.unnamed.len() == 1 => {
            quote! { self.0.to_string() }
        }
        _ => panic!("Display can only be derived for structs with a single unnamed field"),
    }
}

fn generate_enum_impl(data: &syn::DataEnum, opts: &DisplayOpts) -> proc_macro2::TokenStream {
    let match_arms = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let mut kebab_variant = to_kebab_case(variant_name.to_string().trim_start_matches('_'));

        if let Some(ref replace) = opts.replace {
            kebab_variant = kebab_variant.replace(&replace.from, &replace.to);
        }

        match &variant.fields {
            Fields::Unnamed(_) => quote! {
                Self::#variant_name(inner) => format!("{}-{}", #kebab_variant, inner)
            },
            Fields::Unit => quote! { Self::#variant_name => #kebab_variant.to_string() },
            Fields::Named(_) => panic!("Named fields are not supported in Display derive"),
        }
    });

    quote! { match self { #(#match_arms,)* } }
}

fn to_kebab_case(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, ch)| match ch.is_uppercase() && i > 0 {
            true => format!("-{}", ch.to_ascii_lowercase()),
            false => ch.to_ascii_lowercase().to_string(),
        })
        .collect()
}
