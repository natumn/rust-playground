fn main() {
  let x = 5;

  let y = if x==5 { 10 } else { 15 };

  println!("{}",y);

  let number = match x {
      1 => "one",
      3 => "three",
      5 => "five",
      _ => "another number",
  };

  println!("{}",number);
}
