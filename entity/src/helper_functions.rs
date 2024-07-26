use std::{f32::consts::PI, fmt::Display};

use crate::chain::Node;

#[derive(Default, Clone, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub fn point_bounding_rect(point: &Point, radius: &f64) -> [f64; 4] {
    [
        point.x as f64 - radius,
        point.y as f64 - radius,
        point.x as f64 + radius,
        point.y as f64 + radius,
    ]
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn pull_node_on_point(anchor: &Point, mut mover: Node, radius: &f32) -> Node {
    match move_point(anchor, &mover.point, radius) {
        Some(new_pos) => {
            mover.point = new_pos;
        }
        None => {
            return mover;
        }
    }

    mover.theta = get_point_heading(anchor, &mover.point);
    mover.update_sides();

    mover
}

/*
    dx = mover_x - anchor_x
    dy = mover_y - anchor_y

    theta = arctan( dy / dx )
*/
fn get_point_heading(anchor: &Point, mover: &Point) -> f32 {
    let dx = anchor.x - mover.x;
    let dy = anchor.y - mover.y;

    let base_theta = (dy / dx).atan();

    return base_theta;
}

/*
    prev = anchor point
    curr = moving point

    dx = prev_x - curr_x
    dy = prev_y - curr_y

    dist = sqrt( dy^2 + dx^2 )

    if dist <= radius no need to move

    percent_change = radius / dist

    new_x = prev_x + ( dx * percent_change )
    new_y = prev_y + ( dy * percent_change )

*/
pub fn move_point(anchor: &Point, mover: &Point, radius: &f32) -> Option<Point> {
    let dx = mover.x - anchor.x;
    let dy = mover.y - anchor.y;

    let distance = (dx.powi(2) + dy.powi(2)).sqrt();

    // Node is already in radius so can return no new pos
    if &distance <= radius {
        return None;
    }

    let percent_diff = radius / distance;

    Some(Point {
        x: anchor.x + (dx * percent_diff),
        y: anchor.y + (dy * percent_diff),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pull_direction_x() {
        let anchor = Point { x: 10f32, y: 0f32 };
        let mover = Point { x: 3f32, y: 0f32 };
        let result = move_point(&anchor, &mover, &5f32);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.x, 5f32);
        assert_eq!(result.y, 0f32);
    }

    #[test]
    fn pull_direction_y() {
        {
            let anchor = Point { x: 0f32, y: 10f32 };
            let mover = Point { x: 0f32, y: 3f32 };
            let result = move_point(&anchor, &mover, &5f32);
            assert!(result.is_some());
            let result = result.unwrap();
            assert_eq!(result.x, 0f32);
            assert_eq!(result.y, 5f32);
        }
    }

    #[test]
    fn pull_at_angle() {
        let anchor = Point { x: 3.0, y: 3.0 };
        let mover = Point { x: 0.0, y: 5.0 };

        let radius = 1.8;

        let result = move_point(&anchor, &mover, &radius);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(format!("{:.3}", result.x), "1.502");
        assert_eq!(format!("{:.3}", result.y), "3.998");
    }

    #[test]
    fn no_move_need() {
        let anchor = Point { x: 3.0, y: 3.0 };
        let mover = Point { x: 0.0, y: 5.0 };

        let radius = 100.0;

        let result = move_point(&anchor, &mover, &radius);
        assert!(result.is_none());
    }
}
