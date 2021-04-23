pub fn run(){
    // "i32"
    let _x = 1;

    // "f64"
    let _y = 2.5;

    // explicit
    let _z: i64 = 8585858585852;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // boolean
    let is_active: bool = true;

    // boolean from expression
    let is_greater: bool = 10 < 5;

    // character
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (_x, _y, _z, is_active, is_greater, a1, face));
}