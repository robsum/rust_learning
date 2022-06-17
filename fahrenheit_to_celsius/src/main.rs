use std::io;

fn main() {
    println!("Hello, world!");

    println!("Enter Fahrenheit temperature: ");

    let mut fahrenheit = String::new();

    io::stdin().read_line(&mut fahrenheit).expect("Enter a number");

    let fahrenheit: f32 = fahrenheit.trim().parse().expect("Enter a number");

    println!("Fahrenheit {}", fahrenheit);

    let celsius = (fahrenheit - 32.0) * (5.0/9.0);

    println!("It's {} Celsius!", celsius);
}
