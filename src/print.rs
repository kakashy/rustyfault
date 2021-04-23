pub fn run() {
    //  Show hello fault as output
    println!("Print.rs sent me");

    // Basic format
    println!("{} is from {}", "Babe", 9);

    // Positional Args
    println!("This {0} is from {1} and {0} likes to {2}", "guy", "Los Hermanos", "code");

    // Named Args
    println!("{name} likes to listen to {artist}", name = "Jane", artist = "K Dot");

    // Placeholder traits
    println!("Binary: {:b} | Hex: {:x} | Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "Strwing"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}