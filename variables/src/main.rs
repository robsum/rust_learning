const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("The value of x is: {}", x);

    //x = 6;
    //println!("The value of x is: {}", x);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is {}", THREE_HOURS_IN_SECONDS);

    //let THREE_HOURS_IN_SECONDS = 20;
    //println!("Now its {}", THREE_HOURS_IN_SECONDS);

    let x = x + 1;
    println!("The value of x is: {}", x);

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    println!("Max points is {}", MAX_POINTS);

    let spaces = "    ";
    let spaces = spaces.len();

    println!("Spaces is {}", spaces);

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    println!("x = {}, y = {}", x, y);
    
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    println!("{} {} {} {} {}", sum, difference, product, quotient, remainder);

    let f: bool = false;

    let t = true;

    println!("{} {}", t, f);

    let c: char = 'z';

    let z = 'ź';
    let heart_eyed_cat = '😻';

    println!("{} {} {}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("x = {}, y = {}, z = {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{} {} {}", five_hundred, six_point_four, one);

    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];

    let element = a[4];

    println!("{}", element);

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    println!("First month: {}", months[0]);

    another_function(5);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");

    println!("The return value of another_function(1) is {}", another_function(1));

    let number = 3;
    if number < 5 {
        println!("The condition was true");
    } else {
        println!("The condition was false");
    }

    //let number = if true { 5 } else { "six" };
    //println("Number is {number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32) -> i32 {
    println!("Another function {}.", x);
    x
}