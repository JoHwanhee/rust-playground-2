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
}

fn func2() {
    print!("func2");
}

fn five() -> i32 {
    5
}