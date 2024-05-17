pub fn closure() {
    let a = 32;
    let b = 42;

    let add = || -> i32 {
        return a + b;
    };
    let result = add();

    println!("{}", result)
}
