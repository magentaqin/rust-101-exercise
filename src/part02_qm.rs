pub enum SomethingOrNothing<T> {
  Something(T),
  Nothing,
}

// instead of writing out all the variants, we just import them all at once.
pub use self::SomethingOrNothing::*;

// define type and implement
type NumberOrNothing = SomethingOrNothing<i32>;
impl NumberOrNothing {
  pub fn print(self) {
    match self {
      Nothing => println!("The number is: <nothing>"),
      Something(n) => println!("The number is: {}", n)
    }
  }
}


// Generic `impl`, Static functions
// Inside an `impl`, Self refers to `SomethingOrNothing<T>`
impl<T> SomethingOrNothing<T> {
  // static constructor
  fn new(o: Option<T>) -> Self {
    match o {
      None => Nothing,
      Some(t) => Something(t)
    }
  }

  fn to_option(self) -> Option<T> {
    match self {
      Nothing => None,
      Something(t) => Some(t)
    }
  }
}

// call static function 
fn call_constructor(x: i32) -> SomethingOrNothing<i32> {
  SomethingOrNothing::new(Some(x))
}

// define trait
pub trait Minimum : Copy {
  fn min(self, b: Self) -> Self;
}

// implement trait to make vec_min usable with a Vec<i32>
// here, self refered to the Minimum type(the type we are implementing for)
impl Minimum for i32 {
  fn min(self, b: Self) -> Self {
    if self < b {
      self
    } else {
      b
    }
  }
}

// In other programming langauges, we only need T
// But in rust, we could also implement Minimum trait for T.
pub fn vec_min<T: Minimum>(v: Vec<T>) -> SomethingOrNothing<T> {
  let mut min = Nothing;
  for e in v {
    min = Something(match min {
      Nothing => e,
      Something(n) => {
        e.min(n)
      }
    })
  }
  min
}


fn read_vec() -> Vec<i32> {
  vec![18,5,7,3,9,27]
}

pub fn main() {
  let vec = read_vec();
  let min = vec_min(vec);
  min.print();
}