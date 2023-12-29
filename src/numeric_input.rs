use std::{
    fmt::Display,
    ops::{AddAssign, SubAssign},
    str::FromStr,
};

use floem::{
    event::EventListener,
    peniko::Color,
    reactive::{create_effect, create_rw_signal, create_signal, RwSignal},
    style::{AlignContent, AlignItems, CursorStyle},
    style_class,
    view::View,
    views::{container, h_stack, text_input, Decorators},
    widgets::button,
};
use num::Integer;

use crate::{
    accents::{BorderColorVariant, PrimaryFillColorVariant},
    theme::Theme,
};

style_class!(pub SpinboxButton);

impl Theme {
    /// Instantiates an input field that only accepts integer numeric input.
    /// Comes with up/down arrows allowing the user to increment/decrement the
    /// value in steps. If min_value and/or max_value are set, input is restricted
    /// to the given bounds.
    pub fn integer_input<T>(
        self,
        int_signal: RwSignal<T>,
        step: T,
        min_value: Option<T>,
        max_value: Option<T>,
    ) -> impl View
    where
        T: Integer + Clone + Copy + Display + FromStr + SubAssign + AddAssign + 'static,
    {
        let text_signal = create_rw_signal(int_signal.get().to_string());
        let (is_focused, set_is_focused) = create_signal(false);

        create_effect(move |_| {
            let new_text_value = int_signal.get().to_string();
            if text_signal.get_untracked() != new_text_value {
                text_signal.set(new_text_value);
            }
        });
        create_effect(move |_| {
            let text_value = text_signal.get();
            match text_value.parse::<T>() {
                Ok(extracted_int_value) => {
                    let mut clamped_int_value = extracted_int_value;
                    if let Some(some_min_value) = min_value {
                        clamped_int_value = std::cmp::max(clamped_int_value, some_min_value);
                    }
                    if let Some(some_max_value) = max_value {
                        clamped_int_value = std::cmp::min(clamped_int_value, some_max_value);
                    }
                    if int_signal.get_untracked() != clamped_int_value
                        || clamped_int_value != extracted_int_value
                    {
                        int_signal.set(clamped_int_value);
                    }
                }
                Err(_) => {
                    text_signal.set(int_signal.get().to_string());
                }
            }
        });

        container(
            h_stack((
                button(|| "▼")
                    .class(SpinboxButton)
                    .keyboard_navigatable()
                    .on_click(move |_| {
                        int_signal.update(move |i| *i -= step);
                        floem::EventPropagation::Stop
                    }),
                text_input(text_signal)
                    .keyboard_navigatable()
                    .on_event_stop(EventListener::FocusGained, move |_| {
                        set_is_focused.set(true);
                    })
                    .on_event_stop(EventListener::FocusLost, move |_| {
                        set_is_focused.set(false);
                    })
                    .style(move |s| {
                        s.align_content(AlignContent::Center)
                            .border_radius(5.0)
                            .cursor(CursorStyle::Text)
                            .cursor_color(Color::WHITE.with_alpha_factor(0.5))
                            .disabled(|s| {
                                s.color(self.accent_color.get().disabled_text_color())
                                    .cursor(CursorStyle::Default)
                            })
                            .flex_grow(1.0)
                            .font_size(16.0)
                            .focus_visible(|s| {
                                s.outline(2.0)
                                    .outline_color(Color::WHITE.with_alpha_factor(0.5))
                            })
                            .padding_horiz(5.0)
                            .padding_vert(5.0)
                            .margin_vert(5.0)
                    }),
                button(|| "▲")
                    .class(SpinboxButton)
                    .keyboard_navigatable()
                    .on_click(move |_| {
                        int_signal.update(move |i| *i += step);
                        floem::EventPropagation::Stop
                    }),
            ))
            .style(move |s| {
                let accent_color = self.accent_color.get();
                s.apply_if(is_focused.get(), |s| {
                    let accent_color = self.accent_color.get();
                    s.border_color(accent_color.border_color(BorderColorVariant::FocusedColored))
                })
                .apply_if(!is_focused.get(), |s| {
                    let accent_color = self.accent_color.get();
                    s.border_color(accent_color.border_color(BorderColorVariant::DefaultGrayscale))
                        .hover(|s| {
                            s.border_color(
                                accent_color.border_color(BorderColorVariant::HoveredGrayscale),
                            )
                        })
                })
                .align_items(AlignItems::Center)
                .background(
                    accent_color.primary_fill_color(PrimaryFillColorVariant::DefaultGrayscale),
                )
                .border(1.0)
                .border_radius(5.0)
                .class(SpinboxButton, move |s| {
                    s.active(|s| s.background(Color::rgb(0.1, 0.1, 0.1)))
                        .align_items(AlignItems::Center)
                        .background(Color::rgb(0.15, 0.15, 0.15))
                        .border_radius(5.0)
                        .color(Color::rgb(0.8, 0.8, 0.8))
                        .disabled(|s| s.background(Color::TRANSPARENT))
                        .focus_visible(|s| {
                            s.outline(2.0)
                                .outline_color(Color::WHITE.with_alpha_factor(0.5))
                        })
                        .font_size(14.0)
                        .padding_horiz(5.0)
                        .padding_vert(5.0)
                        .margin_horiz(5.0)
                })
                .disabled(|s| {
                    let accent_color = self.accent_color.get();
                    s.background(accent_color.primary_fill_color(PrimaryFillColorVariant::Disabled))
                })
                .width_full()
            }),
        )
    }
}
