enum NumberOrNothing {
  Number(i32),
  Nothing
}

// take NumberOrNothing into the local namespace, and remove all `NumberOrNothing::`
use self::NumberOrNothing::{Number, Nothing};

// iterate the list and return the min element
fn vec_min(vec: Vec<i32>) -> NumberOrNothing {
  let mut min = Nothing;
  // iterate
  for el in vec {
    match min {
      // case 1: initial value
      Nothing => {
        min = Number(el);
      }
      // case 2: it is a i32 Number
      Number(n) => {
        // Now, min is the current number n
        // compare n and el
        let new_min = min_i32(n, el);
        min = Number(new_min);
      }
    }
  }
  return min;
}

fn min_i32(a: i32, b: i32) -> i32 {
  if (a < b) {
    return a;
  } else {
    return b;
  }
}


fn read_vec() -> Vec<i32> {
  vec![18,5,7,1,9,27]
}

fn print_number_or_nothing(n: NumberOrNothing) {
  match n {
      Nothing => println!("The number is: <nothing>"),
      Number(n) => println!("The number is: {}", n),
  };
}

pub fn main() {
  let vec = read_vec();
  let min = vec_min(vec);
  print_number_or_nothing(min);
}
