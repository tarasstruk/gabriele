use crate::resolution::Resolution;
use std::default::Default;

/// Represents current coordinates of the printing element
/// against the Paper coordinate system.
///
/// The upper left corner of the Paper sheet is the pivot point
/// with coordinates `x == 0` and `y == 0`.
///
/// The pivot point is the default position when the Machine starts:
/// ```
/// use gabriele::position::Position;
/// let current_position = Position::default();
/// ```
/// The `x` increases in direction from the left to the right.
/// The `y` increases in direction from the top to the bottom.
///
/// `res` represents `Resolution` which is a part of `Position`.
/// Two positions are never equal to each other when their resolutions differ.
///
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub res: Resolution,
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
        (self.x - base.x) / self.res.x
    }

    pub fn align_to_string_length(&self, len: i32) -> Self {
        let mut pos = self.clone();
        pos.x = pos.res.x * (len - 1);
        pos
    }

    pub fn align_right(&self) -> Position {
        let mut pos = self.clone();
        pos.x += pos.res.x * 60;
        pos
    }
    pub fn step_right(&self) -> Position {
        let mut pos = self.clone();
        pos.x += pos.res.x;
        pos
    }

    pub fn step_left(&self) -> Position {
        let mut pos = self.clone();
        pos.x -= pos.res.x;
        pos
    }

    pub fn cr(&self, base: &Position) -> Position {
        let mut pos = base.clone();
        pos.y = self.y + self.res.y;
        pos
    }

    pub fn newline(&self) -> Position {
        let mut pos = self.clone();
        pos.y = self.y + self.res.y;
        pos
    }

    pub fn increment_x(&self, ratio: i32) -> Position {
        let mut pos = self.clone();
        pos.x = self.x + (self.res.x * ratio);
        pos
    }

    pub fn decrement_x(&self, ratio: i32) -> Position {
        let mut pos = self.clone();
        pos.x = self.x - (self.res.x * ratio);
        pos
    }
}
