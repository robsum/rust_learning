const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("Hello, world!");
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;

    let x = x * 2;

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

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];

    let element = a[4];

    println!("{}", element);

    another_function();
}

fn another_function() {
    println!("Another function.");
}