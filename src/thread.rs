use std::{
    thread::{sleep, spawn},
    time::Duration,
};

pub fn thread_spawn() {
    let thread_busy = || {
        for i in 0..500000 {
            println!("value of i is :{i} from spawn thread");
            sleep(Duration::from_millis(0));
        }
    };
    let a = spawn(thread_busy);
}
