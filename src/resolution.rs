pub const DEFAULT_X_RESOLUTION: i32 = 12;
pub const DEFAULT_Y_RESOLUTION: i32 = 16;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Resolution {
    pub x: i32,
    pub y: i32,
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution {
            x: DEFAULT_X_RESOLUTION,
            y: DEFAULT_Y_RESOLUTION,
        }
    }
}
