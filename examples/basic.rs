pub fn main() {
  let gpu = gfxinfo::active_gpu().expect("No GPU found");
  println!("Vendor: {}", gpu.vendor());
  println!("Model: {}", gpu.model());
  println!("Family: {}", gpu.family());
  println!("Device ID: 0x{:X}", gpu.device_id());
}
