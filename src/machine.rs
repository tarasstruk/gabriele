use crate::database::Db;
use crate::position::Position;
use crate::printing::{Action, Instruction};
use log::info;
use std::default::Default;
use tokio::sync::mpsc::UnboundedSender;

#[allow(unused)]
pub struct Machine {
    sender: UnboundedSender<Instruction>,
    base_pos: Position,
    pos: Position,
    settings: Settings,
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
        let pos: Position = Default::default();
        let base_pos = pos.clone();
        Self {
            sender,
            pos,
            base_pos,
            settings: Default::default(),
        }
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
            let action = Action::new(
                symbol.clone(),
                self.base_pos.clone(),
                self.pos.clone(),
                self.settings,
            );
            // action.run(self)
            self.transmit(symbol.instructions(PrintingDirection::Right));
            self.pos = action.new_position();
        }
    }
}
