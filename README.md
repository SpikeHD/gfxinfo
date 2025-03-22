<div align="center">
  <h1>gpuinfo</h1>
  <p>
    <strong>Rust library for querying GPU information</strong>
  </p>
</div>

# TL;DR

```rust
use gpuinfo::active_gpu;

let gpu = active_gpu();
println!("GPU vendor: {}", gpu.vendor());
println!("GPU model: {}", gpu.model());
println!("GPU family: {}", gpu.family());
println!("GPU device ID: {}", gpu.device_id());

// And with `gpu_info` feature enabled
let info = gpu.info();
println!("Total VRAM: {} bytes", info.total_vram());
println!("Used VRAM: {} bytes", info.used_vram());
println!("Load: {}%", info.load_pct());
println!("Temperature: {} C", info.temperature() / 1000);
```

# TODO

* [ ] Cross-platform
  * [ ] Windows support
  * [x] Linux support
  * [ ] MacOS support
* [ ] Multi-vendor
  * [x] Nvidia
  * [x] AMD
  * [ ] Intel
  * [ ] Generic/Other
* [ ] Supported information
  * [x] Vendor
  * [x] Model
  * [x] Family
  * [x] Device ID
  * [x] Total VRAM
  * [x] Used VRAM
  * [x] Load
  * [x] Temperature