fn main() {
    let twenty = 20;
    let twentyone =21i32;
    let twentytwo = 22_i32;
    
    let addition = twenty + twentyone + twentytwo;

    println!("Addition resul {}", addition);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];
    println!("{:03}", forty_twos[0]);

    // wieird stuff
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;

    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);

    // comparisons

    let a: i32 = 10;
    let b: u16 = 100;
    if a < (b as i32) { // It is safest to cast the smaller type to a larger one
    println!("Ten is less than one hundred.");
}
}
