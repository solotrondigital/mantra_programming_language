# Theming and Design Systems in Mantra Widgets

`mantra_widgets` includes a centralized token-based theme engine.

## Built-In Themes

1. **Light Theme**:
   ```mantra
   let theme = light_theme()
   ```
2. **Dark Theme**:
   ```mantra
   let theme = dark_theme()
   ```

## Design System Tokens

Themes define the following CSS custom properties (variables):
* `--mantra-primary`: Primary brand accent color.
* `--mantra-secondary`: Muted auxiliary tone.
* `--mantra-success`: Positive state indicator (#10b981).
* `--mantra-warning`: Caution indicator (#f59e0b).
* `--mantra-danger`: Error / destructive indicator (#ef4444).
* `--mantra-bg`: Page background.
* `--mantra-surface`: Card / container surface background.
* `--mantra-text`: Primary text color.
* `--mantra-border`: Structural border color.
* `--mantra-radius`: Default corner curvature (default 8px).

## Dynamic Switching

Pass your desired theme to `MantraApp`:

```mantra
let app = MantraApp("Themed App", dark_theme(), ui_content)
```

