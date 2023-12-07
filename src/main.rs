mod daisy;
mod gabi;
use crate::daisy::{german, Db};
use crate::gabi::commando::Commands;
use gabi::machine::Machine;

fn main() {
    let path = std::env::args().nth(1).unwrap();

    let db = Db::new(german::symbols());
    let mut machine = Machine::new(&path);

    machine.prepare();

    // machine.carriage_forward(255);
    //
    // println!("homing the daisy-wheel motor");
    // machine.command(&[0b1000_0010, 0b0000_0101]);
    // machine.wait_long();
    //
    // println!("homing the carriage motor");
    // machine.command(&[0b1000_0010, 0b0000_0011]);
    // machine.wait_long();
    //
    // println!("homing the tape motor");
    // machine.command(&[0b1000_0010, 0b0000_1001]);
    // machine.wait_long();
    //
    // machine.carriage_forward(255);
    //
    machine.print("Grüß Gott", &db);
    //
    // machine.carriage_backward(255);

    // println!("move the carriage forward->");
    // machine.command(&[0b1100_0001, 0b1111_1111]);
    //
    // thread::sleep(Duration::from_millis(4000));
    // println!("move the carriage <-backward");
    // machine.command(&[0b1110_0001, 0b1111_1111]);

    // thread::sleep(Duration::from_millis(4000));
    // println!("homing the carriage motor");
    // machine.command(&[0b1000_0010, 0b0000_0011]);

    // thread::sleep(Duration::from_millis(4000));
    // println!("homing the daisy-wheel motor");
    // machine.command(&[0b1000_0010, 0b0000_0101]);

    // thread::sleep(Duration::from_millis(4000));
    // println!("homing the tape motor");
    // machine.command(&[0b1000_0010, 0b0000_1001]);

    // thread::sleep(Duration::from_millis(2000));

    // for _ in 0..10 {
    //     println!("space");
    //     thread::sleep(Duration::from_millis(500));
    //     machine.command(&[0b1000_0011, 0b0000_0000]);
    // }

    // let mut cbuf = [0_u8; 2];
    // cbuf[1] = 0b1001_0110;
    //
    // for c in 50..101 {
    //     cbuf[0] = c;
    //     println!("type a character {:?}", cbuf);
    //     machine.wait_short();
    //     machine.command(&cbuf);
    //     if c.wrapping_rem(5) == 0 {
    //         machine.wait_short();
    //         machine.command(&[0b1000_0011, 0b0000_0000]);
    //     }
    // }

    // for _ in 0..10 {
    //     println!("back-space");
    //     thread::sleep(Duration::from_millis(500));
    //     machine.command(&[0b1000_0100, 0b0000_0000]);
    // }

    // thread::sleep(Duration::from_millis(4000));
    // println!("roll the paper up");
    // machine.command(&[0b1101_0000, 0b1111_1111]);
    //
    // thread::sleep(Duration::from_millis(4000));
    // println!("roll the paper down]");
    // machine.command(&[0b1111_0000, 0b1111_1111]);

    machine.wait_long();
    machine.go_offline();
    machine.wait_short();
    println!("Gabriele says Tschüss :)");
    // let _ = machine.prepare();

    // let _ = machine.read_status();
    // machine.go_offline();
    // thread::sleep(Duration::from_millis(1000));
    // println!("finish");

    // thread::sleep(Duration::from_millis(1000));
    // println!("post-mortem tasks");
    // let post_mortem = Machine::connect(&path);
    // thread::sleep(Duration::from_millis(1000));
    // // let _ = post_mortem.prepare();
    // thread::sleep(Duration::from_millis(1000));
    // drop(post_mortem);
    // println!("post-mortem tasks finished");
}
