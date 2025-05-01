use crate::engine::Automaton;
use rand::Rng; // Add at top

pub fn apply_preset(name: &str, sim: &mut Automaton, _visible_width: usize, _visible_height: usize) -> Result<(), String> {
    let patterns = match name.to_lowercase().as_str() {
        "glider" => vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
        "blinker" => vec![(1, 0), (1, 1), (1, 2)],
        "block" => vec![(1, 1), (1, 2), (2, 1), (2, 2)],
        "random" => {
            let mut rng = rand::rng();
            for y in 0..sim.height {
                for x in 0..sim.width {
                    if rng.random_bool(0.5) {
                        sim.set_alive(x, y);
                    }
                }
            }
            return Ok(()); // exit early for random
        }
        _ => return Err(format!("Unknown preset: {}", name)),
    };

    // 🛠 FIX: Center based on simulation grid, not terminal size
    let offset_x = sim.width / 2;
    let offset_y = sim.height / 2;

    for (dx, dy) in patterns {
        let x = offset_x + dx;
        let y = offset_y + dy;

        if x < sim.width && y < sim.height {
            sim.set_alive(x, y);
        }
    }

    Ok(())
}
