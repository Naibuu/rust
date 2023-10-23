fn main() {
    let s1 = String::from("Balls.");

    // & basically calls reference as we saw in day 1
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("Hello");
    change(&mut s2);

    println!("{s2}");

    let s3 = &s2;
    let s4 = &s3;
    // yeah so u cant use &mut in this case ^^^ only if u open a new scope

    // whatever the fuck this is 
    println!("{}, {}", s3, s4)
}

// s is a reference to string
fn calculate_length(s: &String) -> usize {
    s.len()
}
// s goes out of scope here sooooo no ownership = ur bad

fn change(s: &mut String) {
    s.push_str(", world!")
}
