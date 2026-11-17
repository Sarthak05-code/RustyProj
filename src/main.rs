use std::{fs::OpenOptions, io::Read};

fn main() -> std::io::Result<()> {
    let mut sql_file = OpenOptions::new().read(true).open("sql.txt")?;
    let mut readable_content = String::new();
    sql_file.read_to_string(&mut readable_content)?;

    // Convert everything to lowercase to make the check case-insensitive
    let content_lower = readable_content.to_lowercase();

    // 1. Find the position (index) of the keywords
    let create_pos = content_lower.find("create table");
    let select_pos = content_lower.find("select");

    // 2. Run the validation logic
    match (create_pos, select_pos) {
        (None, _) => {
            println!("Failure: No 'CREATE TABLE' statement found in the script.");
        }
        (Some(_), None) => {
            println!("Success: Table created (no queries executed yet).");
        }
        (Some(create_idx), Some(select_idx)) => {
            if create_idx > select_idx {
                println!("Failure: You are trying to SELECT before creating the table!");
            } else {
                println!("Success: SQL order is valid. Table created before querying.");
            }
        }
    }

    Ok(())
}
