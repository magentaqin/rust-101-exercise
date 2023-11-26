fn work_on_vector(v: Vec<i32>) {}
pub fn ownership_demo() {
  let v = vec![1,2,3,4];
  work_on_vector(v);
  // ERR! Borrow of moved value: v
  // println!("The first element is: {}", v[0]);
}
