pub fn main() {
  let gpu = gfxinfo::active_gpu().expect("No GPU found");
  let info = gpu.info();
  println!(
    "VRAM usage: {} / {}",
    byte_to_mb(info.used_vram()),
    byte_to_mb(info.total_vram())
  );
  println!("Load: {}%", info.load_pct());
  println!("Temperature: {} C", info.temperature() / 1000);
}

fn byte_to_mb(bytes: u64) -> String {
  format!("{:.2} MB", bytes as f64 / 1024.0 / 1024.0)
}
