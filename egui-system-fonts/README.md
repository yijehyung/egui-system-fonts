# egui-system-fonts

System font loader for `egui`. Auto-detects locale and applies a matching font fallback chain.

## Installation

```toml
[dependencies]
egui-system-fonts = "0.35.0"
```

## Usage

```rust
use egui_system_fonts::{set_auto, FontStyle};

impl MyApp {
    fn new(cc: &eframe::CreationContext) -> Self {
        set_auto(&cc.egui_ctx, FontStyle::Sans);
        Self { /* ... */ }
    }
}
```

Use `add_auto` instead of `set_auto` to keep existing fonts and append system fonts as fallback.

See [docs.rs](https://docs.rs/egui-system-fonts) for region and preset options.

## WASM

On WASM, fonts are downloaded asynchronously from [noto-cjk](https://github.com/notofonts/noto-cjk) via jsDelivr CDN.

## License

MIT OR Apache-2.0
