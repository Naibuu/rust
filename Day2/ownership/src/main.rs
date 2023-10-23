fn main() {
    // variable scope (crazy)
    {
        let s1 = String::from("fgiopjdfigojdfiomgiodfmgiodfmgiodfiogm");

        let (s2, len) = calculate_length(s1);

        println!("the length of '{}' is {}", s2, len)
    }

    let mut s = some_string();

    s.push_str(" hanging low (real)");

    albanian(s);

    let x = 20;

    copy(x)

    // see now s wont be valid since an albanian stole ownership
    //  println!("{s}")
}

fn calculate_length(s: String) -> (String, usize) {
   let length = s.len();

   (s, length)
}
fn some_string() -> String {
   let some_string = String::from("balls");
   // this will return some string
   some_string
}

fn albanian(int: String) {
    println!("{}", int)
}

fn copy(int: u32) {
    println!("{}", int)
}
