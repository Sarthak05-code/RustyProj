package main

import (
	"errors"
	"fmt"
)

const (
	bonusRatePerYear = 0.10 // 10% of salary per year worked
	taxRate          = 0.15 // 15% for everyone
)

type Employee struct {
	ID      int
	Name    string
	Email   string
	IsAdmin bool
	Salary  float64
}

// NewEmployee validates input and returns a ready-to-use value.
func NewEmployee(id int, name, email string, salary float64) (*Employee, error) {
	if salary < 0 {
		return nil, errors.New("salary cannot be negative")
	}
	return &Employee{ID: id, Name: name, Email: email, Salary: salary}, nil
}

// Small interface, named by behavior, defined where it is used.
type BonusCalculator interface {
	Bonus(years int) float64
}

// Value receiver: this method doesn't modify the Employee.
func (e Employee) Bonus(years int) float64 {
	return e.Salary * float64(years) * bonusRatePerYear
}

// Plain function: tax doesn't depend on any Employee state.
func tax(amount float64) float64 {
	return amount * taxRate
}

// NetPay returns an error instead of silently accepting bad input.
func (e Employee) NetPay(years int) (float64, error) {
	if years < 0 {
		return 0, fmt.Errorf("years worked cannot be negative: %d", years)
	}
	gross := e.Salary + e.Bonus(years)
	return gross - tax(gross), nil
}

// Accepts the interface, so any type with a Bonus method works here.
func printBonus(b BonusCalculator, years int) {
	fmt.Printf("Bonus for %d years: %.2f\n", years, b.Bonus(years))
}

func main() {
	emp, err := NewEmployee(1, "Sarthak", "ytsarthak1@gmail.com", 50_000)
	if err != nil {
		fmt.Println("error:", err)
		return
	}

	role := "Employee"
	if emp.IsAdmin {
		role = "Admin"
	}
	fmt.Printf("Hello, %s: %s. Your salary is %.2f\n", role, emp.Name, emp.Salary)

	const years = 5
	printBonus(emp, years)

	net, err := emp.NetPay(years)
	if err != nil {
		fmt.Println("error:", err)
		return
	}
	fmt.Printf("Your salary after bonus and tax is: %.2f\n", net)
}
