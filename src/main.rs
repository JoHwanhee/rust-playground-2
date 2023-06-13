const MAX_POINTS: u64 = 100_000;


fn main() {
    let x = 5;
    let mut x = x * 5;
    x = x * 3;
    let x = x - 10;
    println!("the value is {}", x);

    func2();


    let x = 5;

    let y = {
        let x = 3;
        10
    };

    // 이것은 주석

    println!("The value of y is: {}", y);
    println!("The value of y is: {}", five());

    if x == 1 {
        // println!('a');
    }
    else {
        println!("aa");
    }

    let c = if x == 1 {
        10
    }
    else {
        55
    };

    println!("{}", c);


    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    let s = String::from("hello");
    s.len();
    s.push_str("aa");
    println!("{}", s);
}

fn func2() {
    print!("func2");
}

fn five() -> i32 {
    5
}