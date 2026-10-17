

fn negative_value(number: i32) -> Result<String, String> {
    if number < 0 {
        return Err(String::from("The number can't be zero."));
    }

    Ok(String::from("The number is Fine"))
}

/// Treat even as [`Ok`] and treat odd as [`Err`].
/// Treats negative value will show error as the data type is u32.
fn is_even(number: u32) -> Result<String, String> {
    if number % 2 == 0 {
        return Ok(String::from("The number is even."));
    } else {
        Err(String::from("The number is odd."))
    }
}

fn runner(choice: u32, number: i32) -> Result<(), String> {
    if choice == 1 {
        let result = negative_value(number)?;
        println!("Result : {result}");
    } else if choice == 2 {
        let unsigned = u32::try_from(number).map_err(|e| e.to_string())?;
        let even_or_odd = is_even(unsigned)?;
        println!("Result : {even_or_odd}")
    } else {
        println!("Invalid Number : [{choice}]");
    }
    Ok(())
}

fn main() {
    if let Err(err) = runner(10, 10) {
        println!("Error : {err}");
    } 
}
