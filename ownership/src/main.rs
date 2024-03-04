fn main() {
    println!("Hello, world!");

    {
        let mut s = String::from("hello");

        s.push_str(", world!");

        println!("{}", s);

        //let ss = s;
        //println!("s:{} ss:{}", s, ss);

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

    let _r1 = &mut s;
    //let r2 = &mut s;
    //let r2 = &s;
    //println!("{} {}", _r1, r2);

    println!("{}", s);

    change(&mut s);

    println!("{}", s);

    {
        let mut s = String::from("hello");

        let _r1 = &s; // no problem
        let _r2 = &s; // no problem
        //let r3 = &mut s; // BIG PROBLEM

        //println!("{}, {}, and {}", _r1, _r2, r3);

        let r3 = &mut s;
        println!("{}", r3);

        /*
            The scopes of the immutable references r1 and r2 
            end after the println! where they are last used, 
            which is before the mutable reference r3 is created. 
            These scopes don’t overlap, so this code is allowed: 
            the compiler can tell that the reference is no longer 
            being used at a point before the end of the scope.
        */
    }

    // let _reference_to_nothing = dangle();

    let s = no_dangle();
    println!("{}", s);

    let word = first_word(&String::from("ala ma kota"));
    
    println!("{}", word);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    let word = first_word_slice(&s);

    println!("{}", word);

    let my_string = String::from("hello world");

    let _word = first_word_slice(&my_string[0..6]);
    println!("{_word}");

    let _word = first_word_slice(&my_string[..]);
    println!("{_word}");

    let _word = first_word_slice(&my_string);
    println!("{_word}");

    let my_string_literal = "hello world";
    let _word = first_word_slice(my_string_literal);
    println!("{_word}");

    let _word = first_word_slice(&my_string_literal[6..]);
    println!("{_word}");


    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

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

// fn dangle() -> &'static String {
//     let s = String::from("hello");
//
//     &s
// }

fn no_dangle() -> String {
    String::from("hello")
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