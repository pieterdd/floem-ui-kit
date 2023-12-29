use floem::peniko::Color;
use floem::view::View;
use floem::views::container;
use floem::views::Decorators;

use crate::theme::Theme;

impl Theme {
    /// Instantiates a container that applies Floem UI Kit's window background.
    /// Always use this as the foundation of your layout.
    pub fn primary_container<V: View + 'static>(self, child: V) -> impl View {
        container(child).style(move |s| {
            s.width_full()
                .background(self.accent_color.get().primary_container_background())
                .color(Color::WHITE)
                .font_size(16.0)
        })
    }
}
