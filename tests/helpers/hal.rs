use gabriele::hal::Hal;
use gabriele::printing::Instruction;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;
use virtual_serialport::VirtualPort;

pub fn start_test_hal() -> (UnboundedSender<Instruction>, JoinHandle<()>, VirtualPort) {
    let (tx, rx) = mpsc::unbounded_channel::<Instruction>();
    let (port_use, port) = VirtualPort::pair(9600, 256).unwrap();

    let mut hal = Hal::test_new(rx, port_use.into_boxed());
    let handle = tokio::task::spawn_blocking(move || hal.run());

    (tx, handle, port)
}
