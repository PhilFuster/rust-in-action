pub mod input;
pub mod math;
use std::io::{self, Write};
use input::get_input;
use math::{add};
fn main() {
  fn prompt_user(prompt: &str) {
    print!("{}",prompt.to_string());
    io::stdout().flush().unwrap();

  }
  println!("Welcome to Philip's simple Rust Calculator!");
  // main loop
  let mut op1;
  let mut op2;
  loop {
    // menu prompt
    prompt_user("quit<q>, calculate<c>\n");
    let input = get_input().unwrap();
    match input.as_str() {
      "q"|"Q"|"quit" => {
        println!("Philip's Rust Calculator exited.");
        std::process::exit(0)
      },
      "c"|"C"|"calculate" => (),
      _ => {
        print!("Invalid input {}", input);
        continue;
      },
    }
    loop {
      prompt_user("Operand 1: ");
      let operand1_input = get_input().unwrap();
      let res = operand1_input.parse::<i32>();
      match res {
        Ok(operand) => {
          op1 = operand;
          break;
        },
        Err(e) => {
          eprintln!("Error: {}", e);
          continue;
        },
      }
    }
    loop {
      prompt_user("Operand 2: ");
      let operand2_input = get_input().unwrap();
      let res = operand2_input.parse::<i32>();
      match res {
        Ok(operator) => {
          op2 = operator;
          break;
        },
        Err(e) => {
          eprintln!("Error: {}", e);
          continue;
        },
      }
    }
    loop {
      prompt_user("Operator (+, -, *, /)");
      let operator_input = get_input().unwrap();
      match operator_input.as_str() {
        "+" => println!("{} + {} = {}", op1, op2, add(op1 , op2)),
        "-" => println!("{} - {} = {}", op1, op2, math::subtract(op1 , op2)),
        "*" => println!("{} * {} = {}", op1, op2, math::multiply(op1 , op2)),
        "/" => println!("{} / {} = {}", op1, op2, math::divide(op1 , op2).unwrap()),
        _ => {
          eprintln!("invalid operator entered: {}", operator_input);
          continue;
        },
      }
     break;
    }  
  }

}
