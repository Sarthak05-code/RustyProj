#![allow(non_snake_case)]

use std::{thread::sleep, time::Duration};

fn New_Battle_Engine() {
    
}

#[deprecated(note = "use the new New_Battle_Engine")]
fn Battle_Engine() {
    let mut enemies = vec!["Goblin 1", "Goblin 2", "Goblin 3"];
    let enemy_sum = enemies.len() as f64;

    while !enemies.is_empty() {
        let defeated = enemies.remove(0);

        let remaining = enemies.len() as f64 / enemy_sum * 100.0;
        println!("Defeated: {defeated} | Enemies remaining: {remaining:.1}%");

        sleep(Duration::from_millis(1000));
    }

    println!("All enemies defeated!");
}

fn main() {
    Battle_Engine();
    New_Battle_Engine();
}
