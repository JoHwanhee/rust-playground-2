use std::fs::File;

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

    // let s = String::from("hello");
    // s.len();
    // s.push_str("aa");
    // println!("{}", s);

    let s = String::from("abcd e");
    let len = first_word(&s);

    println!("{}", len);


    // let f = read();
    //

    let num = largest(vec![1,33, 2,3, 4, 1123]);
    match num {
        Some(max) => println!("{}", max),
        None => println!("No numbers in the vector!"),
    }
}

fn func2() {
    print!("func2");
}

fn five() -> i32 {
    5
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }

    s.len()
}

fn read() -> File {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    return f
}

fn largest(nums: Vec<u32>) -> Option<u32> {
    if nums.is_empty() {
        return None;
    }

    let mut the_largest = nums[0];
    for &num in &nums[1..] {
        if num > the_largest {
            the_largest = num;
        }
    }

    Some(the_largest)
}