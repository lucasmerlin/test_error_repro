fn main() {
  assert_eq!(1, 2);
}
#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(1, 2);
  }
}
