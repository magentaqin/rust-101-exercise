// Ownership
fn work_on_vector(v: Vec<i32>) {}
pub fn ownership_demo() {
  let v = vec![1,2,3,4];
  work_on_vector(v);
  // ERR! Borrow of moved value: v
  // println!("The first element is: {}", v[0]);
}

// Borrow a shared reference
fn vec_min(v: &Vec<i32>) -> Option<i32> {
  use std::cmp;
  let mut min = None;
  for e in v.iter() {
    min = Some(match min {
      None => *e,
      Some(n) => cmp::min(n, *e)
    })
  }
  min
}

pub fn shared_ref_demo() {
  let v = vec![5,4,3,2,1];
  let first = &v[0];
  let first_min = vec_min(&v);
  let second_min = vec_min(&v);
  println!("The first element is {}", *first);
  // The first min is Some(1)
  println!("The first min is {:?}", first_min);
  // The second min is Some(1)
  println!("The second min is {:?}", second_min);
}

fn vec_increment(v: &mut Vec<i32>) {
  for e in v.iter_mut() {
    *e += 1;
  }
}


pub fn mutable_ref_demo() {
  let mut v = vec![5,4,3,2,1];
  vec_increment(&mut v);
  // After first mutation: [6, 5, 4, 3, 2]
  println!("After first mutation: {:?}", v);

  // Here, the v is based on [6, 5, 4, 3, 2]
  vec_increment(&mut v);
  // After second mutation: [7, 6, 5, 4, 3]
  println!("After second mutation: {:?}", v);
}