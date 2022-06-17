#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    println!("Hello, world!");

    let _v: Vec<i32> = Vec::new();

    let _v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let v = vec![1, 2, 3, 4];

        let third = &v[2];
        println!("The thirds element is {}", third);

        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is not third element"),
        }
    }

    let v = vec![1, 2, 3, 4, 5];

    //let does_not_exist = &v[100];
    let _does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6); // no way since it is immutable right now

    println!("The first element is: {}", first);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

    let data = "initial contents";

    let _s = data.to_string();

    let _s = "initial contents".to_string();

    let _s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3);

    let s1 = String::from("Hello");
    //let h = s1[0];

    let len = String::from("Hola").len();

    let len = String::from("Здравствуйте").len();
    println!("Здравствуйте: {} len", len);

    let hello = "Здравствуйте";
    //let answer = &hello[0];
    let s = &hello[0..4];

    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // scores.insert(1, 20); // no way

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{} {}", field_name, field_value); // value moved

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let mut integers = vec![1, 2, 3, 2];

    let mut sum = 0;
    integers.sort();
    let len: i32 = integers.len().try_into().unwrap();
    let mut i = 0;
    let mut middle: i32;
    let mut median: i32 = 0;
    middle = len / 2;
    let mut map: HashMap<i32, i32> = HashMap::new();
    for elem in integers {
        i += 1;
        sum += elem;
        if i == middle {
            median = elem;
        }
        let count = map.entry(elem).or_insert(0);
        *count += 1;
    }
    let average = sum / len;
    let i = 0;
    let mut mode: i32 = 0;
    for (key, value) in &map {
        if i == 0 {
            mode = *key;
        }
        if value > map.get(&mode).unwrap() {
            mode = *key;
        }
    }

    println!("Average: {}, median = {}, mode = {}", average, median, mode);
    
    // tutaj coś nie działa i nie wiem czemu:
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut string = String::new();
    loop {
        
        println!("Enter string as 'Add Amir to Sales'");
        

        use std::io;

        io::stdin().read_line(&mut string)
        .expect("Failed to read line");

        // println!("{}", string);

        let mut strings: Vec<&mut str> = string.split_whitespace().collect();

        
        let entry = map.entry(strings[3]).or_insert(vec![]);
        entry.push(strings[1]);
    }


}
