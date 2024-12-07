fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec.get(0);
    let second = vec.get(1);
    println!("First: {:?}, Second: {:?}", first, second);
    //This is safe because we checked the index before dereferencing.
    println!("First: {}, Second: {}", first.unwrap(), second.unwrap());
    //This will panic at runtime if the vector has no element at index 0.
    //println!("First: {}", vec[0]);
}