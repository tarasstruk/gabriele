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
}

impl Position {
    pub fn jump(&mut self, new_pos: &Position) {
        self.x = new_pos.x;
        self.y = new_pos.y;
    }

    pub fn diff(&self, base: &Position) -> (i32, i32) {
        (self.x - base.x, self.y - base.y)
    }

    pub fn step_right(&self, res: Resolution) -> Position {
        let mut pos = *self;
        pos.x += res.x;
        pos
    }

    pub fn step_left(&self, res: Resolution) -> Position {
        let mut pos = *self;
        pos.x -= res.x;
        pos
    }

    pub fn cr(&self, base: &Position, res: Resolution) -> Position {
        let mut pos = *base;
        pos.y = self.y + res.y;
        pos
    }

    pub fn cr_multiple(&self, base: &Position, ratio: i32, res: Resolution) -> Position {
        let mut pos = *base;
        pos.y = self.y + (res.y * ratio);
        pos
    }

    pub fn newline(&self, res: Resolution) -> Position {
        let mut pos = *self;
        pos.y = self.y + res.y;
        pos
    }

    pub fn increment_x(&self, ratio: i32, res: Resolution) -> Position {
        let mut pos = *self;
        pos.x = self.x + (res.x * ratio);
        pos
    }

    pub fn decrement_x(&self, ratio: i32, res: Resolution) -> Position {
        let mut pos = *self;
        pos.x = self.x - (res.x * ratio);
        pos
    }
}
