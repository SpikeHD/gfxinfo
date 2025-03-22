pub fn main() {
  let gpus = gpuinfo::active_gpu();
  println!("{gpus:?}");
}