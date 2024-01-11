use std::fmt::Display;

use floem::{
    event::EventListener,
    peniko::Color,
    reactive::{create_signal, ReadSignal, WriteSignal},
    style::AlignItems,
    style_class,
    view::View,
    views::{container, h_stack, h_stack_from_iter, label, svg, v_stack_from_iter, Decorators},
};
use strum::IntoEnumIterator;

use crate::{
    accents::{BorderColorVariant, PrimaryFillColorVariant},
    theme::Theme,
};

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum RadioGroupVariant {
    Horizontal,
    Vertical,
}

style_class!(pub RadioButtonClass);
style_class!(pub RadioButtonLabelClass);

impl Theme {
    fn radio_button_symbol<T>(self, variant_to_render: T, read_signal: ReadSignal<T>) -> impl View
    where
        T: IntoEnumIterator + 'static + Clone + PartialEq,
    {
        const CHECKED_SVG: &str = r#"
			<svg viewBox="0 0 10 10" xmlns="http://www.w3.org/2000/svg">
				<circle cx="5" cy="5" r="2" fill="white" />
			</svg>
		"#;
        let svg_str = move || {
            if read_signal.get() == variant_to_render {
                CHECKED_SVG
            } else {
                ""
            }
            .to_string()
        };
        svg(svg_str)
    }

    /// Instantiates one option in a radio group widget. For most use cases you'll want [`Self::radio_group`] instead.
    /// Use [`Self::radio_button`] to implement special accommodations such as non-standard layouts or to render
    /// certain options as disabled.
    pub fn radio_button<T>(
        self,
        variant_to_render: T,
        read_signal: ReadSignal<T>,
        write_signal: WriteSignal<T>,
    ) -> impl View
    where
        T: IntoEnumIterator + 'static + Copy + Clone + PartialEq + Display,
    {
        let (is_hovering, set_is_hovering) = create_signal(false);
        let (is_focused, set_is_focused) = create_signal(false);

        container(
            h_stack((
                self.radio_button_symbol(variant_to_render, read_signal)
                    .style(move |s| {
                        let accent_color = self.accent_color.get();

                        let is_selected = read_signal.get() == variant_to_render;
                        let unhovered_bg_color = match is_selected {
                            true => accent_color
                                .primary_fill_color(PrimaryFillColorVariant::DefaultColored),
                            false => accent_color
                                .primary_fill_color(PrimaryFillColorVariant::DefaultGrayscale),
                        };
                        let unhovered_border_color = match is_selected {
                            true => accent_color.border_color(BorderColorVariant::DefaultColored),
                            false => {
                                accent_color.border_color(BorderColorVariant::DefaultGrayscale)
                            }
                        };
                        let hovered_bg_color = match is_selected {
                            true => {
                                accent_color.primary_fill_color(PrimaryFillColorVariant::Hovered)
                            }
                            false => Color::BLACK.with_alpha_factor(0.1),
                        };
                        let hovered_border_color = match is_selected {
                            true => accent_color.border_color(BorderColorVariant::HoveredColored),
                            false => {
                                accent_color.border_color(BorderColorVariant::HoveredGrayscale)
                            }
                        };

                        s.background(unhovered_bg_color)
                            .border_radius(100.0)
                            .border(1.0)
                            .border_color(unhovered_border_color)
                            .disabled(|s| {
                                s.background(
                                    accent_color
                                        .primary_fill_color(PrimaryFillColorVariant::Disabled),
                                )
                                .border_color(
                                    accent_color.border_color(BorderColorVariant::Disabled),
                                )
                            })
                            .padding(12.0)
                            .apply_if(is_hovering.get(), |s| {
                                s.background(hovered_bg_color)
                                    .border_color(hovered_border_color)
                            })
                            .apply_if(is_focused.get(), |s| match is_selected {
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
                    }),
                label(move || variant_to_render).style(move |s| {
                    s.disabled(|s| s.color(self.accent_color.get().disabled_text_color()))
                }),
            ))
            .keyboard_navigatable()
            .style(move |s| {
                s.align_items(AlignItems::Center)
                    .border_radius(5.0)
                    .focus_visible(|s| {
                        s.outline(2.0)
                            .outline_color(Color::WHITE.with_alpha_factor(0.5))
                    })
                    .gap(10.0, 0.0)
            })
            .on_click(move |_| {
                write_signal.set(variant_to_render);
                floem::EventPropagation::Stop
            })
            .on_event_stop(EventListener::PointerEnter, move |_| {
                set_is_hovering.set(true);
            })
            .on_event_stop(EventListener::PointerLeave, move |_| {
                set_is_hovering.set(false);
            })
            .on_event_stop(EventListener::FocusGained, move |_| {
                set_is_focused.set(true);
            })
            .on_event_stop(EventListener::FocusLost, move |_| {
                set_is_focused.set(false);
            }),
        )
    }

    /// Instantiates a radio group widget for given enum T. For the display of labels, we rely on
    /// the `Display` trait. We ask that your enum derives `EnumIter` so the radio group may list
    /// all available options. Options can be laid out horizontally or vertically. A configurable
    /// gap is inserted between items.
    pub fn radio_group<T>(
        self,
        read_signal: ReadSignal<T>,
        write_signal: WriteSignal<T>,
        gap_between_items: f32,
        variant: RadioGroupVariant,
    ) -> impl View
    where
        T: IntoEnumIterator + Copy + Clone + PartialEq + Eq + Display + std::hash::Hash + 'static,
    {
        let variants: im::Vector<T> = T::iter().collect();
        let (variants_signal, _set_variant) = create_signal(variants);

        // These are not reactive. This is a consequence of how items are passed to `h_stack_from_iter`. Since
        // the enum values are known at compile time, this is not an issue.
        let group_items = variants_signal
            .get()
            .into_iter()
            .map(|item| container(self.radio_button(item, read_signal, write_signal)));

        container(match variant {
            RadioGroupVariant::Horizontal => {
                h_stack_from_iter(group_items).style(move |s| s.gap(gap_between_items, 0.))
            }
            RadioGroupVariant::Vertical => {
                v_stack_from_iter(group_items).style(move |s| s.gap(0., gap_between_items))
            }
        })
    }
}
