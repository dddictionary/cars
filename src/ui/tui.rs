use crate::engine::Automaton;
use ratatui::{DefaultTerminal, Frame};
use crossterm::event::{self, Event};
use color_eyre::Result;

pub fn run_tui(_sim: &mut Automaton) -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(())
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("Hello World!", frame.area());
    //
    //if let Some(steps) = cli.steps {
    //    for _ in 0..steps {
    //        sim.draw();
    //        sim.tick();
    //        thread::sleep(Duration::from_millis(100));
    //    }
    //} else {
    //    let running = Arc::new(AtomicBool::new(true));
    //    let r = running.clone();
    //
    //    ctrlc::set_handler(move || {
    //        r.store(false, Ordering::SeqCst);
    //    }).expect("Error setting CTRL_C handler");
    //
    //    while running.load(Ordering::SeqCst) {
    //        sim.draw();
    //        sim.tick();
    //        thread::sleep(Duration::from_millis(100));
    //    }
    //}
}
