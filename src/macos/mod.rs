use std::{error::Error, marker::{PhantomData, PhantomPinned}, ptr::null_mut};

use core_foundation::{base::CFTypeRef, dictionary::{CFDictionaryRef, CFMutableDictionaryRef}, string::CFStringRef};

use crate::{Gpu, GpuInfo};

pub type CVoidRef = *const std::ffi::c_void;


#[repr(C)]
struct IOReportSubscription {
  _data: [u8; 0],
  _phantom: PhantomData<(*mut u8, PhantomPinned)>,
}

type IOReportSubscriptionRef = *const IOReportSubscription;

// https://medium.com/@vladkens/how-to-get-macos-power-metrics-with-rust-d42b0ad53967
#[link(name = "IOReport", kind = "dylib")]
unsafe extern "C" {
  fn IOReportCopyChannelsInGroup(a: CFStringRef, b: CFStringRef, c: u64, d: u64, e: u64) -> CFDictionaryRef;
  fn IOReportMergeChannels(a: CFDictionaryRef, b: CFDictionaryRef, nil: CFTypeRef);
  fn IOReportCreateSubscription(a: CVoidRef, b: CFMutableDictionaryRef, c: *mut CFMutableDictionaryRef, d: u64, b: CFTypeRef) -> IOReportSubscriptionRef;
  fn IOReportCreateSamples(a: IOReportSubscriptionRef, b: CFMutableDictionaryRef, c: CFTypeRef) -> CFDictionaryRef;
  fn IOReportCreateSamplesDelta(a: CFDictionaryRef, b: CFDictionaryRef, c: CFTypeRef) -> CFDictionaryRef;
}

#[derive(Debug)]
pub struct MacGpu {
  vendor: String,
  model: String,
  family: String,
  device_id: u32,
}

impl Gpu for MacGpu {
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
    Box::new(MacGpuInfo {})
  }
}

#[cfg(feature = "gpu_info")]
pub struct MacGpuInfo {}

#[cfg(feature = "gpu_info")]
impl GpuInfo for MacGpuInfo {
  fn total_vram(&self) -> u64 {
    0
  }

  fn used_vram(&self) -> u64 {
    0
  }

  fn load_pct(&self) -> u32 {
    0
  }

  fn temperature(&self) -> u32 {
    0
  }
}

pub fn active_gpu() -> Result<Box<dyn Gpu>, Box<dyn Error>> {
  Err("No GPU found".into())
}
