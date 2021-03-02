fn main() {
  let mut s = String::from("hello");

  s.push_str(", world!"); // push_str() appends a literal to a String

  // takes_ownership(s);

  let x = 5;

  makes_copy(x);

  let len = calculate_length(&s);

  println!("{} length is {}", s, len);
}


fn takes_ownership(some_string: String) { // some_string comes into scope
  println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
  println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: &String) -> usize {
  s.len()
}