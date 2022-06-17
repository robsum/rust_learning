fn main() {
    println!("Hello, world!");

    another_function(5, 10);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("{}", y);

    let x = five();

    println!("{}", x);


    let x = plus_one(x);

    println!("{}", x);
}

fn another_function(x: i32, y: i32) {
    println!("Another function {} {}.", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}