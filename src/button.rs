use std::fmt::Display;

use floem::peniko::Color;
use floem::views::{container, Decorators};
use floem::widgets::ButtonClass;
use floem::{view::View, widgets::button};

use crate::accents::{BorderColorVariant, PrimaryFillColorVariant};
use crate::theme::Theme;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Regular,
    Emphasized,
}

impl Theme {
    /// Instantiates a button widget.
    pub fn button<S: Display + 'static>(
        self,
        label_func: impl Fn() -> S + 'static,
        variant: ButtonVariant,
    ) -> impl View {
        container(button(label_func)).style(move |s| {
            s.class(ButtonClass, move |s| {
                let accent_color = self.accent_color.get();

                s.apply_if(variant == ButtonVariant::Emphasized, |s| {
                    s.active(|s| {
                        s.background(
                            accent_color.primary_fill_color(PrimaryFillColorVariant::Pressed),
                        )
                    })
                    .background(
                        accent_color.primary_fill_color(PrimaryFillColorVariant::DefaultColored),
                    )
                    .border_color(accent_color.border_color(BorderColorVariant::DefaultColored))
                    .focus(|s| {
                        s.border_color(
                            accent_color.border_color(BorderColorVariant::FocusedColored),
                        )
                        .hover(|s| {
                            s.background(
                                accent_color.primary_fill_color(PrimaryFillColorVariant::Hovered),
                            )
                            .border_color(
                                accent_color.border_color(BorderColorVariant::HoveredColored),
                            )
                        })
                    })
                    .hover(|s| {
                        s.background(
                            accent_color.primary_fill_color(PrimaryFillColorVariant::Hovered),
                        )
                        .border_color(accent_color.border_color(BorderColorVariant::HoveredColored))
                    })
                })
                .apply_if(variant == ButtonVariant::Regular, |s| {
                    s.active(|s| s.background(Color::BLACK.with_alpha_factor(0.6)))
                        .background(
                            accent_color
                                .primary_fill_color(PrimaryFillColorVariant::DefaultGrayscale),
                        )
                        .border_color(
                            accent_color.border_color(BorderColorVariant::DefaultGrayscale),
                        )
                        .focus(|s| {
                            s.border_color(
                                accent_color.border_color(BorderColorVariant::FocusedGrayscale),
                            )
                            .hover(|s| s.background(Color::BLACK.with_alpha_factor(0.2)))
                        })
                        .hover(|s| {
                            s.background(Color::BLACK.with_alpha_factor(0.2))
                                .border_color(Color::WHITE.with_alpha_factor(0.2))
                        })
                })
                .border(1.0)
                .border_radius(5.0)
                .color(Color::WHITE)
                .padding_horiz(20.0)
                .padding_vert(10.0)
                .disabled(|s| {
                    s.background(accent_color.primary_fill_color(PrimaryFillColorVariant::Disabled))
                        .border_color(accent_color.border_color(BorderColorVariant::Disabled))
                        .color(accent_color.disabled_text_color())
                })
            })
        })
    }
}
