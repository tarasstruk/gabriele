#![allow(unused)]
#![allow(dead_code)]

use anyhow::Result;
use std::default::Default;
#[derive(Debug, Clone)]
pub struct Position {
    x: i32,
    y: i32,
    x_res: i32,
    y_res: i32,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            x: 0,
            y: 0,
            x_res: 12,
            y_res: 16,
        }
    }
}

impl Position {
    pub fn diff(&self, base: &Position) -> (i32, i32) {
        (self.x - base.x, self.y - base.y)
    }

    pub fn align_right(&self) -> Position {
        let mut pos = self.clone();
        pos.x += pos.x_res * 60;
        pos
    }
    pub fn step_right(&self) -> Position {
        let mut pos = self.clone();
        pos.x += pos.x_res;
        pos
    }

    pub fn step_left(&self) -> Position {
        let mut pos = self.clone();
        pos.x -= pos.x_res;
        pos
    }

    pub fn cr(&self, base: &Position) -> Position {
        let mut pos = base.clone();
        pos.y = self.y + self.y_res;
        pos
    }

    pub fn increment_x(&self, ratio: i32) -> Position {
        let mut pos = self.clone();
        pos.x = self.x + (self.x_res * ratio);
        pos
    }

    pub fn decrement_x(&self, ratio: i32) -> Position {
        let mut pos = self.clone();
        pos.x = self.x - (self.x_res * ratio);
        pos
    }
}
