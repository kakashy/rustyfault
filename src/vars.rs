pub fn run(){
    let name = "Kazu";
    let mut age = 45;
    
    println!("My name is {} and I'm {}.", name, age);

    age = 12;
    println!("My name is {} and I'm {}.", name, age);

    // define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // multiple vars
    let ( my_name, my_age ) = ("ZuKa", 35);
    println!("{} is {}",my_name, my_age);
}