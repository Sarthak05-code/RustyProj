enum Opcode {
    ADD,  // addition
    SUB,  // subtraction
    ADA,  // add to a register
    JMP,  // jump (unconditional)
    JC,   // jump on carry
    HALT, // program end
    MOV,  // move data between variables and addresses.
}

fn parser(token: &str) -> Option<Opcode> {
    match token.to_uppercase().as_str() {
        "ADD" => Some(Opcode::ADD),
        "SUB" => Some(Opcode::SUB),
        "ADA" => Some(Opcode::ADA),
        "JMP" => Some(Opcode::JMP),
        "JC" => Some(Opcode::JC),
        "MOV" => Some(Opcode::MOV),
        "HALT" => Some(Opcode::HALT),
        _ => None,
    }
}

fn main() {
    let text = r#"
        mov A, 10;
        mov B,A
        HALT;
        "#;

    let texts: Vec<&str> = text
        .split(|c: char| c.is_whitespace() || c == ',' || c == ';')
        .filter(|s| !s.is_empty())
        .collect();

    for token in texts {
        if let Some(opcode) = parser(token) {
            match opcode {
                Opcode::MOV => println!("Move instruction. "),
                Opcode::ADD => println!("Addition instruction. "),
                Opcode::ADA => println!("Addtion to Register A instruction. "),
                Opcode::JMP => println!("Unconditional Jump instruction. "),
                Opcode::JC => println!("Jump on carry instruction. "),
                Opcode::HALT => println!("Program stop execution. "),
                Opcode::SUB => println!("Subtract instruction. "),
            }
        } else {
            println!("Operand : {}", token);
        }
    }
}
