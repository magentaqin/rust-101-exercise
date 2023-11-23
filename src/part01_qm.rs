fn sqr(i: i32) -> i32 { i * i}

fn abs(i: i32) -> i32 { 
  if (i >= 0 ) {
    i
  } else {
    -i
  }
}

enum NumberOrNothing {
  Number(i32),
  Nothing
}

use self::NumberOrNothing::{Number, Nothing};

fn number_or_default(n: NumberOrNothing, default: i32) -> i32 {
  match n {
    Nothing => default,
    Number(n) => n,
  }
}

fn compute_stuff(x: i32) -> i32 {
  // expression blocks: evaluate to the last expression they contain
  // so y is equal to z+14
  let y = { 
    let z = x * x; 
    z + 14
  };
  y*y
}

fn vec_min(v: Vec<i32>) -> NumberOrNothing {
  fn min_i32(a: i32, b: i32) -> i32 {
    if (a < b){
      a
    } else {
      b
    }
  }
  let mut min = Nothing;
  // match is moved inside
  for e in v {
    min = Number(match min {
      Nothing => e,
      Number(n) => min_i32(n, e)
    })
  }
  min
}

// Inherent Implementations. Define method `print` on the enum `NumberOrNothing`
impl NumberOrNothing {
  fn print(self) {
    match self {
      Nothing => println!("The number is: <nothing>"),
      Number(n) => println!("The number is: {}", n),
    }
  }
}

fn read_vec() -> Vec<i32> {
  vec![18,5,7,2,9,27]
}

pub fn main() {
  let vec = read_vec();
  let min = vec_min(vec);
  min.print();
}