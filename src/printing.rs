#![allow(unused)]
#![allow(dead_code)]

use crate::daisy::{ActionMapping, AfterSymbolPrinted, Symbol};
use crate::machine::{PrintingDirection, Settings};
use crate::motion;
use crate::position::Position;
use log::debug;

/// The basic directive for the machine
/// Idle specifies milliseconds
/// SendBytes specifies a sequence of 2 bytes to be send over a serial port
#[derive(PartialEq, Debug)]
pub enum Instruction {
    #[allow(unused)]
    Idle(u64),
    SendBytes([u8; 2]),
    Empty,
    Shutdown,
}

impl Instruction {
    pub fn bytes(b1: u8, b2: u8) -> Self {
        Self::SendBytes([b1, b2])
    }
    pub fn wait_short() -> Self {
        Self::Idle(200)
    }

    pub fn wait_tiny() -> Self {
        Self::Idle(50)
    }

    pub fn wait_long() -> Self {
        Self::Idle(1000)
    }
}

impl From<u16> for Instruction {
    fn from(value: u16) -> Self {
        Self::SendBytes(value.to_be_bytes())
    }
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
pub struct Action {
    pub symbol: Symbol,
    pub base_position: Position,
    pub current_position: Position,
    pub settings: Settings,
}

impl Action {
    pub fn new(
        symbol: Symbol,
        base_position: Position,
        current_position: Position,
        settings: Settings,
    ) -> Self {
        Self {
            symbol,
            base_position,
            current_position,
            settings,
        }
    }

    /// Generates a sequence of the Instructions,
    /// taking the current Position as a reference point.
    /// The result of these instructions is the printed Symbol or/and the associated motion.
    pub fn instructions(&self) -> impl Iterator<Item = Instruction> {
        debug!("action {:?}", self.symbol.act);
        match self.symbol.act {
            ActionMapping::Print => self.symbol.instructions(self.settings.direction),
            ActionMapping::Whitespace => match self.settings.direction {
                PrintingDirection::Right => motion::space_jump_right(),
                PrintingDirection::Left => motion::space_jump_left(),
            },
            ActionMapping::CarriageReturn => {
                let new_pos = self.current_position.cr(&self.base_position);
                motion::move_absolute(self.current_position.clone(), new_pos)
            }
        }
    }

    /// New position represents a calculated desired Position
    /// where the machine is expected to be
    /// after the generated Instructions have been executed.
    pub fn new_position(&self) -> Position {
        match self.symbol.act {
            ActionMapping::Print => match self.settings.direction {
                PrintingDirection::Right => {
                    self.current_position.increment_x(self.symbol.x_ratio())
                }
                PrintingDirection::Left => self.current_position.decrement_x(self.symbol.x_ratio()),
            },

            ActionMapping::Whitespace => match self.settings.direction {
                PrintingDirection::Right => self.current_position.step_right(),
                PrintingDirection::Left => self.current_position.step_left(),
            },
            ActionMapping::CarriageReturn => self.current_position.cr(&self.base_position),
        }
    }

    // pub async fn run(self, runner: &mut impl InstructionRunner) {
    //     for cmd in self.instructions() {
    //         match cmd {
    //             Instruction::SendBytes(bytes) => runner.send_bytes(&bytes),
    //             Instruction::Idle(millis) => runner.idle(millis),
    //             Instruction::Empty => continue,
    //         }
    //     }
    //     sender.update_position(self.new_position());
    // }
}

#[cfg(test)]
mod tests {
    use super::Action;
    use crate::daisy::Symbol;
    use crate::position::Position;
    use crate::printing::Instruction;

    #[test]
    fn test_print_symbol() {
        let symbol = Symbol::new(81, 'ü');
        let pos: Position = Default::default();
        let action = Action::new(symbol, pos.clone(), pos.clone(), Default::default());
        let mut commands = action.instructions();
        let new_pos = action.new_position();
        let pos_diff = new_pos.diff(&pos);
        assert_eq!(pos_diff, (12, 0));
        assert_eq!(commands.next(), Some(Instruction::bytes(81, 31 + 128)));
        assert_eq!(commands.next(), None);
    }

    #[test]
    fn test_carriage_return_coordinates() {
        let symbol = Symbol::cr();
        let base_pos: Position = Default::default();
        let mut pos = base_pos.clone();
        // emulate the motion result caused by printing of 10 characters
        // causing the carriage to move by X=+120 units, when Y=0
        for _ in 0..10 {
            pos = pos.step_right();
        }
        let action = Action::new(symbol, base_pos.clone(), pos.clone(), Default::default());
        // let mut commands = action.instructions();
        let new_pos = action.new_position();
        // debug!("POS: {:?}", pos);
        // the diff is the offset of the actual position (after CR is done)
        // from the previous position:
        //   * 120 units in -X direction (to the left side)
        //   * 16 units in the +Y direction (the distance between rows)
        assert_eq!(new_pos.diff(&pos), (-120, 16));
        // at the same time, the distance between the base point should be only
        // relevant in the +Y direction
        assert_eq!(new_pos.diff(&base_pos), (0, 16));
    }

    #[test]
    fn test_carriage_return_instructions() {
        let symbol = Symbol::cr();
        let base_pos: Position = Default::default();
        let mut pos = base_pos.clone();
        for _ in 0..10 {
            pos = pos.step_right();
        }
        let action = Action::new(symbol, base_pos, pos, Default::default());
        let mut cmd = action.instructions();

        assert_eq!(cmd.next(), Some(Instruction::Idle(200)));
        assert_eq!(cmd.next(), Some(Instruction::bytes(0b1110_0000, 120)));
        assert_eq!(cmd.next(), Some(Instruction::Idle(1000)));
        assert_eq!(cmd.next(), Some(Instruction::Idle(200)));
        assert_eq!(cmd.next(), Some(Instruction::bytes(0b1101_0000, 16)));
        assert_eq!(cmd.next(), Some(Instruction::Idle(1000)));
        assert_eq!(cmd.next(), None);
    }
}
