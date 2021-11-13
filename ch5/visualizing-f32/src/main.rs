fn main() {
  let n: f32 = 42.42;
  let n_bits: u32 = n.to_bits();
  let _sign_bit = n_bits >> 31;
  
  let exponent_ = n_bits >> 23;
  println!("exponent in bits after right shift 23 -> {}", exponent_);
  let exponent_ = exponent_ & 0xff;
  println!("exponent in bits after AND mask-> {}", exponent_);
  let exponent = (exponent_ as i32) - 127;
  println!("exponent after subtracting bias of 127-> {}", exponent);
  
  
  let mut mantissa: f32 = 1.0;
  for i in 0..23 {
    let mask = 1 << i;
    let one_at_bit_i = n_bits & mask;
    if one_at_bit_i != 0 {
      let i_ = i as f32;
      let weight = 2_f32.powf( i_ - 23.0 );
      mantissa += weight;
    }
  }
  println!("mantissa {}", mantissa);
}
