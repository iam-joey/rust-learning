use std::{
    sync::Mutex,
    thread::{self, sleep},
    time::Duration,
};

pub fn test_mutex() {
    let counter = Mutex::new(10);

    let fun1 = || loop {
        println!("Trying to  mutex lock thread 1");
        let guard = counter.try_lock();
        if guard.is_ok() {
            println!("thread 1 go locked bulls eye");
            let mut data = guard.unwrap();
            for i in 0..10 {
                *data += i;
                println!("thread 1 in working data is {}", data);
                sleep(Duration::from_millis(2000));
            }
            println!("thread 1 closed");
            break;
        }

        sleep(Duration::from_millis(5000));
    };

    let fun2 = || {
        println!("Trying to  mutex lock thread 2");
        let mut data = counter.lock().unwrap();
        for i in 0..10 {
            *data += i;
            println!("thread 2 in working data is {}", data);
            sleep(Duration::from_millis(2000));
        }
        println!("thread 2 closed");
    };

    thread::scope(|scope| {
        scope.spawn(fun2);
        scope.spawn(fun1);
    });
}
