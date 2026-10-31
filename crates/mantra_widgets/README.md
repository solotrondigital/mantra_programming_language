# mantra_widgets

Official comprehensive UI component library, design system, and responsive rendering engine for the **Mantra Programming Language**.

[![Crate Version](https://img.shields.io/badge/version-0.5.0-blue.svg)](https://mantra-lang.dev)
[![License](https://img.shields.io/badge/license-MIT%20OR%20GPL--3.0-green.svg)](LICENSE)
[![Platforms](https://img.shields.io/badge/platforms-iOS%20%7C%20Android%20%7C%20Web%20%7C%20Desktop-orange.svg)](https://mantra-lang.dev)

---

## Features

* **Complete Component Catalog**:
  * **Buttons**: `MantraButton`, `ElevatedButton`, `FilledButton`, `OutlinedButton`, `IconButton`, `FloatingActionButton`, `LoadingButton`, `ButtonGroup`
  * **Cards**: `MantraCard`, `StatisticsCard`, `ProductCard`, `PricingCard`
  * **Avatars & Badges**: `Avatar`, `CircleAvatar`, `RoundedAvatar`, `Badge`, `Chip`
  * **Images & Media**: `MantraImage`, `CircularImage`, `Carousel`
  * **Navigation**: `AppBar`, `BottomNavigationBar`, `BottomNavItem`, `Drawer`
  * **Forms**: `TextField`, `PasswordField`, `Switch`, `Checkbox`
  * **Layouts**: `Container`, `Row`, `Column`, `GridView`, `ListTile`, `Divider`
  * **Feedback & Loaders**: `AlertDialog`, `Toast`, `CircularLoader`, `ProgressBar`, `Shimmer`, `StarRating`, `LikeButton`
  * **Typography**: `Text`, `Heading`, `GradientText`
  * **Advanced & Animations**: `EmptyState`, `FadeAnimation`, `ExpansionTile`
* **Token-Based Design System**: Light theme, Dark theme, Cupertino (iOS), and Material Design with CSS variable injection.
* **Responsive Engine**: Dynamic breakpoint resolution for Mobile, Tablet, Desktop, and WideScreen displays.
* **Unified Bridge Architecture**: First-class support for Web browsers (WASM/HTML5), Android WebViews (`MantraBridge.kt`), and iOS WebKit (`MantraBridge.swift`).
* **Zero External Dependencies**: Pure, memory-safe Rust engine delivering ultra-fast layout rendering and minimal binary footprints.

---

## Installation

Add `mantra_widgets` to your application's `mantra.toml`:

```bash
mantra add mantra_widgets 0.5.0
```

Or declare it in your `mantra.toml` dependencies:

```toml
[dependencies]
mantra_widgets = "0.5.0"
```

---

## Quick Start (.mantra)

```mantra
import mantra_widgets.*

class HomeScreen:
    def __init__(self):
        self.title = "Welcome to Mantra UI"

    pub def build(self) -> String:
        let btn = MantraButton("Click Me", "primary")
        let card = MantraCard(btn.render())
        return card.render()

def main() -> Int:
    let app = MantraApp("My Mantra App", light_theme(), HomeScreen().build())
    return app.start()
```

---

## Examples

Run the interactive component gallery:

```bash
cd crates/mantra_widgets/examples/gallery
mantra run
```

---

## License

Licensed under either of:
* MIT License ([LICENSE](LICENSE))
* GNU General Public License v3.0 or later

