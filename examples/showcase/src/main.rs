#![windows_subsystem = "windows"]

use floem::kurbo::Size;
use floem::reactive::create_rw_signal;
use floem::reactive::create_signal;
use floem::style::AlignItems;
use floem::view::View;
use floem::views::h_stack;
use floem::views::v_stack;
use floem::views::Decorators;
use floem::window::WindowConfig;
use floem::EventPropagation;
use floem_ui_kit::button::ButtonVariant;
use floem_ui_kit::label::LabelVariant;
use floem_ui_kit::radio::RadioGroupVariant;
use floem_ui_kit::theme::Theme;

fn app_view() -> impl View {
    let theme = Theme::default();

    let (inputs_enabled, set_inputs_enabled) = create_signal(true);

    let (boolean_signal, set_boolean_signal) = create_signal(true);
    let rw_counter = create_rw_signal(0);
    let (counter, set_counter) = (rw_counter.read_only(), rw_counter.write_only());
    let text_value = create_rw_signal(String::from("This is a text"));

    theme.root_view(
        v_stack((
            theme.simple_header("Header"),
            theme.padded_container(
                v_stack((
                    theme
                        .labeled_checkbox(move || inputs_enabled.get(), || "Enable all inputs")
                        .on_click_stop(move |_| {
                            set_inputs_enabled.set(!inputs_enabled.get());
                        }),
                    h_stack((
                        v_stack((
                            theme.label(move || "Enable all inputs", LabelVariant::Dimmed),
                            theme.label(move || "Counter", LabelVariant::Dimmed),
                            theme.label(move || "Accent color", LabelVariant::Dimmed),
                            theme.label(move || "Text input", LabelVariant::Dimmed),
                        ))
                        .style(|s| s.gap(0.0, 5.0)),
                        v_stack((
                            theme.label(
                                move || match inputs_enabled.get() {
                                    true => "Yes",
                                    false => "No",
                                },
                                LabelVariant::Regular,
                            ),
                            theme.label(move || counter.get(), LabelVariant::Regular),
                            theme.label(move || theme.accent_color.get(), LabelVariant::Regular),
                            theme.label(move || text_value.get(), LabelVariant::Regular),
                        ))
                        .style(|s| s.gap(0.0, 5.0)),
                    ))
                    .style(|s| s.gap(20.0, 0.0)),
                    h_stack((
                        theme
                            .button(|| "Increment", ButtonVariant::Emphasized)
                            .on_click(move |_| {
                                set_counter.update(|value| *value += 1);
                                EventPropagation::Stop
                            })
                            .disabled(move || !inputs_enabled.get()),
                        theme
                            .button(|| "Decrement", ButtonVariant::Regular)
                            .on_click(move |_| {
                                set_counter.update(|value| *value -= 1);
                                EventPropagation::Stop
                            })
                            .disabled(move || !inputs_enabled.get()),
                    ))
                    .style(|s| s.gap(10.0, 0.0)),
                    theme
                        .integer_input(rw_counter, 1, Some(-2), Some(9000))
                        .disabled(move || !inputs_enabled.get()),
                    theme
                        .labeled_checkbox(move || boolean_signal.get(), || "Ordinary checkbox")
                        .on_click_stop(move |_| {
                            set_boolean_signal.set(!boolean_signal.get());
                        })
                        .disabled(move || !inputs_enabled.get()),
                    theme
                        .radio_group(
                            theme.accent_color.read_only(),
                            theme.accent_color.write_only(),
                            10.0,
                            RadioGroupVariant::Horizontal,
                        )
                        .disabled(move || !inputs_enabled.get()),
                    theme
                        .text_input(text_value)
                        .disabled(move || !inputs_enabled.get()),
                ))
                .style(|s| s.align_items(AlignItems::Start).gap(0.0, 20.0)),
            ),
        ))
        .style(|s| s.width_full()),
    )
}

fn main() {
    let window_config = WindowConfig::default()
        .size(Size {
            width: 400.0,
            height: 550.0,
        })
        .title("Floem UI Kit Showcase");

    floem::Application::new()
        .window(move |_| app_view(), Some(window_config))
        .run()
}
