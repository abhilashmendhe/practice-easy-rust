// 134. Channels 2
use std::{sync::mpsc::channel, thread};

fn main() {
    let (send, recv) = channel();
    send.send(10);

    let sender2 = send.clone();
    thread::spawn(move || {
        send.send(32);
    });
    thread::spawn(move || {
        sender2.send(32);
    });

    println!("{:?}",recv.recv());
    println!("{:?}",recv.recv());
    println!("{:?}",recv.try_recv());
    println!("{:?}",recv.try_recv());
}
