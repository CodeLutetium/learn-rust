use std::io;

// Takes in a string slice (&str) and prints the string
fn print_string(s: &str) {
    println!("{s}");
}

// Add in sup to the FRONT of the original string without or returning a new value
fn change_string(s: &mut String) {
    *s = format!("Sup {s}");
}

fn main() {
    let mut s: String = String::new();

    // Get name from the user
    println!("Enter name: ");
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s = s.trim().to_string();
    println!("Before: ");
    print_string(&s);
    
    // Append sup with the name
    change_string(&mut s);
    println!("After: ");
    print_string(&s);
}
