fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec.get(0);
    let second = vec.get(1);
    match first {
        Some(value) => println!("First: {}", value),
        None => println!("First element not found"),
    }
    match second {
        Some(value) => println!("Second: {}", value),
        None => println!("Second element not found"),
    }
    //or using if let
    if let Some(value) = vec.get(0){
        println!("First: {}",value);
    } else{
        println!("First element not found");
    }
    if let Some(value) = vec.get(1){
        println!("Second: {}", value);
    }else{
        println!("Second element not found");
    }
}    
