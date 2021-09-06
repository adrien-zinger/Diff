pub fn apply<T: std::clone::Clone + std::marker::Copy + std::fmt::Debug>(mut source: Vec::<T>, diff: Vec<(u8, u32, u32, Vec<T>, Vec<T>)>) -> Vec::<T> {
  for operation in diff.iter() {
    if operation.0 == 0 {
      let range = (operation.1) as usize..(operation.1 + operation.2) as usize;
      source.splice(range, operation.4.clone());
    }
    if operation.0 == 1 {
      let range = (operation.1 + 1) as usize..(operation.1 + 1) as usize;
      source.splice(range, operation.3.clone());
    }
    if operation.0 == 2 {
      let range = operation.1 as usize..(operation.1 + operation.2) as usize;
      source.splice(range, Vec::new());
    }
  }
  source
}
