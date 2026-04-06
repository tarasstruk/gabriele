use crate::database::DaisyDatabase;
use crate::motion::move_relative;
use crate::position::Position;
use crate::printing::{Action, Instruction};
use crate::resolution::Resolution;
use crate::to_symbols::ToSymbols;
use itertools::Itertools;
use log::info;
use std::default::Default;
use tokio::sync::mpsc::UnboundedSender;

pub struct Machine {
    sender: UnboundedSender<Instruction>,
    position: Position,
    settings: Settings,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Settings {
    pub direction: PrintingDirection,
    pub resolution: Resolution,
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

impl Machine {
    pub fn new(sender: UnboundedSender<Instruction>) -> Self {
        let position = Position::default();
        let settings = Settings::default();
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
        info!("stopping the machine");
        self.transmit([Instruction::Halt].into_iter());
    }

    pub fn transmit(&self, instructions: impl Iterator<Item = Instruction>) {
        for item in instructions {
            self.sender
                .send(item)
                .expect("the communication channel is closed");
        }
    }

    pub fn print(&mut self, input: &str, db: impl DaisyDatabase + 'static) {
        for (rep, symbol) in input
            .to_symbols(db)
            .dedup_by_with_count(|x, y| x == y && x.is_groupable())
        {
            let action = Action::new(symbol, &self.settings, rep);
            let instructions = action.instructions(&mut self.position);
            self.transmit(instructions);
        }
    }
    pub fn offset(&mut self, value: i16) {
        self.transmit(move_relative(value, 0));
    }

    pub fn send_empty_instruction(&mut self) {
        self.transmit([].into_iter());
    }
}
