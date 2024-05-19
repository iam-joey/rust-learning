use std::{
    sync::mpsc,
    thread::{self, sleep},
    time::Duration,
};

pub fn test_thread() {
    let (sender, reciver) = mpsc::channel::<i32>();

    let listen = move || {
        println!("Trying to listen");
        let mut a = 0;
        loop {
            let listner = reciver.recv_timeout(Duration::from_millis(1000));

            if listner.is_ok() {
                println!("recived a message {}", listner.unwrap());
            } else {
                a += 1;
            }
            if a > 10 {
                println!("NOthing to listen braking the loop");
                break;
            }
        }
    };

    let t1 = thread::spawn(listen);

    for i in 1..10 {
        sender.send(i + 1);
        sleep(Duration::from_millis(2000));
    }

    t1.join();
}
