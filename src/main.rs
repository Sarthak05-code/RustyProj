use std::io;

fn main() {
    let mut town_hall = 2;
    let mut cannon_level = 1;

    loop {
        println!(
            "\nTown Hall Level : {}\nCannon Level : {}",
            town_hall, cannon_level
        );

        println!("Enter command (town hall, cannon, town hall --11, cannon --5, quit):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match parts.as_slice() {
            // Quit
            ["q"] | ["quit"] => break,

            // Upgrade Town Hall by 1
            ["town", "hall"] => {
                if town_hall >= 11 {
                    println!("Max Town Hall level reached.");
                } else {
                    town_hall += 1;
                    println!("Town Hall upgraded to {}.", town_hall);
                }
            }

            // Set Town Hall directly
            ["town", "hall", level] => {
                if let Some(num) = level.strip_prefix("--") {
                    if let Ok(level) = num.parse::<i32>() {
                        if (1..=11).contains(&level) {
                            town_hall = level;

                            // Cannon cannot be higher than Town Hall
                            if cannon_level > town_hall {
                                cannon_level = town_hall;
                            }

                            println!("Town Hall set to {}.", town_hall);
                        } else {
                            println!("Town Hall level must be between 1 and 11.");
                        }
                    } else {
                        println!("Invalid Town Hall level.");
                    }
                } else {
                    println!("Use format: town hall --11");
                }
            }

            // Upgrade Cannon by 1
            ["cannon"] => {
                if cannon_level >= town_hall {
                    println!("Cannon can't exceed Town Hall level.");
                } else {
                    cannon_level += 1;
                    println!("Cannon upgraded to {}.", cannon_level);
                }
            }

            // Set Cannon directly
            ["cannon", level] => {
                if let Some(num) = level.strip_prefix("--") {
                    if let Ok(level) = num.parse::<i32>() {
                        if (1..=town_hall).contains(&level) {
                            cannon_level = level;

                            println!("Cannon set to {}.", cannon_level);
                        } else {
                            println!(
                                "Cannon level must be between 1 and Town Hall ({}).",
                                town_hall
                            );
                        }
                    } else {
                        println!("Invalid Cannon level.");
                    }
                } else {
                    println!("Use format: cannon --5");
                }
            }

            // Unknown command
            _ => {
                println!("Unknown command.");
            }
        }

        // Check AFTER executing the command
        if town_hall == 11 && cannon_level == 11 {
            println!("\nYou have reached Max level. Auto exiting.");
            break;
        }
    }

    println!(
        "\nFinal Levels\nTown Hall : {}\nCannon Level : {}",
        town_hall, cannon_level
    );
}
