use crate::{engine::Automaton, stats::Stats};
use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Paragraph, Wrap},
};

pub fn run_tui(sim: &mut Automaton, stat_log: &mut Vec<Stats>) -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, sim, stat_log);
    ratatui::restore();
    result
}

fn run(
    mut terminal: DefaultTerminal,
    sim: &mut Automaton,
    stat_log: &mut Vec<Stats>,
) -> Result<()> {
    // println!("Entered run()"); // Confirm function starts
    loop {
        // println!("Drawing frame...");

        terminal.draw(|frame| {
            // println!("Inside draw closure");
            render(frame, sim);
        })?;

        sim.tick();
        // println!("Ticked simulation");
        stat_log.push(Stats::compute(
            sim.generation,
            sim.live_cells,
            sim.grid.len(),
        ));
        // Sleep to slow things down and confirm looping
        std::thread::sleep(std::time::Duration::from_millis(100));

        if event::poll(std::time::Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                // println!("Received key: {:?}", key);
                break Ok(());
            }
        }
    }
}

fn render(frame: &mut Frame, sim: &Automaton) {
    let area = frame.area();
    // println!(
    //     "Render area: {}x{}, Sim grid: {}x{}",
    //     area.width, area.height, sim.width, sim.height
    // );

    let paragraph = Paragraph::new(sim.as_string()).wrap(Wrap { trim: false }); // 👈 This enables soft line wrapping

    frame.render_widget(paragraph, area);
}
