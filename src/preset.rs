use crate::engine::Automaton;

pub fn apply_preset(name: &str, sim: &mut Automaton, visible_width: usize, visible_height: usize) -> Result<(), String> {
    let patterns = match name.to_lowercase().as_str() {
        "glider" => vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
        "blinker" => vec![(1, 0), (1, 1), (1, 2)],
        "block" => vec![(1, 1), (1, 2), (2, 1), (2, 2)],
        _ => return Err(format!("Unknown preset: {}", name)),
    };

    // Estimate pattern size
    let pattern_width = patterns.iter().map(|(x, _)| x).max().unwrap_or(&0) + 1;
    let pattern_height = patterns.iter().map(|(_, y)| y).max().unwrap_or(&0) + 1;

    let offset_x = (visible_width / 2).saturating_sub(pattern_width / 2);
    let offset_y = (visible_height / 2).saturating_sub(pattern_height / 2);

    for (dx, dy) in patterns {
        let x = offset_x + dx;
        let y = offset_y + dy;

        sim.set_alive(x, y);
    }

    Ok(())
}

