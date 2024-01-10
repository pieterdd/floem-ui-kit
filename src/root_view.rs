use floem::peniko::Color;
use floem::view::View;
use floem::views::container;
use floem::views::Decorators;

use crate::theme::Theme;

impl Theme {
    /// Instantiates a container that applies Floem UI Kit's theme.
    /// Always use this as the foundation of your layout.
    pub fn root_view<V: View + 'static>(self, child: V) -> impl View {
        container(child).style(move |s| {
            s.background(self.accent_color.get().root_view_background())
                .color(Color::WHITE)
                .font_size(16.0)
                .width_full()
        })
    }
}
