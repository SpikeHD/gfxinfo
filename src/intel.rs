use crate::{Gpu, GpuInfo};

#[derive(Debug, Clone)]
pub struct IntelGpu {
  vendor: String,
  model: String,
  family: String,
  device_id: u32,
}

impl IntelGpu {}

impl Gpu for IntelGpu {
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

impl GpuInfo for IntelGpu {
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