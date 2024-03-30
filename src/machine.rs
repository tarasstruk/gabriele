use crate::database::Db;
use crate::position::Position;
use crate::printing::{Action, Instruction};
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

    pub fn transmit(&mut self, instructions: impl Iterator<Item = Instruction>) {
        for item in instructions {
            self.sender
                .send(item)
                .expect("the communication channel is closed");
        }
        // for cmd in instructions {
        //     match cmd {
        //         Instruction::SendBytes(bytes) => self.send_bytes(&bytes),
        //         Instruction::Idle(millis) => self.idle(millis),
        //         Instruction::Empty => continue,
        //     }
        // }
    }

    // pub fn set_printing_direction(&mut self, dir: PrintingDirection) {
    //     self.settings.direction = dir;
    //     match self.settings.direction {
    //         PrintingDirection::Left => {
    //             // calculate the new base position
    //             let pos = self.base_pos.align_right();
    //             // update the base position
    //             self.base_pos = pos.clone();
    //             // move from the current place to the new base position
    //             let instructions = motion::move_absolute(self.pos.clone(), pos);
    //             self.execute_instructions(instructions);
    //             wait_long();
    //             // update the current position
    //             self.pos = self.base_pos.clone();
    //             info!("Text align: Right");
    //         }
    //         _ => info!("Text align: Left"),
    //     }
    // }

    // pub fn transmit(&mut self, input: u8) {
    //     self.sender.send(input)
    //         .expect("the communication channel is closed");
    // }

    //
    // pub fn await_acknowledge(&mut self) {
    //     self.command(&[0xA4, 0x00]);
    //     for _ in 0..10 {
    //         wait_short();
    //         let mut buf = [0_u8];
    //         if let Ok(n) = self.conn.read(&mut buf) {
    //             debug!("received byte {:?}", &buf[0]);
    //             if n == 1 && buf[0] == 161_u8 {
    //                 return;
    //             }
    //             if n == 1 && buf[0] == 160_u8 {
    //                 panic!("unexpected status code is received");
    //             }
    //         }
    //     }
    //     panic!("no answer is received from the machine");
    // }
    //
    // pub fn command(&mut self, bytes: &[u8]) {
    //     for byte in bytes {
    //         self.write_byte(*byte);
    //     }
    // }
    //
    // pub fn prepare(&mut self) {
    //     wait_long();
    //     info!("stopping accepting printing commands");
    //     self.command(&[0xA3, 0x00]);
    //
    //     wait_long();
    //     info!("going off-line");
    //     self.command(&[0xA0, 0x00]);
    //
    //     wait_long();
    //     info!("going first on-line");
    //     self.command(&[0xA1, 0x00]);
    //
    //     wait_long();
    //     info!("reading the status from machine");
    //     self.await_acknowledge();
    //
    //     wait_long();
    //     info!("preparing the machine for printing");
    //     self.command(&[0xA2, 0x00]);
    //
    //     info!("machine is now accepting the printing commands");
    //     wait_long();
    // }

    // pub fn go_offline(&mut self) {
    //     self.transmit([Instruction::wait_long()].iter());
    //     info!("stopping accepting printing commands");
    //     self.transmit([Instruction::SendBytes([0xA3, 0x00])].iter());
    //     info!("going off-line");
    //     self.transmit([Instruction::SendBytes([0xA0, 0x00])].iter());
    // }

    // pub fn wait_long(&self) {
    //     wait(1000);
    // }
    //
    // pub fn wait_short(&self) {
    //     wait(200);
    // }
    //
    // pub fn wait_tiny(&self) {
    //     wait(50);
    // }
    //
    // pub fn wait(&self, millis: u64) {
    //     thread::sleep(Duration::from_millis(millis));
    // }

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

    // pub fn print_line(&mut self, input: &str, db: &Db) {
    //     for symbol in db.printables(input) {
    //         let action = Action::new(
    //             symbol.clone(),
    //             self.base_pos.clone(),
    //             self.pos.clone(),
    //             self.settings,
    //         );
    //         action.run(self)
    //     }
    //     let new_pos = self.pos.newline();
    //     let instructions = motion::move_absolute(self.pos.clone(), new_pos.clone());
    //     self.execute_instructions(instructions);
    //     self.pos = new_pos;
    //     wait_short();
    // }
    //
    // pub async fn print_by_directional(&mut self, input: &str, db: &Db) {
    //     let lines = input.lines();
    //     for (line_num, line) in lines.enumerate() {
    //         let line = line.trim_end();
    //         if (line_num % 2) == 1 {
    //             let new_pos = self.pos.align_to_string_length(line.len() as i32);
    //             let instructions = motion::move_absolute(self.pos.clone(), new_pos.clone());
    //             self.execute_instructions(instructions);
    //             self.pos = new_pos;
    //             wait_short().await;
    //             self.settings.direction = PrintingDirection::Left;
    //             let rev_line = line.chars().rev().collect::<String>();
    //             self.print_line(&rev_line, db);
    //         } else {
    //             self.settings.direction = PrintingDirection::Right;
    //             self.print_line(line, db);
    //         }
    //     }
    // }
}

// pub trait InstructionRunner {
//     async fn send_bytes(&mut self, bytes: &[u8]);
//
//     fn update_position(&mut self, pos: Position);
//
//     // fn idle(&self, millis: u64) {
//     //     wait(millis).await
//     // }
// }
//
// impl InstructionRunner for Machine {
//     fn send_bytes(&mut self, bytes: &[u8]) {
//         for byte in bytes {
//             self.transmit(*byte);
//         }
//     }
//
//     fn update_position(&mut self, pos: Position) {
//         self.pos = pos;
//     }
// }
