// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(data.clone()); // Clone data
    println!("Last character: {}", last_char);

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &String) {
    let mut owned_data = data.to_string(); // Clone data
    owned_data = owned_data.to_uppercase(); // Convert to uppercase

    println!("{}", owned_data);
}
