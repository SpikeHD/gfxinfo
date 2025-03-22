use std::{error::Error, rc::Rc};

use nvml_wrapper::{enum_wrappers::device::{Brand, TemperatureSensor}, Nvml};

use crate::{Gpu, GpuInfo};

#[derive(Debug)]
pub struct NvidiaGpu {
  nvml: Rc<Nvml>,

  vendor: String,
  model: String,
  family: String,
  device_id: u32,
}

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

  #[cfg(feature = "gpu_info")]
  fn info(&self) -> Box<dyn GpuInfo> {
    Box::new(NvidiaGpuInfo {
      nvml: self.nvml.clone(),
    })
  }
}

#[cfg(feature = "gpu_info")]
struct NvidiaGpuInfo {
  nvml: Rc<Nvml>,
}

#[cfg(feature = "gpu_info")]
impl GpuInfo for NvidiaGpuInfo {
  fn total_vram(&self) -> u64 {
    let meminfo = match self.nvml.device_by_index(0)
      .and_then(|device| device.memory_info()) {
        Ok(meminfo) => meminfo,
        Err(_) => return 0,
      };

    meminfo.total
  }

  fn used_vram(&self) -> u64 {
    let meminfo = match self.nvml.device_by_index(0)
      .and_then(|device| device.memory_info()) {
        Ok(meminfo) => meminfo,
        Err(_) => return 0,
      };

    meminfo.used
  }

  fn load_pct(&self) -> u32 {
    let util = match self.nvml.device_by_index(0)
      .and_then(|device| device.utilization_rates()) {
        Ok(u) => u,
        Err(_) => return 0,
      };

    util.gpu
  }

  fn temperature(&self) -> u32 {
    let temp = match self.nvml.device_by_index(0)
      .and_then(|device| device.temperature(TemperatureSensor::Gpu)) {
        Ok(u) => u,
        Err(_) => return 0,
      };

    // This one returns as celsius, so we need to make it consistent
    temp * 1000
  }
}

pub fn active_gpu() -> Result<NvidiaGpu, Box<dyn Error>> {
  let nvml = Nvml::init()?;

  let vendor;
  let model;
  let family;
  let device_id;

  {
    // TODO: first device may not always be the active one
    let device = nvml.device_by_index(0)?;

    vendor = "Nvidia".to_string();
    model = device.name()?;
    family = brand_to_string(device.brand()?);
    device_id = device.pci_info()?.pci_device_id;
  }

  Ok(NvidiaGpu {
    nvml: Rc::new(nvml),

    vendor,
    model,
    family,
    device_id,
  })
}

fn brand_to_string(brand: Brand) -> String {
  match brand {
    Brand::GeForce => "GeForce".to_string(),
    Brand::Nvidia => "Nvidia".to_string(),
    Brand::NvidiaRTX => "NvidiaRTX".to_string(),
    Brand::Tesla => "Tesla".to_string(),
    Brand::Unknown => "Unknown".to_string(),
    Brand::NVS => "NVS".to_string(),
    Brand::GRID => "GRID".to_string(),
    Brand::Quadro => "Quadro".to_string(),
    Brand::QuadroRTX => "QuadroRTX".to_string(),
    Brand::GeForceRTX => "GeForceRTX".to_string(),
    Brand::Titan => "Titan".to_string(),
    Brand::TitanRTX => "TitanRTX".to_string(),
    Brand::VApps => "VApps".to_string(),
    Brand::VPC => "VPC".to_string(),
    Brand::VCS => "VCS".to_string(),
    Brand::VWS => "VWS".to_string(),
    Brand::CloudGaming => "CloudGaming".to_string(),
    Brand::VGaming => "VGaming".to_string(),
  }
}
