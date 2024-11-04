use crate::database::Db;
use crate::motion::move_carriage;
use crate::position::Position;
use crate::printing::{Action, Instruction};
use crate::resolution::Resolution;
use log::info;
use std::default::Default;
use tokio::sync::mpsc::UnboundedSender;

pub struct Machine {
    sender: UnboundedSender<Instruction>,
    base_pos: Position,
    pos: Position,
    settings: Settings,
    #[allow(unused)]
    res: Resolution,
}

#[derive(Default, Copy, Clone)]
pub struct Settings {
    pub direction: PrintingDirection,
}
#[derive(Default, Copy, Clone)]
pub enum PrintingDirection {
    #[default]
    Right,
    Left,
}

impl Machine {
    pub fn new(sender: UnboundedSender<Instruction>) -> Self {
        let pos = Position::default();
        let base_pos = pos.clone();
        let res = Resolution::default();
        let settings = Settings::default();
        Self {
            sender,
            base_pos,
            pos,
            res,
            settings,
        }
    }

    pub fn current_position(&self) -> Position {
        self.pos
    }

    pub fn shutdown(&mut self) {
        info!("stopping the machine");
        self.transmit([Instruction::Shutdown].into_iter());
    }

    pub fn transmit(&mut self, instructions: impl Iterator<Item = Instruction>) {
        for item in instructions {
            self.sender
                .send(item)
                .expect("the communication channel is closed");
        }
    }

    pub fn print(&mut self, input: &str, db: &Db) {
        for symbol in db.printables(input) {
            let action = Action::new(symbol.clone(), self.settings);
            let instructions = action.instructions(&self.base_pos, &mut self.pos);
            self.transmit(instructions);
        }
    }
    pub fn offset(&mut self, value: i32) {
        let instructions = move_carriage(value);
        self.transmit(instructions);
    }

    pub fn send_empty_instruction(&mut self) {
        self.transmit([Instruction::Empty].into_iter());
    }
}
