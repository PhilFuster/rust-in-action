pub fn add(x: i32, y: i32) -> i32 {
  x + y
}

pub fn subtract(x: i32, y:i32) -> i32 {
  x - y
}

pub fn multiply(x: i32, y:i32) -> i32 {
  x * y
}

pub fn divide(numerator: i32, denominator: i32) -> Result<i32, &'static str>{
  if denominator == 0 {
    return Err("divide by zero error.");
  }
  Ok(numerator / denominator)
}