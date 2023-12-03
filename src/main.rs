mod daisy;
mod gabi;
use gabi::Machine;
use gabi::*;

fn main() {
    let path = std::env::args().nth(1).unwrap();

    let mut gabi = Machine::new(&path);

    gabi.prepare();

    gabi.carriage_forward(255);
    gabi.carriage_backward(255);

    // println!("move the carriage forward->");
    // gabi.command(&[0b1100_0001, 0b1111_1111]);
    //
    // thread::sleep(Duration::from_millis(4000));
    // println!("move the carriage <-backward");
    // gabi.command(&[0b1110_0001, 0b1111_1111]);

    // thread::sleep(Duration::from_millis(4000));
    // println!("homing the carriage motor");
    // gabi.command(&[0b1000_0010, 0b0000_0011]);

    // thread::sleep(Duration::from_millis(4000));
    // println!("homing the daisy-wheel motor");
    // gabi.command(&[0b1000_0010, 0b0000_0101]);

    // thread::sleep(Duration::from_millis(4000));
    // println!("homing the tape motor");
    // gabi.command(&[0b1000_0010, 0b0000_1001]);

    // thread::sleep(Duration::from_millis(2000));

    // for _ in 0..10 {
    //     println!("space");
    //     thread::sleep(Duration::from_millis(500));
    //     gabi.command(&[0b1000_0011, 0b0000_0000]);
    // }

    // let mut cbuf = [0_u8; 2];
    // cbuf[1] = 0b1001_0110;
    //
    // for c in 50..106 {
    //     cbuf[0] = c;
    //     println!("type a character {:?}", cbuf);
    //     thread::sleep(Duration::from_millis(100));
    //     gabi.command(&cbuf);
    //     // cbuf[0] += 1;
    //     if c.wrapping_rem(5) == 0 {
    //         thread::sleep(Duration::from_millis(100));
    //         gabi.command(&[0b1000_0011, 0b0000_0000]);
    //     }
    // }

    // for _ in 0..10 {
    //     println!("back-space");
    //     thread::sleep(Duration::from_millis(500));
    //     gabi.command(&[0b1000_0100, 0b0000_0000]);
    // }

    // thread::sleep(Duration::from_millis(4000));
    // println!("roll the paper up");
    // gabi.command(&[0b1101_0000, 0b1111_1111]);
    //
    // thread::sleep(Duration::from_millis(4000));
    // println!("roll the paper down]");
    // gabi.command(&[0b1111_0000, 0b1111_1111]);

    gabi.wait_long();
    gabi.go_offline();
    gabi.wait_short();
    println!("Gabriele says Tschüss :)");
    // let _ = gabi.prepare();

    // let _ = gabi.read_status();
    // gabi.go_offline();
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
