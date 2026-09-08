fn main() {
    let name = "aaabbccccc"; // NOTE : we want this := a3b2c5

    let name_bytes: Vec<char> = name.chars().collect();

    let mut new_name = String::new();
    let mut count = 1;

    for i in 0..name_bytes.len() {
        if i + 1 < name_bytes.len() && name_bytes[i] == name_bytes[i + 1] {
            count += 1;
        } else {
            new_name.push(name_bytes[i]);
            new_name.push_str(&count.to_string());
            count = 1;
        }
    }
    println!("The new name : {}", new_name);
}
