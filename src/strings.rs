pub fn run(){
    let mut hello = String::from("Hello ");

    // push char
    hello.push('W');

    // push a string
    hello.push_str("orld");

    // capacity
    println!("Capacity: {} bytes", hello.capacity());

    // replace
    println!("Replacing: {}", hello.replace("World", "Darkness"));

    //  contains
println!("Contains 'World': {}", hello.contains("World"));

    println!("{}", hello)
}