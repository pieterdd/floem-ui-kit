use floem::view::View;
use floem::views::container;
use floem::views::v_stack;
use floem::views::Decorators;

use crate::theme::Theme;

impl Theme {
    /// Wraps a stack of UI elements in the theme's configured horizontal window margin, and
    /// applies a padding of the same amount in vertical direction for symmetry purposes.
    /// You'd ordinarily do this to prevent form widgets from sticking to the side of the window.
    pub fn padded_container<V: View + 'static>(self, child: V) -> impl View {
        let padding = self.horizontal_window_margin;
        container(v_stack((child,))).style(move |s| s.width_full().padding(padding))
    }
}
