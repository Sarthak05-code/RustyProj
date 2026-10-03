mod extract;
unsafe extern "C" {}

unsafe fn caller(number: u32) -> u32 {
    number << 10
}

trait Speaker {
    fn speak(&self) {}
}

struct Dog;
impl Speaker for Dog {
    fn speak(&self) {
        println!("The dog is speaking");
    }
}

struct Cat;
impl Speaker for Cat {
    fn speak(&self) {
        println!("The cat is speaking");
    }
}

fn main() {
    let number = unsafe { caller(10) };

    println!("The number is {}", number);

    let speakers: Vec<Box<dyn Speaker>> = vec![Box::new(Dog), Box::new(Cat)];
    for speaker in speakers {
        speaker.speak();
    }

    extract::name_caller("Sarthak");
}
