#![allow(unused)]
#![allow(dead_code)]

use anyhow::Result;
use std::default::Default;

#[derive(Debug, Clone)]
pub struct Position {
    x: u32,
    y: u32,
    x_abs: u32,
    y_abs: u32,
    x_res: u32,
    y_res: u32,
    margin_left_abs: u32,
    margin_right_abs: u32,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            x: 0,
            y: 0,
            x_abs: 0,
            y_abs: 0,
            x_res: 12,
            y_res: 16,
            margin_left_abs: 0,
            margin_right_abs: 0,
        }
    }
}

impl Position {
    fn re_calc(&mut self) -> Result<()> {
        self.x_abs = self.x * self.x_res;
        self.y_abs = self.y * self.y_res;

        Ok(())
    }

    pub fn step_right(&mut self) -> Result<()> {
        self.x += 1;
        self.re_calc()
    }

    pub fn step_left(&mut self) -> Result<()> {
        self.x -= 1;
        self.re_calc()
    }

    pub fn step_up(&mut self) -> Result<()> {
        self.y -= 1;
        self.re_calc()
    }

    pub fn step_dn(&mut self) -> Result<()> {
        self.y += 1;
        self.re_calc()
    }
}
