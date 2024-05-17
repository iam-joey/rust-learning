pub fn test_iterators() {
    let fruits = vec!["Mango", "Bananna", "Apple", "Orange"];
    let vegetables = vec!["Cabbage", "Beetroot", "Brinjal"];
    let mut fru = fruits.iter();
    fru.next();
    fru.next();

    let fruit = fruits.iter().chain(vegetables.iter());

    fruit.for_each(|x| println!("{}", x));

    let changed = fruits.iter().map(|e| String::from(*e));
    let new = changed.map(|mut e| {
        e.push_str(" fruit");
        return e;
    });

    new.for_each(|e| println!("{}", e));

    let fruit = fruits.iter();

    let first_names = vec!["Joey", "Chandler", "Ross"];

    first_names.iter().skip(3).for_each(|e| println!("{}", e));

    let nums = vec![1, 2, 3];

    let nums2: Vec<_> = nums.iter().map(|a| a + 1).collect();
    println!("{:?}", nums2)
}

pub fn dsa() {
    let nums = vec![1, 2, 3];
    let n = nums.len();

    for i in (0..n).rev() {
        println!("{}", nums[i])
    }

    let a = vec![0; 3];

    a.iter().for_each(|x| println!("{}", x))
}
