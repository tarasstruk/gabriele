use crate::database::DaisyDatabase;
use crate::motion::move_relative;
use crate::position::Position;
use crate::printing::{Action, Instruction};
use crate::to_symbols::ToSymbols;
use core::default::Default;
use itertools::Itertools;

pub trait InstructionSender {
    #[allow(async_fn_in_trait)]
    async fn send(&self, instr: Instruction);
}

pub struct Machine<T: InstructionSender, D: DaisyDatabase + 'static> {
    sender: T,
    position: Position,
    settings: Settings,
    db: D,
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

impl<T: InstructionSender, D: DaisyDatabase + 'static> Machine<T, D> {
    pub fn new(sender: T, db: D) -> Self {
        let position = Default::default();
        let settings = Default::default();
        Self {
            sender,
            position,
            settings,
            db,
        }
    }

    pub fn current_position(&self) -> Position {
        self.position
    }

    pub async fn shutdown(&mut self) {
        self.transmit([Instruction::Halt].into_iter()).await;
    }

    pub async fn transmit(&self, instructions: impl Iterator<Item = Instruction>) {
        for item in instructions {
            self.sender.send(item).await;
        }
    }

    pub async fn print(&mut self, input: &str) {
        let symbols = input
            .to_symbols(&self.db)
            .dedup_by_with_count(|x, y| x == y && x.is_groupable());

        for (rep, symbol) in symbols {
            let action = Action::new(symbol, &self.settings, rep, &self.position);
            let target_pos = action.target_position();

            for instr in action.instructions(&target_pos) {
                self.sender.send(instr).await;
            }
            self.position = target_pos;
        }
    }

    pub async fn offset(&mut self, value: i16) {
        self.transmit(move_relative(value, 0)).await;
    }

    pub async fn send_empty_instruction(&mut self) {
        self.transmit([].into_iter()).await;
    }
}
