#![allow(unused)]
#![allow(dead_code)]

use crate::machine::{PrintingDirection, Settings};
use crate::motion;
use crate::position::Position;
use crate::resolution::Resolution;
use crate::symbol::{ActionMapping, Symbol};
use either::Either;
use log::debug;

/// The basic directive for the machine
/// Idle specifies milliseconds
/// SendBytes specifies a sequence of 2 bytes to be send over a serial port
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Instruction {
    SendBytes(u16),
    Halt,
}

// Action defines what we do with a Symbol
// each instance of Action can produce a series of Instructions.
//
// If a concrete Action cannot be performed in the concrete conditions
// then the execution of program should be stopped.
//
// In the future releases the Action should be able to respond with a recovery option.
// For example a print-symbol action: when the carriage has reached the left limit,
// we cannot perform this action. The recovery-action would the carriage return.
// In this case a recovery strategy would look like this:
// - the current action (printing a symbol) cannot be performed;
// - tt responds with empty set (iterator) for instructions method;
// - the executor party checks for a recovery actions (should be proposed by the current one)
// - the recovery sequence would contain a copy of the current action at the end of the list

/// Action represents a concrete primitive action to be performed by the Machine
/// in the current conditions, taking into account the base_position and the current_position.
pub struct Action<'a> {
    pub symbol: &'static Symbol,
    pub settings: &'a Settings,
    pub repeat: usize,
}

impl<'a> Action<'a> {
    pub fn new(symbol: &'static Symbol, settings: &'a Settings, repeat: usize) -> Self {
        Self {
            symbol,
            settings,
            repeat,
        }
    }

    pub fn whitespace_instructions(
        &self,
        old_position: &Position,
        new_position: &Position,
    ) -> impl Iterator<Item = Instruction> {
        match self.settings.direction {
            PrintingDirection::Left if (self.repeat == 1) => {
                Either::Left(motion::space_jump_left())
            }
            PrintingDirection::Right if (self.repeat == 1) => {
                Either::Right(Either::Left(motion::space_jump_right()))
            }
            _ => Either::Right(Either::Right(motion::move_absolute(
                old_position,
                new_position,
            ))),
        }
    }

    /// Generates a sequence of the Instructions,
    /// taking the current Position as a reference point.
    /// The result of these instructions is the printed Symbol or/and the associated motion.
    pub fn instructions(
        &self,
        base_position: &Position,
        current_position: &mut Position,
    ) -> impl Iterator<Item = Instruction> + use<'_> {
        let old_position = *current_position;
        self.update_position(base_position, current_position);
        debug!("action {:?}", self.symbol.act);
        match self.symbol.act {
            ActionMapping::Print => Either::Left(self.symbol.instructions(self.settings.direction)),
            ActionMapping::Whitespace => Either::Right(Either::Left(
                self.whitespace_instructions(&old_position, current_position),
            )),
            ActionMapping::LineFeed => Either::Right(Either::Right(
                (motion::move_absolute(&old_position, current_position)),
            )),
        }
    }

    /// New position represents a calculated desired Position
    /// where the machine is expected to be
    /// after the generated Instructions have been executed.
    pub fn update_position(&self, base_position: &Position, position: &mut Position) {
        let resolution = &self.settings.resolution;
        let pos = match self.symbol.act {
            ActionMapping::Print => match self.settings.direction {
                PrintingDirection::Right => {
                    &position.increment_x(self.symbol.x_positions_increment(), resolution)
                }
                PrintingDirection::Left => {
                    &position.decrement_x(self.symbol.x_positions_increment(), resolution)
                }
            },

            ActionMapping::Whitespace => &position.increment_x(
                self.repeat as i32 * i32::from(self.settings.direction),
                resolution,
            ),

            ActionMapping::LineFeed => {
                &position.cr_multiple(base_position, self.repeat as i32, resolution)
            }
        };
        position.jump(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::Action;
    use crate::machine::Settings;
    use crate::position::Position;
    use crate::printing::Instruction;
    use crate::printing::Instruction::SendBytes;
    use crate::resolution::Resolution;
    use crate::symbol::Symbol;

    static U_UMLAUT_SYMBOL: Symbol = Symbol::new('ü').petal(81);
    static LINE_FEED_SYMBOL: Symbol = Symbol::line_feed();

    #[test]
    fn test_print_symbol() {
        let mut pos: Position = Default::default();
        let base_pos: Position = Default::default();

        let settings = Settings::default();
        let action = Action::new(&U_UMLAUT_SYMBOL, &settings, 1);
        let mut commands = action.instructions(&base_pos, &mut pos);
        let pos_diff = pos.diff(&base_pos);

        assert_eq!(pos_diff, (12, 0));
        assert_eq!(
            commands.next(),
            Some(SendBytes(u16::from_be_bytes([81, 31 + 128])))
        );

        assert_eq!(commands.next(), None);
    }

    #[test]
    fn test_line_feed_coordinates() {
        let resolution = Resolution::default();
        let base_pos: Position = Default::default();
        let mut pos = base_pos.clone();
        // emulate the motion result caused by printing of 10 characters
        // causing the carriage to move by X=+120 units, when Y=0
        for _ in 0..10 {
            pos.jump(&pos.step_right(resolution));
        }
        assert_eq!(pos.diff(&base_pos), (120, 0));

        let mut pos: Position = Default::default();
        let base_pos: Position = Default::default();
        let settings = Settings::default();
        let action: Action = Action::new(&LINE_FEED_SYMBOL, &settings, 1);
        let mut instructions = action.instructions(&base_pos, &mut pos);
        assert!(instructions.next().is_some());

        // The distance between the base point should be only
        // relevant in the +Y direction
        assert_eq!(pos.diff(&base_pos), (0, 16));
    }

    #[test]
    fn test_line_feed_instructions() {
        let resolution = Resolution::default();
        let base_pos: Position = Default::default();
        let mut pos = base_pos.clone();
        for _ in 0..10 {
            pos.jump(&pos.step_right(resolution));
        }

        let settings = Settings::default();
        let action = Action::new(&LINE_FEED_SYMBOL, &settings, 1);
        let mut cmd = action.instructions(&base_pos, &mut pos);

        let details = u16::from_be_bytes([0b1110_0000, 120]);
        assert_eq!(cmd.next(), Some(Instruction::SendBytes(details)));

        let details = u16::from_be_bytes([0b1101_0000, 16]);
        assert_eq!(cmd.next(), Some(Instruction::SendBytes(details)));
        assert_eq!(cmd.next(), None);
    }
}
