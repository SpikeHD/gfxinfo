use crate::{Gpu, GpuInfo};

#[derive(Debug, Clone)]
pub struct NvidiaGpu {
  vendor: String,
  model: String,
  family: String,
  device_id: u32,
}

impl NvidiaGpu {}

impl Gpu for NvidiaGpu {
  fn vendor(&self) -> &str {
    &self.vendor
  }

  fn model(&self) -> &str {
    &self.model
  }

  fn family(&self) -> &str {
    &self.family
  }

  fn device_id(&self) -> &u32 {
    &self.device_id
  }
}

impl GpuInfo for NvidiaGpu {
  fn total_vram(&self) -> u64 {
    todo!()
  }

  fn used_vram(&self) -> u64 {
    todo!()
  }

  fn load_pct(&self) -> u32 {
    todo!()
  }

  fn temperature(&self) -> u32 {
    todo!()
  }
}
