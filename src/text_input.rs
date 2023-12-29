use floem::views::container;
use floem::{peniko::Color, reactive::RwSignal, style::CursorStyle, view::View};

use crate::accents::{BorderColorVariant, PrimaryFillColorVariant};
use crate::theme::Theme;
use floem::views::text_input as base_text_input;
use floem::views::Decorators;

impl Theme {
    pub fn text_input(self, rw_signal: RwSignal<String>) -> impl View {
        container(base_text_input(rw_signal).style(move |s| {
            let accent_color = self.accent_color.get();

            s.background(accent_color.primary_fill_color(PrimaryFillColorVariant::DefaultGrayscale))
                .border(1.0)
                .border_color(accent_color.border_color(BorderColorVariant::DefaultGrayscale))
                .border_radius(5.0)
                .cursor(CursorStyle::Text)
                .cursor_color(Color::WHITE.with_alpha_factor(0.5))
                .disabled(|s| {
                    let accent_color = self.accent_color.get();
                    s.background(accent_color.primary_fill_color(PrimaryFillColorVariant::Disabled))
                        .color(self.accent_color.get().disabled_text_color())
                        .cursor(CursorStyle::Default)
                })
                .hover(move |s| {
                    s.border_color(accent_color.border_color(BorderColorVariant::HoveredGrayscale))
                })
                .focus(move |s| {
                    s.border_color(accent_color.border_color(BorderColorVariant::FocusedColored))
                })
                .padding_horiz(15)
                .padding_vert(10)
                .font_size(16.0)
        }))
    }
}
