pub fn main() {
  let gpu = gpuinfo::active_gpu();
  println!("VRAM usage: {} / {}", byte_to_mb(gpu.used_vram()), byte_to_mb(gpu.total_vram()));
  println!("Load: {}%", gpu.load_pct());
  println!("Temperature: {} C", gpu.temperature() / 1000);
}

fn byte_to_mb(bytes: u64) -> String {
  format!("{:.2} MB", bytes as f64 / 1024.0 / 1024.0)
}