use serde::Deserialize;
use std::{error::Error, ptr::null_mut};
use windows::{
  Win32::System::Performance::{
    PDH_CSTATUS_BAD_COUNTERNAME, PDH_CSTATUS_VALID_DATA, PDH_FMT_DOUBLE, PdhAddCounterW,
    PdhCloseQuery, PdhCollectQueryData, PdhGetFormattedCounterValue, PdhGetRawCounterValue,
    PdhOpenQueryW,
  },
  core::PCWSTR,
};
use wmi::{COMLibrary, WMIConnection};

use crate::{Gpu, GpuInfo};

#[cfg(feature = "nvidia")]
use crate::nvidia;

#[derive(Debug)]
pub struct WindowsGpu {
  total_vram: u64,

  vendor: String,
  model: String,
  family: String,
  device_id: u32,
}

impl Gpu for WindowsGpu {
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
    Box::new(WindowsGpuInfo {
      total_vram: self.total_vram,
    })
  }
}

#[cfg(feature = "gpu_info")]
pub struct WindowsGpuInfo {
  // This is grabbed in the WindowsGpu struct
  total_vram: u64,
}

#[cfg(feature = "gpu_info")]
impl GpuInfo for WindowsGpuInfo {
  fn total_vram(&self) -> u64 {
    self.total_vram
  }

  fn used_vram(&self) -> u64 {
    unsafe { counter_value("\\GPU Adapter Memory(*)\\Dedicated Usage".to_string()).unwrap_or(0) }
  }

  fn load_pct(&self) -> u32 {
    unsafe {
      counter_value("\\GPU Engine(*)\\Utilization Percentage".to_string()).unwrap_or(0) as u32
    }
  }

  fn temperature(&self) -> u32 {
    // TODO: fix
    0
  }
}

#[derive(Deserialize)]
#[serde(rename = "Win32_VideoController")]
#[serde(rename_all = "PascalCase")]
struct WMIGpu {
  name: String,
  video_processor: String,
  adapter_compatibility: String,
  device_id: String,
  adapter_ram: u32,
}

pub fn active_gpu() -> Result<Box<dyn Gpu>, Box<dyn Error>> {
  // Prefer using the easy Nvidia library
  #[cfg(feature = "nvidia")]
  {
    let gpu = nvidia::active_gpu()?;
    if let Ok(gpu) = gpu {
      return Ok(Box::new(gpu));
    }
  }

  let com = COMLibrary::new()?;
  let wmi = WMIConnection::new(com)?;
  let gpu: Vec<WMIGpu> = wmi.raw_query("SELECT * FROM Win32_VideoController")?;

  let gpu = match gpu.first() {
    Some(gpu) => gpu,
    None => return Err("No GPU found".into()),
  };

  Ok(Box::new(WindowsGpu {
    total_vram: gpu.adapter_ram as u64,

    vendor: gpu.video_processor.clone(),
    model: gpu.name.clone(),
    family: gpu.adapter_compatibility.clone(),
    // TODO: fix
    device_id: 0x0,
  }))
}

pub unsafe fn counter_value(counter_path: String) -> Result<u64, Box<dyn Error>> {
  let query = null_mut();
  let status = unsafe { PdhOpenQueryW(None, 0, query); };

  if status != 0 {
    return Err(format!("Could not open query: {}", status).into());
  }

  let counter_path = PCWSTR(
    counter_path
      .encode_utf16()
      .chain(std::iter::once(0))
      .collect::<Vec<u16>>()
      .as_ptr(),
  );

  let counter = null_mut();

  let status = unsafe { PdhAddCounterW(*query, counter_path, 0, counter); };

  if status != 0 {
    return Err(format!("Could not add counter: {}", status).into());
  }

  let value = null_mut();
  let status = unsafe { PdhCollectQueryData(*query); };

  if status != 0 {
    return Err(format!("Could not collect query data: {}", status).into());
  }

  let status = unsafe { PdhGetFormattedCounterValue(*counter, PDH_FMT_DOUBLE, None, value); };

  if status != 0 {
    return Err(format!("Could not get raw counter value: {}", status).into());
  }

  let mut value = 0u64;

  // Deref value
  unsafe {
    let deref = *value;
    value = value.Anonymous.doubleValue.round() as u64;
  }

  // Close query
  let status = unsafe { PdhCloseQuery(*query); };

  if status != 0 {
    return Err(format!("Could not close query: {}", status).into());
  }

  Ok(value)
}
