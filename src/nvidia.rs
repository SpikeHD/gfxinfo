use crate::Gpu;

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