pub fn get_map(input: &str) -> Vec<Vec<u8>> {
  input
      .lines()
      .map(|l| {
          l.bytes().collect()
      }).collect()
}