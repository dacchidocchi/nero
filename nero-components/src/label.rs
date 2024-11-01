use leptos::{
    html::{h1, h2, h3, p, span, ElementChild},
    prelude::{ClassAttribute, IntoAny, MaybeSignal},
    IntoView,
};
use typewind::{
    typography::{FontSize, LineClamp, TextAlign, TextColor, TextOverflow, TextTransform},
    ToClasses,
};

use crate::IntoComponent;

/// Possible HTML tags that can be used for labels.
pub enum LabelTag {
    /// `<h1>`
    H1,
    /// `<h2>`
    H2,
    /// `<h3>`
    H3,
    /// `<p>`
    P,
    /// `<span>`
    Span,
}

/// Represents a label with configurable properties for text color, font size,
/// line clamp, text alignment, text transform and text overflow.
#[derive(ToClasses)]
pub struct Label {
    #[tw(skip)]
    text: MaybeSignal<String>,
    #[tw(skip)]
    tag: LabelTag,
    font_size: Option<FontSize>,
    line_clamp: Option<LineClamp>,
    align: Option<TextAlign>,
    color: Option<TextColor>,
    transform: Option<TextTransform>,
    overflow: Option<TextOverflow>,
}

impl Label {
    /// Creates a new `Label` with the specified [`MaybeSignal<String>`] text.
    ///
    /// By default the label is created with the `P` tag.
    ///
    /// # Example
    /// ```
    /// use leptos::{
    ///     html::{div, ElementChild},
    ///     prelude::{signal, Get, MaybeSignal, Update},
    ///     IntoView
    /// };
    /// use nero_components::{IntoComponent, Label, LabelTag, Button};
    ///
    /// fn counter() -> impl IntoView {
    ///     let (value, set_value) = signal(0);
    ///     
    ///     div().child((
    ///         Label::new(MaybeSignal::derive(move || value.get().to_string()))
    ///             .tag(LabelTag::H1)
    ///             .into_component(),
    ///         Button::new(
    ///             Label::new("Click me!".into()),
    ///             move || set_value.update(|value| *value += 1),
    ///         )
    ///         .into_component()
    ///     ))
    /// }    
    /// ```
    pub fn new(text: MaybeSignal<String>) -> Self {
        Self {
            text,
            tag: LabelTag::P,
            font_size: None,
            line_clamp: None,
            align: None,
            color: None,
            transform: None,
            overflow: None,
        }
    }

    /// Sets the HTML tag to be used for the label (defaults to `P`).
    pub fn tag(mut self, tag: LabelTag) -> Self {
        self.tag = tag;
        self
    }

    /// Sets the `font-size` property of the label.
    pub fn font_size(mut self, font_size: FontSize) -> Self {
        self.font_size = Some(font_size);
        self
    }

    /// Sets the line clamp of the label.
    pub fn line_clamp(mut self, line_clamp: LineClamp) -> Self {
        self.line_clamp = Some(line_clamp);
        self
    }

    /// Sets the text alignment of the label.
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = Some(align);
        self
    }

    /// Sets the text color of the label.
    pub fn color(mut self, color: TextColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the `text-transform` property of the label.
    pub fn transform(mut self, transform: TextTransform) -> Self {
        self.transform = Some(transform);
        self
    }

    /// Sets the `text-overflow` property of the label.
    pub fn overflow(mut self, overflow: TextOverflow) -> Self {
        self.overflow = Some(overflow);
        self
    }
}

impl IntoComponent for Label {
    fn into_component(self) -> impl IntoView {
        let classes = self.classes();
        match self.tag {
            LabelTag::H1 => h1().class(classes).child(self.text).into_any(),
            LabelTag::H2 => h2().class(classes).child(self.text).into_any(),
            LabelTag::H3 => h3().class(classes).child(self.text).into_any(),
            LabelTag::P => p().class(classes).child(self.text).into_any(),
            LabelTag::Span => span().class(classes).child(self.text).into_any(),
        }
    }
}
