Master prompt — Build mantra_widgets
Copy the following prompt into your AI coding agent (Claude Code, Codex, Cursor, or another development agent).
MASTER DEVELOPMENT PROMPT: BUILD MANTRA_WIDGETS
1. Project Overview
You are a principal programming language engineer, UI framework architect, compiler engineer, and senior cross-platform application developer.

Your task is to design and implement mantra_widgets, the official, comprehensive UI component library for the Mantra Programming Language.

The library should provide a developer experience and breadth of reusable UI components comparable to Flutter's GetWidget package:

https://pub.dev/packages/getwidget

However, Mantra Widgets must be an independent implementation built specifically for the Mantra language ecosystem.
Main objective
Develop a complete, reusable, customizable, responsive, production-ready UI component library that allows developers to build beautiful applications without implementing common UI components from scratch.

The library must support Mantra's frontend application architecture and target:
- iOS
- Android
- Web
It should also be architected to support future desktop platforms.
2. First: Understand the Mantra Ecosystem
Before writing code, inspect the existing Mantra compiler, parser, runtime, standard library, project generator, frontend architecture, and package management system.

Understand:
1. Mantra syntax and grammar.
2. .mantra source files.
3. Module and package imports.
4. Class and object-oriented programming.
5. Interfaces, inheritance, and polymorphism.
6. Functions, callbacks, and event handling.
7. Static type checking.
8. Runtime and memory management.
9. Native compilation architecture.
10. Frontend rendering and platform integration.
11. Existing project structure and conventions.
Do not invent Mantra syntax.

All package APIs and example applications must use syntax that is genuinely supported by the current Mantra compiler.

If a required language capability is missing, identify it and propose the smallest necessary compiler/runtime enhancement.

Do not break existing Mantra programs or introduce unnecessary language changes.
3. Architecture
Build mantra_widgets using a layered architecture.
Layer 1: Core UI Engine
Implement:
- Widget base abstractions.
- Widget lifecycle.
- Widget composition.
- Component properties.
- Component state.
- Reactive state management.
- Event handling.
- Rendering abstraction.
- Layout constraints.
- Responsive measurements.
- Component tree.
- Theme inheritance.
- Accessibility semantics.
- Error handling.
Layer 2: Design System
Create a centralized design system.

