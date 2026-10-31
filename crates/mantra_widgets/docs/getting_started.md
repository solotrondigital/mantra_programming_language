# Getting Started with Mantra Widgets

Welcome to `mantra_widgets`, the official UI component framework for Mantra applications.

## Prerequisites

Ensure you have Mantra v0.5.0 installed:

```bash
mantra --version
```

## Adding the Dependency

Add `mantra_widgets` to your application's `mantra.toml`:

```bash
mantra add mantra_widgets 0.5.0
```

## Creating Your First Screen

In your project's `src/main.mantra`:

```mantra
import mantra_widgets.*

class MainScreen:
    pub def build(self) -> String:
        let btn = MantraButton("Explore", "primary")
        let col = Column(btn.render())
        return col.render()

def main() -> Int:
    let app = MantraApp("Mantra Demo", light_theme(), MainScreen().build())
    return app.start()
```

## Running the Application

Run in the browser development server:

```bash
mantra run -d chrome --web-port 4300
```

