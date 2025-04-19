# cars 

**cars** is a CLI + TUI tool for simulating custom, rule-based cellular automata. It supports multiple presets, custom rule input, and real-time visualizations using Ratatui.

---

## Features

- Simulate 2D binary-state cellular automata (e.g., Conway’s Game of Life)
- Custom rules in `B/S` notation (e.g., `B3/S23`)
- ASCII or interactive TUI rendering with Ratatui
- Built-in presets: `glider`, `blinker`, `block`
- CLI-based configuration of rules, grid size, steps, and more

---

## Usage

### Build the project

```bash
cargo build --release
```

### Run with default options

```bash
cargo run
```

### Run a glider in a 40x20 grid with TUI

```bash
cargo run -- --preset glider --width 40 --height 20 --ui tui
```

### Run with custom rule (e.g., HighLife)

```bash
cargo run -- --preset glider --rule B36/S23 --ui tui
```

### Run until Ctrl+C (no `--steps`)

```bash
cargo run -- --preset blinker --width 30 --height 30 --ui tui
```

---

## CLI Options

| Flag           | Description                                                         | Default      |
|----------------|---------------------------------------------------------------------|--------------|
| `--rule`       | Cellular automaton rule string in `B/S` format                      | `B3/S23`     |
| `--width`      | Grid width                                                          | `10`         |
| `--height`     | Grid height                                                         | `10`         |
| `--steps`      | Number of generations to simulate (omit to run until Ctrl+C)        | *(infinite)* |
| `--preset`     | Pattern to seed initially (`glider`, `blinker`, `block`)            | *(optional)* |
| `--ui`         | UI mode: `tui` for interactive, omit for plain terminal output      | *(optional)* |

---

## Rule Syntax

Uses standard [Life-like cellular automaton](https://conwaylife.com/wiki/Life-like_cellular_automaton) notation:

- `B` = birth conditions
- `S` = survival conditions

Example:  
`B3/S23` (Conway’s Game of Life):  
- Dead cells become alive with **exactly 3** neighbors  
- Live cells survive with **2 or 3** neighbors

---

## Future Plans

- Export simulation frames to GIF
- Support for random seeding
- Interactive controls: pause/resume, speed adjustment
- Support for non-totalistic and multi-state automata

---

## Tech Stack

- Rust 🦀
- [`clap`](https://crates.io/crates/clap) for argument parsing
- [`ratatui`](https://crates.io/crates/ratatui) for terminal UI
- [`crossterm`](https://crates.io/crates/crossterm) for input handling

---

## 📜 License

MIT

---
