#![allow(unused)]
#![allow(dead_code)]

use anyhow::Result;
use std::default::Default;
use std::panic::catch_unwind;

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
    pub fn step_right(&mut self) -> Result<()> {
        self.x += self.x_res;
        Ok(())
    }

    pub fn step_left(&mut self) -> Result<()> {
        self.x -= self.x_res;
        Ok(())
    }

    pub fn step_up(&mut self) -> Result<()> {
        self.y -= self.y_res;
        Ok(())
    }

    pub fn step_dn(&mut self) -> Result<()> {
        self.y += self.y_res;
        Ok(())
    }

    pub fn delta_x_from(&mut self, base: &Position) -> i32 {
        self.x - base.x
    }

    pub fn delta_y_from(&mut self, base: &Position) -> i32 {
        self.y - base.y
    }

    pub fn reset(&mut self) -> Result<()> {
        self.x = 0;
        self.y = 0;
        Ok(())
    }

    pub fn diff(&self, base: &Position) -> (i32, i32) {
        (self.x - base.x, self.y - base.y)
    }

    pub fn carriage_return(&mut self, base: &Position) -> (i32, i32) {
        let mut new_pos = base.clone();
        new_pos.y = self.y + self.y_res;
        let diffs = new_pos.diff(self);
        self.x = new_pos.x;
        self.y = new_pos.y;
        diffs
    }
}
