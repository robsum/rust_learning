fn main() {
    println!("Hello, world!");

    println!("Enter which Fibonacci number to generate");

    let mut number = String::new();

    std::io::stdin().read_line(&mut number).expect("Enter number");

    let number: i32 = number.trim().parse().expect("Enter a number");

    let mut one = 0;
    let mut two = 1;
    let mut three = one + two;

    for _element in 3..number+1 {
        one = two;
        two = three;
        three = one + two;
    }

    println!("{} Fibonacci number is {}", number, three);
}
