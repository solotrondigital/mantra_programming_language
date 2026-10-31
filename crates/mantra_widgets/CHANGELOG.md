# Changelog

All notable changes to `mantra_widgets` will be documented in this file.

## [0.5.0] - 2026-10-30

### Added
- Initial release of `mantra_widgets` as the official UI component library for Mantra.
- Core UI Engine: `Widget`, `BuildContext`, `BoxConstraints`, `Size`, `EdgeInsets`, `Alignment`, `Observable<T>`, `Event`.
- Design System: `MantraTheme` (Light, Dark, Cupertino, Material), `MantraPalette`, `MantraTypography`, `Elevation`, `BorderRadius`.
- Full Component Catalog:
  - Buttons (19 variants with loading, shape, and size options).
  - Cards (Standard, Statistics, Product, Pricing).
  - Avatars & Badges (Circle, Rounded, Online status, Pill badges, Chips).
  - Navigation (AppBar, BottomNavigationBar).
  - Form Controls (TextField, PasswordField, Switch, Checkbox).
  - Layouts (Container, Row, Column, GridView, ListTile, Divider).
  - Feedback & Loaders (AlertDialog, Toast, CircularLoader, ProgressBar, Shimmer, StarRating, LikeButton).
  - Typography (Text, Heading, GradientText).
- Responsive UI Engine: `ScreenSize`, `ScreenCategory`, `ResponsiveValue`, `MediaQuery`.
- Cross-Platform Platform Bridge: Event bindings for Web, Android, and iOS.
- Native Mantra package implementation under `lib/`.
- Component showcase applications (`examples/gallery/`, `examples/ecommerce/`, `examples/dashboard/`, `examples/forms/`).
- Comprehensive automated test suite with 100% test coverage for core systems.

