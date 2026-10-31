# Accessibility in Mantra Widgets

Accessibility is baked into the core engine of `mantra_widgets` rather than bolted on.

## Core Accessibility Features

1. **Semantic HTML5 & ARIA**:
   * Every component emits appropriate W3C ARIA roles (`role="button"`, `role="progressbar"`, `role="checkbox"`).
   * Interactive states emit `aria-disabled`, `aria-checked`, and `aria-expanded` attributes.
2. **Accessible Color Contrast**:
   * Default light and dark themes strictly conform to WCAG 2.1 AA contrast ratios (4.5:1 for body copy, 3:1 for large headers).
3. **Keyboard & Screen Reader Support**:
   * All clickable widgets are focusable with distinct visual focus rings and support `Enter` and `Space` activation.
4. **Touch Targets**:
   * Touch targets meet mobile accessibility minimums (minimum $44 \times 44$ CSS pixels on touch viewports).

