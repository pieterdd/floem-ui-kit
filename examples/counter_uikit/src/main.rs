use floem::reactive::create_signal;
use floem::view::View;
use floem::views::{h_stack, v_stack, Decorators};
use floem::EventPropagation;
use floem_ui_kit::button::ButtonVariant;
use floem_ui_kit::label::LabelVariant;
use floem_ui_kit::theme::Theme;

fn app_view() -> impl View {
    let theme = Theme::default();

    let (counter, set_counter) = create_signal(0);

    theme.root_view(
        theme.padded_container(v_stack((
            theme.label(
                move || format!("Value: {}", counter.get()),
                LabelVariant::Regular,
            ),
            h_stack((
                theme
                    .button(|| "Increment", ButtonVariant::Emphasized)
                    .on_click(move |_| {
                        set_counter.update(|value| *value += 1);
                        EventPropagation::Stop
                    }),
                theme
                    .button(|| "Decrement", ButtonVariant::Emphasized)
                    .on_click(move |_| {
                        set_counter.update(|value| *value -= 1);
                        EventPropagation::Stop
                    }),
            )),
        ))),
    )
}

fn main() {
    floem::launch(app_view);
}
