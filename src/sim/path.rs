use super::{types::Vector, RelCoords};

pub fn get_path_deltas(start: Vector, end: Vector) -> impl IntoIterator<Item = RelCoords> {
    let mut path_iter = get_path(start, end).into_iter();
    let mut prev = path_iter.next().expect("path always has at least one element (the start/end)");
    path_iter.map(move |current| {
        let delta = current - prev;
        prev = current;
        delta
    })
}

pub fn get_path(start: Vector, end: Vector) -> Vec<RelCoords> {
    let start_coords = RelCoords::from(start);
    let end_coords = RelCoords::from(end);

    match (end_coords - start_coords).as_tuple() {
        (0, 0) => vec![start_coords],
        (0, dy) if dy > 0 => (start_coords.y..end_coords.y + 1).map(|y| RelCoords::new(start_coords.x, y)).collect(),
        (0, dy) if dy < 0 => (end_coords.y..start_coords.y + 1).rev().map(|y| RelCoords::new(start_coords.x, y)).collect(),
        (dx, 0) if dx > 0 => (start_coords.x..end_coords.x + 1).map(|x| RelCoords::new(x, start_coords.y)).collect(),
        (dx, 0) if dx < 0 => (end_coords.x..start_coords.x + 1).rev().map(|x| RelCoords::new(x, start_coords.y)).collect(),
        (dx, dy) => {
            let mut path = vec![RelCoords::from(start)];
            let dx = dx.signum() as f32;
            let dy = dy.signum() as f32;

            let delta_pos = (end - start).abs();

            let dx_in_cell = if dx < 0.0 { fract_below(start.x) } else { fract_above(start.x) };
            let dy_in_cell = if dy < 0.0 { fract_below(start.y) } else { fract_above(start.y) };

            let mut traversed = Vector::ZERO;

            loop {
                if (traversed.x.abs() + dx_in_cell) * delta_pos.y > (traversed.y.abs() + dy_in_cell) * delta_pos.x {
                    // |dx| * |slope| > |dy|, so traverse vertically
                    traversed.y += dy;
                } else {
                    traversed.x += dx;
                }
                
                let next_coords = (start + traversed).into();
                path.push(next_coords);
                if next_coords == end_coords {
                    return path;
                }
            }
        },
    }
}

/// Equal to `ceil(f) - f`
pub fn fract_above(f: f32) -> f32 {
    f.ceil() - f
}

/// Equal to `f - floor(f)`
pub fn fract_below(f: f32) -> f32 {
    f - f.floor()
}
