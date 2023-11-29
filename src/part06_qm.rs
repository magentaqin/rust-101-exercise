use part05_qm::BigInt;

impl BigInt {
  fn min_try1(self, other: Self) -> Self {
    debug_assert!(self.test_invariant && other.test_invariant());
    if self.data.len() < other.data.len() {
      self
    } else if self.data.len() > other.data.len() {
      other
    } else {
      self
    }
  }
}

fn vec_min(v: &Vec<BigInt>) -> Option<BigInt> {
  let mut min: Option<BigInt> = None;
  for e in v {
    let e = e.clone();
    min = Some(match min {
      None => e,
      Some(n) => e.min_try1(n)
    })
  }
  min
}

use part02_qm::{SomethingOrNothing,Something,Nothing};
impl<T: Copy> Copy for SomethingOrNothing<T> {}