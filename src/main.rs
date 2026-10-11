#![allow(dead_code)]

const TAX_RATE: f64 = 0.15;
const BONUS_PER_YEAR: f64 = 0.10;
#[derive(Debug)]
struct UserData {
    id: u32,
    user_name: String,
    email: String,
    salary: f64,
    year_worked: u32,
    status: EmployeeStatus,
}

#[derive(Debug)]
enum EmployeeStatus {
    NormalEmployee,
    Admin,
}

trait BonusCalculator {
    fn bonus(&self) -> f64;
}

impl UserData {
    fn new(
        id: u32,
        user_name: &str,
        email: &str,
        salary: f64,
        year_worked: u32,
    ) -> Result<Self, String> {
        if salary < 0.0 {
            return Err(String::from("Salary can't be negative"));
        }

        Ok(Self {
            id,
            user_name: user_name.to_string(),
            email: email.to_string(),
            salary,
            year_worked,
            status: EmployeeStatus::NormalEmployee,
        })
    }

    fn net_pay(&self) -> Result<f64, String> {
        let gross = self.salary + self.bonus();
        Ok(gross - tax(gross))
    }
}

impl BonusCalculator for UserData {
    fn bonus(&self) -> f64 {
        self.salary * self.year_worked as f64 * BONUS_PER_YEAR
    }
}

fn tax(amount: f64) -> f64 {
    amount * TAX_RATE
}

fn print_bonus(b: &impl BonusCalculator) {
    println!("Bonus : {:.2}", b.bonus());
}

fn run() -> Result<(), String> {
    let user = UserData::new(1, "Sarthak", "ytsarthak1@gmail.com", 50_000.0, 12)?;
    println!("User : {:?}", user);
    print_bonus(&user);
    let net = user.net_pay()?;
    println!("Amount after Tax : {:.2}", net);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error : {e}");
    }
}
