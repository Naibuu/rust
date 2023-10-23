fn main() {
    let s1 = String::from("Hello world!");
    
    let word = first_word(&s1);

    println!("{}", word);

    let s2 = String::from("you are a monkey lol");

    let len = s2.len();

    let slice = &s2[0..len];

    println!("{}", slice)
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}