#![allow(unused)]
#![allow(dead_code)]

use anyhow::Result;
use std::default::Default;

pub const DEFAULT_X_RESOLUTION: i32 = 12;
pub const DEFAULT_Y_RESOLUTION: i32 = 16;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub x_res: i32,
    pub y_res: i32,
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
    pub fn jump(&mut self, new_pos: &Position) {
        self.x = new_pos.x;
        self.y = new_pos.y;
    }

    pub fn diff(&self, base: &Position) -> (i32, i32) {
        (self.x - base.x, self.y - base.y)
    }

    pub fn x_offset(&self, base: &Position) -> i32 {
        (self.x - base.x) / self.x_res
    }

    pub fn align_to_string_length(&self, len: i32) -> Self {
        let mut pos = self.clone();
        pos.x = pos.x_res * (len - 1);
        pos
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

    pub fn newline(&self) -> Position {
        let mut pos = self.clone();
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