Implement:
- Colors.
- Typography.
- Font weights.
- Spacing.
- Padding.
- Margins.
- Borders.
- Border radius.
- Shadows.
- Elevation.
- Gradients.
- Iconography.
- Component sizes.
- Shape variants.
- Animation durations.
- Responsive breakpoints.
Support:
- Light theme.
- Dark theme.
- Custom themes.
- Theme inheritance.
- Global theme configuration.
- Per-component theme overrides.
- Material-inspired design.
- iOS-inspired design.
- Custom design systems.
Layer 3: Widget Components
Implement the complete component catalog described below.
4. Component Catalog
A. Buttons
Implement:
- MantraButton
- ElevatedButton
- FilledButton
- OutlinedButton
- TextButton
- RoundedButton
- SquareButton
- IconButton
- SocialButton
- FloatingActionButton
- ButtonGroup
- ButtonBar
- LoadingButton
- GradientButton
- ToggleButton
- SegmentedButton
- CloseButton
- BackButton
- ChipButton
Support:
- Different sizes.
- Different shapes.
- Icons.
- Loading states.
- Disabled states.
- Click callbacks.
- Hover states.
- Press animations.
- Custom colors.
- Custom borders.
B. Cards
Implement:
- MantraCard
- BasicCard
- ProfileCard
- ProductCard
- ImageCard
- HorizontalCard
- VerticalCard
- ExpandableCard
- InteractiveCard
- GradientCard
- ElevatedCard
- OutlinedCard
- StatisticsCard
- InformationCard
- PricingCard
- NotificationCard
C. Avatar and Profile
Implement:
- Avatar
- CircleAvatar
- SquareAvatar
- RoundedAvatar
- AvatarGroup
- AvatarBadge
- ProfileAvatar
- ProfileCard
- UserProfile
- OnlineStatusIndicator
Support image, initials, icons, fallback images, status indicators, and custom sizes.
D. Badges and Labels
Implement:
- Badge
- IconBadge
- ButtonBadge
- NotificationBadge
- StatusBadge
- DotBadge
- CounterBadge
- Label
- Tag
- Chip
- ChoiceChip
- FilterChip
- ActionChip
E. Images
Implement:
- MantraImage
- NetworkImage
- AssetImage
- CircularImage
- RoundedImage
- ImageOverlay
- ImageGallery
- ImageCarousel
- ImagePlaceholder
- ImageWithCaption
- ImageWithBadge
Include loading indicators, error fallbacks, caching, image fitting, and responsive sizing.
F. Carousel and Sliders
Implement:
- Carousel
- ImageCarousel
- ProductCarousel
- CardCarousel
- BannerCarousel
- HorizontalCarousel
- VerticalCarousel
- InfiniteCarousel
- PageSlider
- ImageSlider
- RangeSlider
- CustomSlider
Support autoplay, indicators, infinite scrolling, swipe gestures, animation, and custom navigation controls.
G. Navigation
Implement:
- AppBar
- BottomNavigationBar
- NavigationRail
- NavigationDrawer
- Drawer
- DrawerHeader
- TabBar
- TabBarView
- SegmentTabs
- Breadcrumb
- NavigationMenu
- SideMenu
- FloatingNavigationBar
- TopNavigationBar
Support route navigation, selected states, icons, badges, and responsive behavior.
H. Forms and Input Controls
Implement:
- TextField
- TextArea
- PasswordField
- EmailField
- SearchField
- NumberField
- PhoneField
- DatePicker
- TimePicker
- DateRangePicker
- Dropdown
- MultiSelectDropdown
- Autocomplete
- Checkbox
- CheckboxListTile
- Radio
- RadioListTile
- Switch
- Toggle
- Slider
- RangeSlider
- Form
- FormField
- FormValidator
- FormController
Include:
- Required validation.
- Custom validation.
- Error messages.
- Helper text.
- Input formatting.
- Focus management.
- Keyboard handling.
- Form submission.
- Accessibility.
I. Lists and Layout Components
Implement:
- ListView
- ListTile
- GridView
- StaggeredGrid
- MasonryGrid
- Wrap
- Row
- Column
- Stack
- Container
- Padding
- Center
- Align
- Expanded
- Flexible
- Spacer
- Divider
- VerticalDivider
- ScrollView
- HorizontalScrollView
- StickyHeader
- ExpandableList
- ReorderableList
- InfiniteList
J. Dialogs and Notifications
Implement:
- AlertDialog
- ConfirmationDialog
- CustomDialog
- BottomSheet
- ModalBottomSheet
- Toast
- Snackbar
- Notification
- Banner
- PopupMenu
- Tooltip
- ContextMenu
- LoadingDialog
Support custom content, animations, actions, dismissal, and callbacks.
K. Progress and Loading
Implement:
- Loader
- CircularLoader
- LinearLoader
- ProgressBar
- CircularProgress
- Shimmer
- SkeletonLoader
- LoadingOverlay
- RefreshIndicator
- PullToRefresh
L. Accordion and Expandable Components
Implement:
- Accordion
- ExpansionTile
- ExpandablePanel
- ExpandableCard
- ExpandableList
- FAQAccordion
- MultiExpansionPanel
M. Typography
Implement:
- Text
- Heading
- DisplayText
- Paragraph
- Caption
- Label
- RichText
- StyledText
- GradientText
- LinkText
- TruncatedText
Support custom fonts, font weights, line heights, alignment, decoration, overflow, and text scaling.
N. Rating and Feedback
Implement:
- Rating
- StarRating
- HeartRating
- EmojiRating
- ReviewCard
- FeedbackForm
- ReactionButton
- LikeButton
- DislikeButton
O. Borders and Decorations
Implement:
- Border
- DashedBorder
- DottedBorder
- GradientBorder
- CircularBorder
- RoundedBorder
- Shadow
- Gradient
- ShapeDecoration
P. Animation
Implement:
- FadeAnimation
- ScaleAnimation
- SlideAnimation
- RotationAnimation
- BounceAnimation
- PulseAnimation
- ShakeAnimation
- ShimmerAnimation
- HeroAnimation
- AnimatedContainer
- AnimatedSwitcher
- AnimatedList
Support duration, delay, easing, curves, repeat, reverse, and lifecycle management.
Q. Advanced Components
Implement:
- IntroScreen
- OnboardingScreen
- EmptyState
- ErrorState
- SuccessState
- Timeline
- Stepper
- Calendar
- ChatBubble
- ChatMessage
- NotificationCenter
- SearchBar
- FloatingWidget
- DashboardWidget
- StatisticsWidget
- PricingTable
- SocialLoginPanel
5. Component API Standards
Every widget must follow consistent naming, property, and event conventions.

For example, use the following conceptual API, adapting syntax to the actual Mantra grammar:
import mantra_widgets

MantraApp(
    theme: MantraTheme.light(),
    home: HomeScreen()
)

MantraButton(
    text: "Get Started",
    type: ButtonType.primary,
    size: ButtonSize.large,
    onPressed: () {
        print("Button clicked")
    }
)

MantraCard(
    elevation: 4,
    radius: 12,
    child: MantraText("Welcome to Mantra")
)

