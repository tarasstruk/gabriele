use crate::database::DaisyDatabase;
use crate::motion::move_relative;
use crate::position::Position;
use crate::printing::{Action, Instruction};
use crate::to_symbols::ToSymbols;
use core::default::Default;
use itertools::Itertools;

pub trait InstructionSender: Sized {
    fn send(&self, instr: Instruction);
}

pub struct Machine<T: InstructionSender> {
    sender: T,
    position: Position,
    settings: Settings,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Settings {
    pub direction: PrintingDirection,
    pub base_position: Position,
}
#[derive(Default, Copy, Clone, Debug)]
pub enum PrintingDirection {
    #[default]
    Right,
    Left,
}

impl From<PrintingDirection> for i32 {
    fn from(direction: PrintingDirection) -> Self {
        match direction {
            PrintingDirection::Right => 1,
            PrintingDirection::Left => -1,
        }
    }
}

impl<T: InstructionSender> Machine<T> {
    pub fn new(sender: T) -> Self {
        let position = Default::default();
        let settings = Default::default();
        Self {
            sender,
            position,
            settings,
        }
    }

    pub fn current_position(&self) -> Position {
        self.position
    }

    pub fn shutdown(&mut self) {
        self.transmit([Instruction::Halt].into_iter());
    }

    pub fn transmit(&self, instructions: impl Iterator<Item = Instruction>) {
        for item in instructions {
            self.sender.send(item);
        }
    }

    pub fn print(&mut self, input: &str, db: impl DaisyDatabase + 'static) {
        let symbols = input
            .to_symbols(db)
            .dedup_by_with_count(|x, y| x == y && x.is_groupable());

        for (rep, symbol) in symbols {
            let action = Action::new(symbol, &self.settings, rep, &self.position);
            let target_pos = action.target_position();

            for instr in action.instructions(&target_pos) {
                self.sender.send(instr);
            }
            self.position = target_pos;
        }
    }

    pub fn offset(&mut self, value: i16) {
        self.transmit(move_relative(value, 0));
    }

    pub fn send_empty_instruction(&mut self) {
        self.transmit([].into_iter());
    }
}
