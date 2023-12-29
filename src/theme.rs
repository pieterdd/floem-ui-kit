use floem::reactive::{create_rw_signal, RwSignal};

use crate::accents::AccentColor;

/// Allows you to create any of the widgets supported by Floem UI Kit. The
/// `Theme` struct contains any settings that will apply across the entire
/// UI.
///
/// ```
/// use floem_ui_kit::accents::AccentColor;
/// use floem_ui_kit::theme::Theme;
///
/// let theme = Theme {
///     ..Default::default()
/// };
/// ```
#[derive(Clone, Copy)]
pub struct Theme {
    /// Controls color variations of the theme. Wrapped in a signal so your
    /// UI may change appearance without restarting.
    pub accent_color: RwSignal<AccentColor>,

    /// UI elements generally shouldn't stick to the edge of the window,
    /// but exceptions are sometimes necessary for decorative elements.
    /// To cover your use cases, you can set a horizontal window margin here.
    /// It does not do anything by default. You may however:
    ///
    /// - Make use of the `padded_container` function to wrap your layout
    ///   in an amount of padding equal to the target window margin.
    /// - Read the value of this field in your own code whenever you prefer
    ///   to manually insert horizontal window margin.
    ///
    /// Decorative UI elements containing text (such as Floem UI Kit's own
    /// header) may hook in to this field to keep different parts of the window
    /// layout aligned.
    pub horizontal_window_margin: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            accent_color: create_rw_signal(AccentColor::Magenta),
            horizontal_window_margin: 20.0,
        }
    }
}
