use oop::AveragedCollection;


fn main() {
    println!("Hello, world!");

    let mut collection: AveragedCollection = AveragedCollection::new();
    collection.add(3);
    println!("Average: {}", collection.average());
}
