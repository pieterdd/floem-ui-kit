use std::fmt::Display;

use floem::{
    peniko::Color,
    views::{create_value_container_signals, Decorators, ValueContainer},
    widgets::{labeled_checkbox as floem_labeled_checkbox, CheckboxClass, LabeledCheckboxClass},
};

use crate::{
    accents::{BorderColorVariant, PrimaryFillColorVariant},
    theme::Theme,
};

impl Theme {
    /// Instantiates a checkbox widget controlling a boolean with a text label next to it.
    pub fn labeled_checkbox<S: Display + 'static>(
        self,
        checked: impl Fn() -> bool + Clone + 'static,
        label: impl Fn() -> S + 'static,
    ) -> ValueContainer<bool> {
        let (inbound_signal, _) = create_value_container_signals(checked.clone());

        floem_labeled_checkbox(checked, label).style(move |s| {
            s.class(CheckboxClass, |s| {
                let accent_color = self.accent_color.get();
                let is_selected = inbound_signal.get();

                let unhovered_bg_color = match is_selected {
                    true => {
                        accent_color.primary_fill_color(PrimaryFillColorVariant::DefaultColored)
                    }
                    false => {
                        accent_color.primary_fill_color(PrimaryFillColorVariant::DefaultGrayscale)
                    }
                };
                let unhovered_border_color = match is_selected {
                    true => accent_color.border_color(BorderColorVariant::DefaultColored),
                    false => accent_color.border_color(BorderColorVariant::DefaultGrayscale),
                };
                let hovered_bg_color = match is_selected {
                    true => accent_color.primary_fill_color(PrimaryFillColorVariant::Hovered),
                    false => Color::BLACK.with_alpha_factor(0.1),
                };
                let hovered_border_color = match is_selected {
                    true => accent_color.border_color(BorderColorVariant::HoveredColored),
                    false => accent_color.border_color(BorderColorVariant::HoveredGrayscale),
                };

                s.active(|s| {
                    s.background(accent_color.primary_fill_color(PrimaryFillColorVariant::Pressed))
                })
                .background(unhovered_bg_color)
                .color(Color::WHITE)
                .padding(12.0)
                .border(1.0)
                .border_color(unhovered_border_color)
                .border_radius(5.0)
                .disabled(|s| {
                    s.background(
                        self.accent_color
                            .get()
                            .primary_fill_color(PrimaryFillColorVariant::Disabled),
                    )
                    .border_color(
                        self.accent_color
                            .get()
                            .border_color(BorderColorVariant::Disabled),
                    )
                })
                .focus(|s| match is_selected {
                    true => s.border_color(
                        self.accent_color
                            .get()
                            .border_color(BorderColorVariant::FocusedColored),
                    ),
                    false => s.border_color(
                        self.accent_color
                            .get()
                            .border_color(BorderColorVariant::FocusedGrayscale),
                    ),
                })
                .hover(|s| {
                    s.background(hovered_bg_color)
                        .border_color(hovered_border_color)
                })
            })
            .class(LabeledCheckboxClass, |s| {
                s.focus(|s| s.hover(|s| s.background(Color::TRANSPARENT)))
                    .hover(|s| s.background(Color::TRANSPARENT))
                    .padding(0)
            })
        })
    }
}
