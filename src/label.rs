use std::fmt::Display;

use floem::peniko::Color;
use floem::views::{container, Decorators};
use floem::{view::View, views::label};

use crate::theme::Theme;

#[derive(Clone, Copy)]
pub enum LabelVariant {
    Regular,
    Dimmed,
}

impl Theme {
    /// Instantiates a plain text label that sources its contents from a
    /// dynamic rendering function.
    pub fn label<S: Display + 'static>(
        self,
        render_func: impl Fn() -> S + 'static,
        variant: LabelVariant,
    ) -> impl View {
        container(label(render_func).style(move |s| {
            s.color(match variant {
                LabelVariant::Regular => Color::WHITE,
                LabelVariant::Dimmed => Color::rgb(0.55, 0.55, 0.55),
            })
        }))
    }
}
