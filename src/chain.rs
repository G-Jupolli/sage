use std::{f32::consts::PI, fmt::Display};

use crate::helper_functions::{pull_node_on_point, Point};

#[derive(Debug)]
pub struct Chain {
    pub head: Head,
    node_distancing: f32,
    max_x: i32,
    max_y: i32,
}

/*
    Direction radians:
        Right = 0
        down = π / 2
        left = π
        up = - π / 2

    dx = mag * cos(x)
    dy = mag * sin(x)
*/
#[derive(Debug)]
pub struct Head {
    pub point: Point,
    pub theta: f32,
    pub speed: f32, // Need to decide if this is pixels /s or /tick
    pub children: Vec<Node>,
}

struct HeadSides {
    pub left: Point,
    pub right: Point,
    pub left_eue: Point,
    pub right_eye: Point,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub point: Point,
    pub radial: f32,
    pub theta: f32,
    pub sides: Sides,
}

#[derive(Debug, Clone, Default)]
pub struct Sides {
    pub left: Point,
    pub right: Point,
}

impl Display for Sides {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Left: {}, Right: {}", self.left, self.right)
    }
}

impl From<(&Point, &f32, &f32)> for Sides {
    fn from((origin, radial, heading): (&Point, &f32, &f32)) -> Self {
        Sides {
            left: Point {
                x: origin.x + (heading - PI / 2.0).cos() * radial,
                y: origin.y + (heading - PI / 2.0).sin() * radial,
            },
            right: Point {
                x: origin.x + (heading + PI / 2.0).cos() * radial,
                y: origin.y + (heading + PI / 2.0).sin() * radial,
            },
        }
    }
}

#[derive(Debug)]
pub struct MoveCommand {
    up: bool,
    right: bool,
    down: bool,
    left: bool,
}

fn get_desire_heading(cmd: MoveCommand) -> Option<f32> {
    let xdir: Option<u8> = match (cmd.left, cmd.right) {
        (true, false) => Some(1),
        (false, true) => Some(0),
        _ => None,
    };

    let ydir: Option<i8> = match (cmd.up, cmd.down) {
        (true, false) => Some(-1),
        (false, true) => Some(1),
        _ => None,
    };

    /*
    x:
        0 - Right
        1 - Left
    y:
        -1 - up
        1 - down
    */
    let desire_heading: f32 = match (xdir, ydir) {
        (None, Some(y)) => y as f32 * PI / 2.0,
        (Some(x), None) => x as f32 * PI,
        (Some(x), Some(y)) => match (x, y) {
            (0, 1) => PI / 4.0,
            (0, -1) => -PI / 4.0,
            (1, 1) => 3.0 * PI / 4.0,
            (1, -1) => -3.0 * PI / 4.0,
            _ => panic!("Invalid Move Condition {:?}", cmd),
        },
        (None, None) => return None,
    };

    Some(desire_heading)
}

impl Chain {
    pub fn create(
        x: f32,
        y: f32,
        node_distancing: f32,
        node_radials: Vec<i16>,
        max_x: i32,
        max_y: i32,
    ) -> Chain {
        assert_eq!(node_radials.len(), 10, "There can only be 10 nodes");

        let head = Head {
            point: Point {
                x: x.clone(),
                y: y.clone(),
            },
            theta: -PI / 2f32,
            speed: 5.0,
            children: node_radials
                .into_iter()
                .enumerate()
                .map(|(index, radial)| {
                    let radial = radial as f32;
                    let origin = Point {
                        x: x.clone(),
                        y: y - (node_distancing * index as f32),
                    };
                    let heading = PI / 2f32;
                    Node {
                        sides: Sides::from((&origin, &radial, &heading)),
                        point: origin,
                        theta: heading,
                        radial,
                    }
                })
                .collect(),
        };

        Chain {
            head,
            node_distancing,
            max_x,
            max_y,
        }
    }

    pub fn travel(self: &mut Self) {
        self.head.move_chain(&self.node_distancing);
    }
}

impl Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chain\n{}", self.head)
    }
}

impl Display for Head {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut base_str = format!(
            "  Head - Pos: {}, Heading: {}, Speed: {}\n  Children:\n",
            self.point, self.theta, self.speed
        );

        for child in &self.children {
            let child_str = format!("    {child}\n");
            base_str.push_str(&child_str);
        }

        write!(f, "{base_str}")
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "    Pos: {}, Heading: {}, Sides: {}",
            self.point, self.theta, self.sides
        )
    }
}

impl Head {
    // fn add_heading(self: &mut Self, heading: f32) {
    //     let heading_diff = max(min(heading, MAX_DIFF), -MAX_DIFF);

    // }

    fn move_chain(self: &mut Self, spacing: &f32) {
        self.point.x += self.speed * self.theta.cos();
        self.point.y += self.speed * self.theta.sin();

        self.theta += 0.2;

        let mut child_iter = self.children.clone().into_iter();

        let mut new_children = vec![];

        let push_node = pull_node_on_point(
            &self.point,
            child_iter.next().expect("First Child Must Exist"),
            spacing,
        );

        let mut prev_point = push_node.point.clone();

        new_children.push(push_node);

        while let Some(curr) = child_iter.next() {
            let new_node = pull_node_on_point(&prev_point, curr, spacing);

            prev_point = new_node.point.clone();

            new_children.push(new_node);
        }

        self.children = new_children;
    }
}

impl Node {
    pub fn update_sides(self: &mut Self) {
        self.sides.left.x = self.point.x + (self.theta - PI / 2.0).cos() * self.radial;
        self.sides.left.y = self.point.y + (self.theta - PI / 2.0).sin() * self.radial;

        self.sides.right.x = self.point.x + (self.theta + PI / 2.0).cos() * self.radial;
        self.sides.right.y = self.point.y + (self.theta + PI / 2.0).sin() * self.radial;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_desire_heading() {
        // No Movement Commanded
        assert_eq!(
            get_desire_heading(MoveCommand {
                up: false,
                right: false,
                down: false,
                left: false
            }),
            None
        );
        // All Movement Commanded
        assert_eq!(
            get_desire_heading(MoveCommand {
                up: true,
                right: true,
                down: true,
                left: true
            }),
            None
        );

        // Straight Lines
        assert_eq!(
            get_desire_heading(MoveCommand {
                up: true,
                right: false,
                down: false,
                left: false
            }),
            Some(-PI / 2.0)
        );
        assert_eq!(
            get_desire_heading(MoveCommand {
                up: false,
                right: true,
                down: false,
                left: false
            }),
            Some(0.0)
        );
        assert_eq!(
            get_desire_heading(MoveCommand {
                up: false,
                right: false,
                down: true,
                left: false
            }),
            Some(PI / 2.0)
        );
        assert_eq!(
            get_desire_heading(MoveCommand {
                up: false,
                right: false,
                down: false,
                left: true
            }),
            Some(PI)
        );

        // Diagonals
        assert_eq!(
            get_desire_heading(MoveCommand {
                up: false,
                right: true,
                down: true,
                left: false
            }),
            Some(PI / 4.0)
        );
        assert_eq!(
            get_desire_heading(MoveCommand {
                up: false,
                right: false,
                down: true,
                left: true
            }),
            Some(3.0 * PI / 4.0)
        );
        assert_eq!(
            get_desire_heading(MoveCommand {
                up: true,
                right: true,
                down: false,
                left: false
            }),
            Some(-PI / 4.0)
        );
        assert_eq!(
            get_desire_heading(MoveCommand {
                up: true,
                right: false,
                down: false,
                left: true
            }),
            Some(-3.0 * PI / 4.0)
        );
    }
}