MantraAvatar(
    image: "assets/profile.png",
    size: 64
)
These are conceptual examples. Adapt them to valid Mantra syntax.

Every component must support appropriate:
- Properties.
- Default values.
- Customization.
- Event callbacks.
- State handling.
- Accessibility.
- Theme integration.
- Error handling.
6. Responsive UI Engine
Build a responsive system supporting:
- Mobile layouts.
- Tablet layouts.
- Desktop layouts.
- Web layouts.
- Portrait orientation.
- Landscape orientation.
- Dynamic resizing.
- Flexible constraints.
- Breakpoint-based rendering.
Provide responsive utilities:
- ResponsiveValue.
- ResponsiveBuilder.
- ResponsiveLayout.
- ScreenSize.
- Breakpoints.
- MediaQuery.
- OrientationBuilder.
Components must automatically adapt to screen dimensions where appropriate.
7. Platform Rendering Architecture
Create a rendering abstraction that separates widget definitions from platform-specific rendering.

The architecture should support:
- iOS rendering.
- Android rendering.
- Browser-based web rendering.
Investigate the existing Mantra frontend runtime before selecting implementation technologies.

Do not introduce Flutter or Dart as mandatory dependencies.

If native platform integrations are required, implement clean adapters and document their responsibilities.

Ensure consistent component behavior across platforms while allowing platform-specific interactions and visual conventions.
8. State Management
Implement a lightweight reactive state management system.

Required capabilities:
- Observable state.
- Reactive variables.
- State controllers.
- Component updates.
- Event streams.
- Lifecycle cleanup.
- Shared application state.
- Local component state.
- Derived state.
- State subscriptions.
Avoid unnecessary rendering and memory leaks.
9. Accessibility
Implement:
- Screen reader support.
- Semantic labels.
- Keyboard navigation.
- Focus management.
- Accessible color contrast.
- Touch target sizing.
- Reduced motion.
- Text scaling.
- Accessible form errors.
Accessibility must be considered during component development, not added at the end.
10. Package Structure
Organize the package using a modular architecture.

Suggested structure:
mantra_widgets/
├── package.mantra
├── manifest
├── README.md
├── CHANGELOG.md
├── LICENSE
├── docs/
│   ├── getting_started.md
│   ├── installation.md
│   ├── theming.md
│   ├── responsive_ui.md
│   ├── accessibility.md
│   └── api_reference.md
├── lib/
│   ├── core/
│   ├── foundation/
│   ├── theme/
│   ├── colors/
│   ├── typography/
│   ├── layout/
│   ├── buttons/
│   ├── cards/
│   ├── avatars/
│   ├── badges/
│   ├── images/
│   ├── carousel/
│   ├── navigation/
│   ├── forms/
│   ├── dialogs/
│   ├── notifications/
│   ├── loaders/
│   ├── lists/
│   ├── animations/
│   ├── ratings/
│   ├── decorations/
│   └── advanced/
├── examples/
│   ├── basic/
│   ├── forms/
│   ├── dashboard/
│   ├── ecommerce/
│   └── social_app/
├── tests/
│   ├── unit/
│   ├── integration/
│   ├── rendering/
│   └── accessibility/
└── benchmarks/
Adjust directory and manifest naming to Mantra's actual package conventions.
11. Package Management and Installation
Integrate the package with Mantra's official package manager.

The intended developer experience should be:
mantra add mantra_widgets
Import:
import mantra_widgets
The package must support:
- Semantic versioning.
- Dependency resolution.
- Version locking.
- Transitive dependencies.
- Package validation.
- Cross-platform compatibility checks.
- Documentation metadata.
- Reproducible installations.
Ensure the package can eventually be distributed through Mantra's official registry and supported package distribution channels.
12. Documentation and Examples
Provide complete documentation for every public component.

Each component must have:
1. Description.
2. API properties.
3. Default values.
4. Supported variants.
5. Usage examples.
6. Event handling.
7. Theme customization.
8. Responsive behavior.
9. Platform compatibility.
10. Known limitations.
Build a component showcase application called:
Mantra Widgets Gallery
It should contain:
- All available components.
- Interactive previews.
- Live property customization where supported.
- Light/dark theme switching.
- Mobile/tablet/web previews.
- Copyable Mantra code examples.
- Component search.
- Component categories.
- Documentation links.
13. Testing and Quality Assurance
Create comprehensive automated tests.

Test:
- Component construction.
- Property validation.
- Rendering.
- Layout calculations.
- State changes.
- Event callbacks.
- Theme inheritance.
- Responsive behavior.
- Animation lifecycle.
- Accessibility.
- Platform compatibility.
- Memory management.
Build visual regression tests where the rendering infrastructure supports them.

Every released component must have automated tests and at least one working example.

