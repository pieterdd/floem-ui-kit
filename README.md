# Floem UI Kit

[![crates.io](https://img.shields.io/crates/v/floem-ui-kit.svg)](https://crates.io/crates/floem-ui-kit)
[![docs.rs](https://docs.rs/floem-ui-kit/badge.svg)](https://docs.rs/floem-ui-kit)

Want a pretty Rust GUI with minimal time investment? Use a kit of premade UI components! Floem UI Kit provides themed widgets you can use in conjunction with the [Floem](https://github.com/lapce/floem) GUI library.

- ✅ Supports all major desktop operating systems
- ✅ All widgets implement hover, focus and disabled state
- ✅ Supports multiple accent colors

⚠️ **Floem UI Kit, like Floem, is experimental software. Breaking API changes may regularly happen.** ⚠️

![Showcase](docs/img/showcase.png)

## First time using Floem?

Its [README](https://github.com/lapce/floem) and [documentation](http://lapce.dev/floem/floem/) provide a quick start example, an introduction and some background on Floem's architecture. You might notice that it incorporates familiar concepts from other UI frameworks, including:

- Interface composition using nestable horizontal and vertical widget containers
- A styling system that brings many CSS-like features to Rust, without the overhead of a browser runtime
- Reactivity using signals, as seen in frameworks like [SolidJS](https://www.solidjs.com/)
- Implementation of UIs using functions, similar to Jetpack Compose

## Installation

You'll need to install a compatible Floem version along with Floem UI Kit. To see which Floem version we're currently based on, go to [Cargo.toml](Cargo.toml).

## Getting started with Floem UI Kit

The Floem UI Kit workflow involves:

1. Creating an instance of [`floem_ui_kit::theme::Theme`](https://docs.rs/floem-ui-kit/latest/floem_ui_kit/theme/struct.Theme.html). `Theme::default()` creates an instance with default settings.
2. Using the widget creation methods in `Theme` to build your UI. If Floem UI Kit doesn't have exactly what you want, you can mix and match with self-written UI components. You can keep using Floem's `v_stack` and `h_stack` methods to lay out your components.
3. Wrapping your window contents in Floem UI Kit's padded container to ensure your widgets don't stick to the side of the window.
4. Wrapping the padded container in Floem UI Kit's root view to apply the theme to the entire window.

[Over here](examples/counter/src/main.rs) you'll find Floem's quickstart example ported to Floem UI Kit. Here is a reference render:

![A counter in Floem UI Kit](docs/img/counter_uikit.png)

These components are still sticking to one another. You'll probably want to add a small gap between them. `v_stack` and `h_stack` can automatically add those gaps, provided that you request it:

```rust
v_stack(/* .. */)
	.style(|s| s.gap(0.0, 10.0))
```

The above will add no horizontal gap and a vertical 10-pixel gap.

## Supported widgets

For more information on how to use these, see the [code docs](https://docs.rs/floem-ui-kit/latest/floem_ui_kit/theme/struct.Theme.html#implementations).

| **Widget**                        | **Preview**                                  |
| --------------------------------- | -------------------------------------------- |
| Button _(multiple variants)_      | ![Button](docs/img/button.png)               |
| Checkbox                          | ![Checkbox](docs/img/checkbox.png)           |
| Integer input (spinbox)           | ![Integer input](docs/img/integer_input.png) |
| Label _(multiple variants)_       | ![Label](docs/img/label.png)                 |
| Radio group _(multiple variants)_ | ![Radio group](docs/img/radio_group.png)     |
| Simple header                     | ![Simple header](docs/img/simple_header.png) |
| Text input                        | ![Text input](docs/img/text_input.png)       |

For an example incorporating all available widgets, [see here](examples/showcase/src/main.rs). It's the source code for the screenshot at the top of the README.

## Not quite what you're looking for?

You can browse the [floem-themes](https://github.com/topics/floem-themes) topic on GitHub to look for other Floem UI libraries and themes.

As far as Rust UI libraries go, [Iced](https://iced.rs/) is a well-known one. Device manufacturer System76 is using it to implement its own desktop environment. You might be able to build onto their work by checking out [libcosmic](https://github.com/pop-os/libcosmic). Be advised that its learning curve may be a bit steeper than Floem's, and that the library is primarily intended for use by applications that are native to the Cosmic desktop environment.

If you're willing to consider Electron-like solutions that are not completely native and may have a larger resource footprint, I'd definitely check out [Tauri](https://tauri.app/). Since it's built on HTML/JS/CSS, you can use it with any web-based UI framework.

And finally, [Are we GUI yet?](https://areweguiyet.com/) is a big collection of GUI-related tools.
