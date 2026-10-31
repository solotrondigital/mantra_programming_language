# Responsive UI Engine

`mantra_widgets` adapts fluidly across mobile, tablet, and desktop viewports.

## Breakpoints

| Category | Viewport Width | Typical Target |
| :--- | :--- | :--- |
| **Mobile** | `< 768px` | iPhone, Android phones |
| **Tablet** | `768px – 1023px` | iPad, Android tablets |
| **Desktop** | `1024px – 1440px` | Laptops, desktop monitors |
| **WideScreen** | `> 1440px` | 4K / Ultra-wide monitors |

## Responsive Adaptations

In Rust:
```rust
let columns = ResponsiveValue::new(1)
    .with_tablet(2)
    .with_desktop(4);
```

In Mantra:
```mantra
let screen = ScreenSize(window_width, window_height)
if screen.is_mobile():
    # Render single-column mobile view
else:
    # Render multi-column dashboard
```

