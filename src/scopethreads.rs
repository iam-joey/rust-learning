use std::thread;
//this is how we share data from parent scope to child threads without loosing the ownership
pub fn scope_thread_spawn() {
    let a = 32;
    let s = "Hello Idiot".to_string();
    let closure = || {
        println!("inside closure");
        println!("the number is {a}");
        println!("{}", s);
    };

    thread::scope(|scope| {
        scope.spawn(closure).join();
        println!("second time");
        scope.spawn(closure).join();
        println!("third time");
        scope.spawn(closure).join();
    });
    println!("back to the parent thread");
    println!("the number is {a}");
    println!("{}", s);
}
