use iterators::dsa;
use std::{thread::sleep, time::Duration};
use traits::Sounds;

mod closure;
mod dsa;
mod helper;
mod iterators;
mod mutex;
mod scopethreads;
mod thread;
mod traits;
fn main() {
    // let pet1 = traits::Dog;
    // let pet2 = traits::Cat;
    // let pet3 = traits::Lion;
    // let a = traits::Person {
    //     first_name: "Darwin".to_string(),
    //     pet: pet2,
    // };
    // a.pet.sound();

    // closure::closure(3, 2);
    // iterators::test_iterators();
    //iterators::dsa();
    // let a = vec![1, 2, 3];
    // let b = dsa::plus_one(a);
    // println!("{:?}", b);

    // let a = dsa::single_number(vec![2, 2, 3, 3, 4, 4, 5, 5, 6]);
    // println!("{}", a);

    // let a = vec!["apple", "Mango", "Orange"];
    // let max_len = a.iter().map(|x| x.len()).max().unwrap();
    // println!("{}", max_len)
    // closure::closure()
    // thread::thread_spawn();
    // for i in 1..10 {
    //     sleep(Duration::from_millis(1));
    //     println!("{i} from main thread");
    // }

    // scopethreads::scope_thread_spawn();
    // println!("Hello from main");
    mutex::test_mutex()
}
