fn main() {
    let stored_values = ["Sarthak", "Thapa", "is", "my", "name", "."];

    for text in &stored_values {
        if text.len() != 1 {
            println!("Valid text -> {text}")
        } else {
            println!("Invalid text -> {text}")
        }
    }

    println!("{:?}", stored_values);
}
