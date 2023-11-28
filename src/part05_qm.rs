#[derive(Debug)]
pub struct BigInt {
  pub data: Vec<u64>,
}

impl BigInt {
  pub fn new(x:u64) -> Self {
    if x == 0 {
      BigInt { data: vec![] }
    } else {
      BigInt { data: vec![x] }
    }
  }

  pub fn test_invariant(&self) -> bool {
    if self.data.len() == 0 {
      true
    } else {
      self.data[self.data.len()-1] != 0
    }
  }

  pub fn from_vec(mut v: Vec<u64>) -> Self {
    while let Some(last_digit) = v.last() {
      if *last_digit == 0 {
        v.pop();
      } else {
        break;
      }
    }
    BigInt { data: v }
  }
}

pub fn clone_demo() {
  let v = vec![0,1 << 2];
  println!("Before is {:?}", v);
  let b1 = BigInt::from_vec((&v).clone());
  println!("After is {:?}", b1);
}

impl Clone for BigInt {
  fn clone(&self) -> Self {
      BigInt { data: self.data.clone() }
  }
}

