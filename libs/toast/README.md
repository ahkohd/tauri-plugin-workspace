Show a toast.

### Install
_This lib requires a Rust version of at least **1.64**_

There are three general methods of installation that we can recommend.

1. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)
2. Pull sources directly from Github using git tags / revision hashes (most secure)
3. Git submodule install this repo in your tauri project and then use file protocol to ingest the source (most secure, but inconvenient to use)

Install the Core plugin by adding the following to your `Cargo.toml` file:

`src-tauri/Cargo.toml`
```toml
[dependencies]
toast = { git = "https://github.com/ahkohd/tauri-toolkit", branch = "v1" }
```

### Demo
![A demo of toast](https://github.com/ahkohd/tauri-toolkit/blob/v2/assets/toast.gif)

## Usage
```rust
use toast::{Toast, ToastExt};

fn main() {
    // setup the toast with default configuration
    app_handle.manage(Toast::default());

    // show a toast
    // `message` can be formatted in markdown 
    app_handle.toast("Hello, **World!** 🎉");
}
```

## Functions

- `app_handle.toast(message: &str)`:
  Shows a toast
  - `message: &str`: The toast message. It supports markdown.

## ToastConfig
Configure the toast
- `font_size: f64`: The font size of the toast text label
- `padding: (f64, f64)`: The padding of the toast 
- `position: ToastPosition`: Where to place the toast
- `offset: f64`: Offset the toast position
- `duration: f64`: The duration of the toast
- `fade_duration: f64`: The duration of the enter and exit fade animation

## ToastPosition
- `Top`: The toast will be position at the top center of the monitor with cursor
- `Bottom`: The toast will be position at the bottom center of the monitor with cursor

To configure your toast, use `Toast::new(...)` instead of `Toast::default()`. For example:
```rust
use toast::{Toast, ToastConfig, ToastPosition};

app_handle.manage(Toast::new(ToastConfig {
    position: ToastPosition::Top,
    offset: 20.0,
    ..Default::default()
}));
```

## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.

## License
MIT or MIT/Apache 2.0 where applicable
