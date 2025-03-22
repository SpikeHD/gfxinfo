pub fn main() {
  let gpu = gpuinfo::active_gpu();
  println!("Vendor: {}", gpu.vendor());
  println!("Model: {}", gpu.model());
  println!("Family: {}", gpu.family());
  println!("Device ID: 0x{:X}", gpu.device_id());
}
