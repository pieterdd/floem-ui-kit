use floem::reactive::create_signal;
use floem::view::View;
use floem::views::{h_stack, label, text, v_stack, Decorators};
use floem::EventPropagation;

/// Renders your UI, and updates it whenever a signal fires
fn app_view() -> impl View {
    // Use counter.get() to access the counter value.
    // Change it with set_counter.update(..) or set_counter.set().
    let (counter, set_counter) = create_signal(0);

    // v_stack = vertical group layout
    v_stack((
        label(move || format!("Value: {}", counter.get())),
        // h_stack = horizontal group layout
        h_stack((
            text("Increment").on_click(move |_| {
                set_counter.update(|value| *value += 1);
                EventPropagation::Stop
            }),
            text("Decrement").on_click(move |_| {
                set_counter.update(|value| *value -= 1);
                EventPropagation::Stop
            }),
        )),
    ))
}

fn main() {
    floem::launch(app_view);
}
