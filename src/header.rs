use floem::view::View;
use floem::views::container;
use floem::views::label;
use floem::views::Decorators;

use crate::accents::PrimaryFillColorVariant;
use crate::theme::Theme;

impl Theme {
    /// Instantiates a minimalistic decorative element containing a title.
    pub fn simple_header(self, title: &str) -> impl View {
        let compiled_title = String::from(title);

        container(label(move || compiled_title.clone()).style(|s| s.font_size(28.0))).style(
            move |s| {
                let accent_color = self.accent_color.get();

                s.padding_horiz(self.horizontal_window_margin)
                    .padding_vert(self.horizontal_window_margin / 1.4)
                    .width_full()
                    .background(
                        accent_color.primary_fill_color(PrimaryFillColorVariant::DefaultColored),
                    )
                    .border_bottom(1.5)
                    .border_color(accent_color.secondary_fill_color())
            },
        )
    }
}
