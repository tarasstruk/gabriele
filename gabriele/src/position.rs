use crate::resolution::Resolution;
use core::default::Default;

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
    pub fn diff(&self, base: &Position) -> (i32, i32) {
        (self.x - base.x, self.y - base.y)
    }

    pub fn update_x(&mut self, value: i32) {
        self.x += self.res.x * value;
    }

    pub fn update_y(&mut self, value: i32) {
        self.y += self.res.y * value;
    }

    pub fn apply_line_feed(&mut self, value: i32) {
        self.update_y(value);
        self.x = 0;
    }
}
