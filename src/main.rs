use std::time::Duration;

use tokio::time::sleep;

trait Greet {
    fn greet(&self) -> String;
}

trait Number {
    fn multiply(&self) -> i32;
}

impl Number for i32 {
    fn multiply(&self) -> i32 {
        let number = self * self;
        return number;
    }
}

impl Greet for String {
    fn greet(&self) -> String {
        format!("Hello , {}", self)
    }
}

async fn fetch_url(id: u32, delay_sec: u64) -> String {
    println!("Starting fetch for jobs {id}");
    sleep(Duration::from_secs(delay_sec)).await;
    format!("Result from jobs {id}")
}

#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();

    let handle1 = tokio::spawn(fetch_url(1, 2));
    let handle2 = tokio::spawn(fetch_url(2, 3));
    let handle3 = tokio::spawn(fetch_url(3, 5));

    let (r1, r2, r3) = tokio::join!(handle1, handle2, handle3);
    println!("{} ", r1.unwrap());
    println!("{} ", r2.unwrap());
    println!("{} ", r3.unwrap());

    println!("Total elasped time {:.2?}", start.elapsed());

    let name = String::from("Sarthak");
    println!("{} \n", name.greet());

    let number = 12;
    println!("The number {} multiple is {}", number, number.multiply());
}