Do not publish placeholder components that merely expose names without implementing functionality.
14. Performance Requirements
Optimize for:
- Fast component creation.
- Efficient rendering.
- Minimal memory usage.
- Lazy rendering.
- List virtualization.
- Image caching.
- Animation performance.
- Reduced unnecessary state updates.
- Small package footprint.
Benchmark representative screens on Android, iOS, and web.
15. Development Roadmap
Do not attempt to implement the entire library in one untested development step.

Divide development into independently testable milestones.
MVP 1 — Foundation
- Widget core.
- Component architecture.
- Theme engine.
- Colors.
- Typography.
- Basic layout.
- Rendering adapters.
MVP 2 — Essential Components
- Buttons.
- Cards.
- Avatars.
- Badges.
- Images.
- Dividers.
- Basic lists.
- Typography components.
MVP 3 — Navigation and Interaction
- AppBar.
- Drawer.
- Tabs.
- Bottom navigation.
- Carousel.
- Accordion.
- Dialogs.
- Toasts.
- Bottom sheets.
MVP 4 — Forms
- Text fields.
- Dropdowns.
- Checkboxes.
- Radio buttons.
- Switches.
- Date and time pickers.
- Validation.
- Form controllers.
MVP 5 — Advanced UI
- Loaders.
- Shimmer.
- Animations.
- Ratings.
- Sliders.
- Sticky headers.
- Advanced lists.
- Responsive builders.
MVP 6 — Production Readiness
- Accessibility.
- Performance.
- Cross-platform integration.
- Complete testing.
- Documentation.
- Examples.
- Package registry integration.
MVP 7 — Widgets Gallery
Create a complete showcase application demonstrating every implemented component.
16. Development Rules
Follow these rules throughout implementation:
1. Inspect the existing Mantra codebase before making architectural decisions.
2. Use native Mantra syntax.
3. Do not introduce Dart or Flutter dependencies as the core implementation.
4. Do not duplicate existing compiler/runtime functionality unnecessarily.
5. Use object-oriented programming and polymorphism where appropriate.
6. Maintain strict type safety.
7. Follow Mantra's existing formatting and indentation conventions.
8. Use modular, reusable code.
9. Implement real functionality rather than mock components.
10. Keep platform-specific code isolated.
11. Add tests for every component.
12. Maintain backward compatibility.
13. Document every public API.
14. Do not break existing Mantra applications.
15. Avoid unnecessary third-party dependencies.
16. Never mark an incomplete component as production-ready.
17. Expected Deliverables
At completion, deliver:
1. Complete mantra_widgets package source code.
2. Core widget engine.
3. Design system.
4. Full component catalog.
5. Responsive layout engine.
6. Reactive state integration.
7. iOS rendering integration.
8. Android rendering integration.
9. Web rendering integration.
10. Package manager integration.
11. Automated tests.
12. Documentation.
13. Example applications.
14. Mantra Widgets Gallery.
15. Installation and usage guide.
16. Versioned release package.
18. Definition of Done
The project is considered complete only when:
- Developers can install mantra_widgets through Mantra's package manager.
- Developers can import it into .mantra applications.
- Components compile using the actual Mantra compiler.
- Components render correctly on supported platforms.
- Themes and customizations work.
- Responsive layouts work.
- Interactive components respond correctly.
- Tests pass.
- Documentation is available.
- The example applications run successfully.
FINAL INSTRUCTION
Start by auditing the existing Mantra compiler and frontend architecture.

Produce an architecture assessment and implementation plan first.

Then implement MVP 1 completely, including working source code, tests, and documentation.

After validating MVP 1, proceed milestone by milestone.

Do not stop at architecture diagrams, sample snippets, TODOs, or pseudocode.

Build an actual working UI component ecosystem that can become the official UI library of the Mantra Programming Language.

The ultimate goal is:

Mantra + mantra_widgets = A complete cross-platform application development experience with a rich, reusable, production-ready UI ecosystem.




Recommended package identity

mantra_widgets

Official Mantra UI Component Library
Recommended package name


Attribute	Value
Package	mantra_widgets
Import	import mantra_widgets
Initial release	0.1.0
Platforms	iOS, Android, Web
Architecture	Native Mantra
License	MIT (proposed)


My architectural recommendation
One important decision: don't build 1,000 independent widgets immediately.

Build a powerful shared foundation first. For example, 100 components should reuse the same theme engine, layout engine, animation engine, and event system.

This will make mantra_widgets easier to maintain and much more consistent.

I recommend eventually having three official Mantra UI packages:
Package	Responsibility
mantra_ui_core	Rendering, layout, lifecycle, state
mantra_widgets	Reusable UI components
mantra_icons	Complete icon library

This gives Mantra a scalable UI ecosystem rather than one enormous package.