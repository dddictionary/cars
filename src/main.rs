/// CLI-driven entry point for the cellular automata simulation.
///
/// Responsibilities:
/// - Accepts arguments from the user to configure the simulation:
///     - Rule string (e.g., "B3/S23")
///     - Grid dimensions (width x height)
///     - Number of generations to simulate
///     - Optional initial pattern file (or built-in presets)
/// - Initializes the automaton grid
/// - Seeds the grid based on input (or defaults to a known pattern)
/// - Runs the simulation for the given number of generations
/// - Prints each generation to the terminal using ASCII output
///
/// # CLI Argument Specification
/// - `--rule <RULE>`: A rule string like "B3/S23" (default: "B3/S23")
/// - `--width <W>` and `--height <H>`: Grid dimensions (default: 10x10)
/// - `--steps <N>`: Number of generations to run (default: 10)
/// - `--preset <PATTERN>`: Optional preset to seed the grid (e.g., "glider", "blinker")
///
/// # Future Extensions
/// - Support for random seeding
/// - Option to save output as a file (e.g., a GIF or text log)
/// - Interactive mode (step/pause/resume)
/// - TUI frontend toggle
// src/main.rs
use clap::Parser;
use std::{sync::atomic::{AtomicBool, Ordering}, thread, time::Duration};
use cars::{engine::Automaton, preset::apply_preset, rules::Rule};
use std::sync::Arc;

/// CLI options for the cellular automata simulation
#[derive(Parser, Debug)]
#[command(name = "Cellular Automata")]
#[command(about = "Simulates cellular automata like Conway's Game of Life", long_about = None)]
struct Cli {
    /// Rule string in the format Bx/Sy (e.g., B3/S23)
    #[arg(long, default_value = "B3/S23")]
    rule: String,

    /// Width of the grid
    #[arg(long, default_value_t = 10)]
    width: usize,

    /// Height of the grid
    #[arg(long, default_value_t = 10)]
    height: usize,

    /// Number of steps/generations to simulate
    #[arg(long)]
    steps: Option<usize>,

    /// Optional preset pattern to initialize the grid
    #[arg(long)]
    preset: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // Parse the rule string
    let rule = Rule::from_str(&cli.rule).expect("Invalid rule format");

    // Initialize the simulation grid
    let mut sim = Automaton::new(cli.width, cli.height, rule);

    // TODO: Apply preset pattern to seed the grid
    if let Some(name) = cli.preset.as_deref() {
        if let Err(e) = apply_preset(name, &mut sim) {
            eprintln!("Error: {}", e);
            return;
        }
    }

    // TODO: Run the simulation for the specified number of steps
    if let Some(steps) = cli.steps {
        for _ in 0..steps {
            sim.draw();
            sim.tick();
            thread::sleep(Duration::from_millis(100));
        }
    } else {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        }).expect("Error setting CTRL_C handler");

        while running.load(Ordering::SeqCst) {
            sim.draw();
            sim.tick();
            thread::sleep(Duration::from_millis(100));
        }
    }
}

