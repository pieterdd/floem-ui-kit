use std::fmt::Display;

use floem::{
    event::EventListener,
    peniko::Color,
    reactive::{create_signal, ReadSignal, WriteSignal},
    style::AlignItems,
    style_class,
    view::View,
    views::{container, h_stack, label, svg, virtual_list, Decorators, VirtualListDirection},
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
            .base_style(|s| s.align_items(AlignItems::Center))
            .keyboard_navigatable()
            .style(move |s| {
                s.border_radius(5.0)
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
        let last_variant = *(variants
            .last()
            .expect("Cannot pass empty enum to radio group"));
        let (variants_signal, _set_variant) = create_signal(variants);

        container(
            virtual_list(
                match variant {
                    RadioGroupVariant::Horizontal => VirtualListDirection::Horizontal,
                    RadioGroupVariant::Vertical => VirtualListDirection::Vertical,
                },
                floem::views::VirtualListItemSize::Fixed(Box::new(|| 20.0)),
                move || variants_signal.get(),
                move |item| *item,
                move |item| {
                    container(self.radio_button(item, read_signal, write_signal)).style(move |s| {
                        s.apply_if(
                            item != last_variant && variant == RadioGroupVariant::Vertical,
                            |s| s.margin_bottom(gap_between_items),
                        )
                        .apply_if(
                            item != last_variant && variant == RadioGroupVariant::Horizontal,
                            |s| s.margin_right(gap_between_items),
                        )
                    })
                },
            )
            .style(move |s| {
                s.apply_if(variant == RadioGroupVariant::Vertical, |s| s.flex_col())
                    .apply_if(variant == RadioGroupVariant::Horizontal, |s| s.flex_row())
            }),
        )
    }
}
