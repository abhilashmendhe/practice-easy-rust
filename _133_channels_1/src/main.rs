use std::sync::mpsc::channel;

// 133. Channels
/*
    Channel is another concurrent type. Easier to use than Arc<Mutex<type>>
    It is multiple producer, single consumer
*/
fn main() {
    let (sender, receiver) = channel();
    sender.send(10);
    sender.send(11);
    println!("{:?}",receiver.recv());
    println!("{:?}",receiver.recv());
    println!("{:?}",receiver.try_recv());
}
