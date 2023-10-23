fn main() {
    let z: u32 = multiply(5, 2);

    println!("{z}")
}

fn multiply(x: u32, y: u32) -> u32 {
    if x * y < 10 {
        return 0;
    } else {
        return x * y;
    }
}
