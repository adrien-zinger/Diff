use std::io::Write;
pub fn write(name: &str, mut diff: std::vec::Vec<(u8, u32, std::vec::Vec<u8>, std::vec::Vec<u8>)>) {
  let mut res = std::vec::Vec::new();
  for operation in diff.iter_mut() {
    res.push(operation.0);
    let position = operation.1.to_be_bytes();
    // here, instead of writing 4 bytes, we can check the
    // required size before and dump only wht we want, in the
    // better case, we get 8 bytes less, in the worst we have
    // only one more
    res.push(position[0]);
    res.push(position[1]);
    res.push(position[2]);
    res.push(position[3]);
    let size_raw = (operation.2.len() as u32).to_be_bytes();
    res.push(size_raw[0]);
    res.push(size_raw[1]);
    res.push(size_raw[2]);
    res.push(size_raw[3]);
    res.append(&mut operation.2);
    let size_sub = (operation.3.len() as u32).to_be_bytes();
    res.push(size_sub[0]);
    res.push(size_sub[1]);
    res.push(size_sub[2]);
    res.push(size_sub[3]);
    res.append(&mut operation.3);
  }
  let mut file = std::fs::File::create(name).unwrap();
  file.write(&snap::raw::Encoder::new().compress_vec(&res).unwrap()).unwrap();
}

pub fn _decompress() -> std::vec::Vec<(u8, u32, std::vec::Vec<u8>, std::vec::Vec<u8>)> {
  std::vec::Vec::new()
}