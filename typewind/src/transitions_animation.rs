use typewind_macros::{Display, Parse};

tailwind_types!(TransitionProperty, TransitionDuration, TransitionTimingFunction, TransitionDelay, Animation);

/// Utilities for controlling which CSS properties transition.
/// 
/// <https://tailwindcss.com/docs/transition-property>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "transition")]
pub enum TransitionProperty {
    /// ```css
    /// {
    ///     transition-property: none;
    /// }
    /// ```
    None,
    /// ```css
    /// {
    ///     transition-property: all;
    ///     transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    ///     transition-duration: 150ms;
    /// }
    /// ```
    All,
    /// ```css
    /// {
    ///     transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;
    ///     transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    ///     transition-duration: 150ms;
    /// }
    /// ```
    #[display(no_prefix)]
    Transition,
    /// ```css
    /// {
    ///     transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;
    ///     transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    ///     transition-duration: 150ms;
    /// }
    /// ```
    Colors,
    /// ```css
    /// {
    ///     transition-property: opacity;
    ///     transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    ///     transition-duration: 150ms;
    /// }
    /// ```
    Opacity,
    /// ```css
    /// {
    ///     transition-property: box-shadow;
    ///     transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    ///     transition-duration: 150ms;
    /// }
    /// ```
    Shadow,
    /// ```css
    /// {
    ///     transition-property: transform;
    ///     transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    ///     transition-duration: 150ms;
    /// }
    /// ```
    Transform,
}

/// Utilities for controlling the duration of CSS transitions.
/// 
/// <https://tailwindcss.com/docs/transition-duration>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "duration")]
pub enum TransitionDuration {
    /// ```css
    /// {
    ///     transition-duration: 0s;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     transition-duration: 75ms;
    /// }
    /// ```
    _75,
    /// ```css
    /// {
    ///     transition-duration: 100ms;
    /// }
    /// ```
    _100,
    /// ```css
    /// {
    ///     transition-duration: 150ms;
    /// }
    /// ```
    _150,
    /// ```css
    /// {
    ///     transition-duration: 200ms;
    /// }
    /// ```
    _200,
    /// ```css
    /// {
    ///     transition-duration: 300ms;
    /// }
    /// ```
    _300,
    /// ```css
    /// {
    ///     transition-duration: 500ms;
    /// }
    /// ```
    _500,
    /// ```css
    /// {
    ///     transition-duration: 700ms;
    /// }
    /// ```
    _700,
    /// ```css
    /// {
    ///     transition-duration: 1000ms;
    /// }
    /// ```
    _1000,
}

/// Utilities for controlling the easing of CSS transitions.
/// 
/// <https://tailwindcss.com/docs/transition-timing-function>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "ease")]
pub enum TransitionTimingFunction {
    /// ```css
    /// {
    ///     transition-timing-function: linear;
    /// }
    /// ```
    Linear,
    /// ```css
    /// {
    ///     transition-timing-function: cubic-bezier(0.4, 0, 1, 1);
    /// }
    /// ```
    In,
    /// ```css
    /// {
    ///     transition-timing-function: cubic-bezier(0, 0, 0.2, 1);
    /// }
    /// ```
    Out,
    /// ```css
    /// {
    ///     transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
    /// }
    /// ```
    InOut,
}

/// Utilities for controlling the delay of CSS transitions.
/// 
/// <https://tailwindcss.com/docs/transition-delay>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "delay")]
pub enum TransitionDelay {
    /// ```css
    /// {
    ///     transition-delay: 0s;
    /// }
    /// ```
    _0,
    /// ```css
    /// {
    ///     transition-delay: 75ms;
    /// }
    /// ```
    _75,
    /// ```css
    /// {
    ///     transition-delay: 100ms;
    /// }
    /// ```
    _100,
    /// ```css
    /// {
    ///     transition-delay: 150ms;
    /// }
    /// ```
    _150,
    /// ```css
    /// {
    ///     transition-delay: 200ms;
    /// }
    /// ```
    _200,
    /// ```css
    /// {
    ///     transition-delay: 300ms;
    /// }
    /// ```
    _300,
    /// ```css
    /// {
    ///     transition-delay: 500ms;
    /// }
    /// ```
    _500,
    /// ```css
    /// {
    ///     transition-delay: 700ms;
    /// }
    /// ```
    _700,
    /// ```css
    /// {
    ///     transition-delay: 1000ms;
    /// }
    /// ```
    _1000,
}

/// Utilities for animating elements with CSS animations.
/// 
/// <https://tailwindcss.com/docs/animation>
#[derive(Debug, Clone, PartialEq, Display, Parse)]
#[display(prefix = "animate")]
pub enum Animation {
    /// ```css
    /// {
    ///     animation: none;
    /// }
    /// ```
    None,
    /// ```css
    /// animation: spin 1s linear infinite;
    /// 
    /// @keyframes spin {
    ///   from {
    ///     transform: rotate(0deg);
    ///   }
    ///   to {
    ///     transform: rotate(360deg); 
    ///   }
    /// }
    /// ```
    Spin,
    /// ```css
    /// animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;
    /// 
    /// @keyframes ping {
    ///   75%, 100% {
    ///     transform: scale(2);
    ///     opacity: 0;
    ///   }
    /// }
    /// ```
    Ping,
    /// ```css
    /// animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
    /// 
    /// @keyframes pulse {
    ///   0%, 100% {
    ///     opacity: 1;
    ///   }
    ///   50% {
    ///     opacity: .5;`
    ///   }
    /// }
    /// ```
    Pulse,
    /// ```css 
    /// animation: bounce 1s infinite;
    /// 
    /// @keyframes bounce {
    ///   0%, 100% {
    ///     transform: translateY(-25%);
    ///     animation-timing-function: cubic-bezier(0.8, 0, 1, 1);
    ///   }
    ///   50% {
    ///     transform: translateY(0);
    ///     animation-timing-function: cubic-bezier(0, 0, 0.2, 1);
    ///   }
    /// }
    /// ```
    Bounce,
}

