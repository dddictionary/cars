use crate::{engine::Automaton, preset::apply_preset};
use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};

pub fn run_tui(sim: &mut Automaton, preset_name: &str) -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let term_size = ratatui::prelude::Terminal::size(&terminal)?;
    let visible_width = term_size.width as usize;
    let visible_height = term_size.height as usize;
    let _ = apply_preset(preset_name, sim, visible_width, visible_height);

    let result = run(terminal, sim);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, sim: &mut Automaton) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, sim))?;
        sim.tick();
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, sim: &Automaton) {
    let area: Rect = frame.area();
    let block = Paragraph::new(sim.as_string())
        .block(Block::default().title("Automation").borders(Borders::ALL));
    frame.render_widget(block, area);
}
