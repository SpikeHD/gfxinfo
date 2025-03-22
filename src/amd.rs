use std::{error::Error, fmt::Debug, fs::File, os::fd::AsRawFd, path::PathBuf, sync::{Arc, Mutex}};

use libdrm_amdgpu_sys::{LibDrmAmdgpu, AMDGPU::{self, DeviceHandle, GPU_INFO}};

use crate::Gpu;

#[derive(Clone)]
pub struct AmdGpu {
  path: PathBuf,
  device: Arc<Mutex<DeviceHandle>>,

  vendor: String,
  model: String,
  family: String,
  device_id: u32,
}

impl Debug for AmdGpu {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("AmdGpu")
      .field("path", &self.path)
      .field("vendor", &self.vendor)
      .field("model", &self.model)
      .field("family", &self.family)
      .field("device_id", &self.device_id)
      .finish()
  }
}

impl Gpu for AmdGpu {
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

// https://github.com/Umio-Yasuno/libdrm-amdgpu-sys-rs/blob/main/examples/amdgpu_info.rs
pub fn active_gpu() -> Result<AmdGpu, Box<dyn Error>> {  
  let drm = LibDrmAmdgpu::new().map_err(|_| "Could not initialize libdrm")?;
  let pci_devs = AMDGPU::get_all_amdgpu_pci_bus();

  if pci_devs.is_empty() {
    return Err("No AMD GPU found".into());
  }

  // TODO: first() is almost definitely not the right way to do this
  let dev_path = match pci_devs.first() {
    Some(pci_dev) => pci_dev.get_drm_render_path()?,
    None => return Err("No AMD GPU found".into()),
  };
  
  let (dev, _, _) = {
    let fd = File::open(&dev_path)?;
    drm.init_device_handle(fd.as_raw_fd()).map_err(|e| format!("Could not init device handle: {e}"))?
  };

  let info = dev.device_info().map_err(|e| format!("Could not get device info: {e}"))?;

  Ok(
    AmdGpu {
      path: dev_path,
      device: Arc::new(Mutex::new(dev)),

      vendor: "AMD".to_string(),
      model: info.find_device_name_or_default().to_string(),
      family: info.get_family_name().to_string(),
      device_id: info.device_id(),
    }
  )
}