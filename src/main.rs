use clap::Parser;
use cars::{engine::Automaton, preset::apply_preset, rules::Rule, ui::tui};
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
}

fn main() {
    let cli = Cli::parse();

    let (width, height) = match (cli.width, cli.height) {
        (Some(w), Some(h)) => (w, h),
        _ => {
            let term = Terminal::new(ratatui::backend::CrosstermBackend::new(std::io::stdout())).unwrap();
            let size = term.size().unwrap();
            (size.width as usize, size.height as usize)
        }
    };

    let rule = Rule::from_str(&cli.rule).expect("Invalid rule format");
    let mut sim = Automaton::new(width, height, rule);

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

            if let Err(e) = tui::run_tui(&mut sim) {
                eprintln!("TUI error: {}", e);
            }
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

            loop {
                println!("{}", sim.as_string());
                sim.tick();
                std::thread::sleep(std::time::Duration::from_millis(150));
            }
        }
    }
}
