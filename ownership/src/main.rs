fn main() {
    println!("Hello, world!");

    {
        let mut s = String::from("hello");

        s.push_str(", world!");

        println!("{}", s);

        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);

        let x = 5;
        let y = x;

        println!("x = {}, y = {}", x, y);
    }

    let s = String::from("hello");

    takes_ownership(s);

    //println!("{}", s);

    let x = 5;

    makes_copy(x);

    println!("{}", x);

    let _s1 = gives_ownership();

    let s2 = String::from("hello");

    let _s3 = takes_and_gives_back(s2);

    //println!("{}", s2);

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let s1 = String::from("hello");

    let len = calculate_length_reference(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    println!("{}", s);

    change(&mut s);

    println!("{}", s);

    let word = first_word(&String::from("ala ma kota"));
    
    println!("{}", word);

    let mut s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    let word = first_word_slice(&s);

    println!("{}", word);


}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_reference(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}