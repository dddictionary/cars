use crate::engine::{Automaton, Cell};



pub fn apply_preset(name: &str, sim: &mut Automaton) -> Result<(), String> {
    let patterns = match name.to_lowercase().as_str() {
        "glider" => vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
        "blinker" => vec![(1, 0), (1, 1), (1, 2)],
        "block" => vec![(1, 1), (1, 2), (2, 1), (2, 2)],
        _ => return Err(format!("Unknown preset: {}", name)),
    };

    let offset_x = sim.width / 2;
    let offset_y = sim.height / 2;
    
    for (dx, dy) in patterns {
        let x = offset_x + dx;
        let y = offset_y + dy;

        sim.set_alive(x, y);
    }   

    Ok(())
}
