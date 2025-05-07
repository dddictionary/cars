use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use cars::{engine::Automaton, preset::apply_preset, rules::Rule, stats::Stats, ui::tui};
use clap::Parser;
use ratatui::prelude::Terminal; // only needed to get terminal size

/// CLI options for the cellular automata simulation
#[derive(Parser, Debug)]
#[command(name = "Cellular Automata")]
#[command(about = "Simulates cellular automata like Conway's Game of Life", long_about = None)]
struct Cli {
    /// Rule string in the format Bx/Sy (e.g., B3/S23)
    #[arg(long, default_value = "B3/S23")]
    rule: String,

    /// Width of the grid
    // #[arg(long, default_value_t = 10)]
    #[arg(long)]
    width: Option<usize>,

    /// Height of the grid
    // #[arg(long, default_value_t = 10)]
    #[arg(long)]
    height: Option<usize>,

    /// Number of steps/generations to simulate
    #[arg(long)]
    steps: Option<usize>,

    /// Optional preset pattern to initialize the grid
    #[arg(long)]
    preset: Option<String>,

    /// Optional ui to ouput the result to. Ideally in the future there would be an option for
    /// outputting to gifs, and other file formats.
    #[arg(long)]
    ui: Option<String>,

    #[arg(long)]
    stats: Option<String>,
}

fn save_stats(path: &str, stats: &[Stats]) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(path)?;
    writeln!(file, "generation,live_cells,entropy")?;
    for s in stats {
        writeln!(file, "{},{},{}", s.generation, s.live_cells, s.entropy)?;
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let (width, height) = match (cli.width, cli.height) {
        (Some(w), Some(h)) => (w, h),
        _ => {
            let term =
                Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout())).unwrap();
            let size = term.size().unwrap();
            (size.width as usize, size.height as usize)
        }
    };

    let rule = Rule::from_str(&cli.rule).expect("Invalid rule format");
    let mut sim = Automaton::new(width, height, rule);
    let path = cli.stats.unwrap_or("stats_log.csv".to_string());
    let mut stat_log: Vec<Stats> = Vec::new();
    match cli.ui.as_deref() {
        Some("tui") => {
            // compute terminal size before init
            // let term = Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout())).unwrap();
            // let size = term.size().unwrap();
            // let visible_width = size.width as usize;
            // let visible_height = size.height as usize;

            // apply preset BEFORE TUI starts
            if let Some(name) = cli.preset.as_deref() {
                if let Err(e) = apply_preset(name, &mut sim, width, height) {
                    eprintln!("Preset error: {e}");
                    return;
                }
            }

            if let Err(e) = tui::run_tui(&mut sim, &mut stat_log) {
                eprintln!("TUI error: {}", e);
            }

            let _ = save_stats(&path, &stat_log);
        }
        Some(other) => {
            eprintln!("Unsupported UI mode: {other}");
        }
        None => {
            if let Some(name) = cli.preset.as_deref() {
                if let Err(e) = apply_preset(name, &mut sim, width, height) {
                    eprintln!("Preset error: {e}");
                    return;
                }
            }
            let running = Arc::new(AtomicBool::new(true));
            let r = running.clone();

            ctrlc::set_handler(move || {
                r.store(false, Ordering::SeqCst);
            })
            .expect("Error setting Ctrl-C handler");
            while running.load(Ordering::SeqCst) {
                println!("{}", sim.as_string());
                sim.tick();
                stat_log.push(Stats::compute(
                    sim.generation,
                    sim.live_cells,
                    sim.grid.len(),
                ));
                std::thread::sleep(std::time::Duration::from_millis(150));
            }

            let _ = save_stats(&path, &stat_log);
        }
    }
}
