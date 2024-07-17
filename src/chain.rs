use std::{
    cmp::{max, min},
    f32::consts::PI,
    fmt::Display,
};

use crate::helper_functions::{move_point, Point};

const MAX_DIFF: f32 = PI / 3f32;

#[derive(Debug)]
pub struct Chain {
    pub head: Head,
}

#[derive(Debug)]
pub struct Head {
    point: Point,
    theta: f32,
    speed: f32, // Need to decide if this is pixels /s or /tick
    children: Vec<Point>,
}

// #[derive(Default, Clone, Debug)]
// pub struct Node {
//     point: Point,
// }

impl Chain {
    pub fn create(x: f32, y: f32) -> Chain {
        let head = Head {
            point: Point { x, y },
            theta: PI / 2f32,
            speed: 0.0,
            children: vec![Point::default(); 10],
        };

        Chain { head }
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
            "  Head - pos: {}, theta: {}, speed: {}\n  Children:\n",
            self.point, self.theta, self.speed
        );

        for child in &self.children {
            let child_str = format!("    {child}\n");
            base_str.push_str(&child_str);
        }

        write!(f, "{base_str}")
    }
}

// impl Display for Node {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "    {}", self.point)
//     }
// }

impl Head {
    // fn add_heading(self: &mut Self, heading: f32) {
    //     let heading_diff = max(min(heading, MAX_DIFF), -MAX_DIFF);

    // }

    pub fn move_chain(self: &mut Self) {
        // self.point.x += self.speed * self.theta.sin();
        // self.point.y += self.speed * self.theta.cos();
        self.point.x += 1.0;
        self.point.y += 1.0;

        // let new_self_point = Point {
        //     x: self.point.x + 1.0,
        //     y: self.point.y + 1.0,
        // }

        let radius = 0.5;

        let mut child_iter = self.children.clone().into_iter();

        let mut prev = child_iter.next().expect("First Child Must Exist");

        let mut new_children = vec![];

        let push_point = match move_point(&self.point, &prev, radius) {
            Some(new_point) => new_point,
            None => prev,
        };

        new_children.push(push_point.clone());

        prev = push_point;

        while let Some(curr) = child_iter.next() {
            let push_point = match move_point(&prev, &curr, radius) {
                Some(new_point) => new_point,
                None => curr,
            };
            new_children.push(push_point.clone());
            prev = push_point;
        }

        self.children = new_children;
    }
}
