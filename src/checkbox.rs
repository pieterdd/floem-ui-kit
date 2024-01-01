use std::fmt::Display;

use floem::{
    event::EventListener,
    peniko::Color,
    reactive::{create_signal, ReadSignal, WriteSignal},
    style::AlignItems,
    view::View,
    views::{container, h_stack, label, svg, Decorators},
};

use crate::{
    accents::{BorderColorVariant, PrimaryFillColorVariant},
    theme::Theme,
};

impl Theme {
    fn checkbox_symbol(self, read_signal: ReadSignal<bool>) -> impl View {
        const CHECKED_SVG: &str = r#"
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16">
				<g transform="matrix(0.925671,0,0,0.925671,2.36266,1.94611)">
					<path d="M5.19,11.83L0.18,7.44L1.82,5.56L4.81,8.17L10,1.25L12,2.75L5.19,11.83Z" style="fill:white;fill-rule:nonzero;"/>
				</g>
			</svg>
		"#;
        let svg_str = move || if read_signal.get() { CHECKED_SVG } else { "" }.to_string();
        svg(svg_str)
    }

    /// Instantiates a checkbox widget controlling a boolean with a text label next to it.
    pub fn labeled_checkbox<S: Display + 'static>(
        self,
        read_signal: ReadSignal<bool>,
        write_signal: WriteSignal<bool>,
        label_render_func: impl Fn() -> S + 'static,
    ) -> impl View {
        let (is_hovering, set_is_hovering) = create_signal(false);
        let (is_focused, set_is_focused) = create_signal(false);

        container(
            h_stack((
                self.checkbox_symbol(read_signal).style(move |s| {
                    let accent_color = self.accent_color.get();

                    let is_selected = read_signal.get();
                    let unhovered_bg_color = match is_selected {
                        true => {
                            accent_color.primary_fill_color(PrimaryFillColorVariant::DefaultColored)
                        }
                        false => accent_color
                            .primary_fill_color(PrimaryFillColorVariant::DefaultGrayscale),
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

                    s.background(unhovered_bg_color)
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
                label(label_render_func).style(move |s| {
                    s.disabled(|s| s.color(self.accent_color.get().disabled_text_color()))
                }),
            ))
            .keyboard_navigatable()
            .style(|s| {
                s.align_items(AlignItems::Center)
                    .border_radius(5.0)
                    .focus_visible(|s| {
                        s.outline(2.0)
                            .outline_color(Color::WHITE.with_alpha_factor(0.5))
                    })
                    .gap(10.0, 0.0)
            })
            .on_click(move |_| {
                write_signal.set(!read_signal.get());
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
}
