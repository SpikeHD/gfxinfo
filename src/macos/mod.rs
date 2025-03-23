use std::{error::Error, ptr::null_mut};

use core_foundation::dictionary::{CFDictionary, CFMutableDictionaryRef};
use core_foundation::number::CFNumber;
use core_foundation::string::CFString;
use core_foundation::{
  base::{CFRelease, CFTypeRef, TCFType, TCFTypeRef, kCFAllocatorDefault},
  data::{__CFData, CFData},
  dictionary::__CFDictionary,
  number::__CFNumber,
  string::__CFString,
};
use io_kit_sys::{
  IOIteratorNext, IOObjectRelease, IORegistryEntryCreateCFProperties, IOServiceGetMatchingServices,
  IOServiceMatching, kIOMasterPortDefault,
  types::{io_iterator_t, io_registry_entry_t},
};

use crate::{Gpu, GpuInfo};

pub enum DataType {
  Number,
  _String,
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
    // TODO: on many systems, vram and system ram are the same
    0
  }

  fn used_vram(&self) -> u64 {
    performance_stat("In use system memory", DataType::Number).unwrap_or(0)
  }

  fn load_pct(&self) -> u32 {
    performance_stat("Device Utilization %", DataType::Number).unwrap_or(0) as u32
  }

  fn temperature(&self) -> u32 {
    // TODO: no idea how to get this
    0
  }
}

pub fn active_gpu() -> Result<Box<dyn Gpu>, Box<dyn Error>> {
  unsafe {
    // Get devices
    let match_dict = IOServiceMatching(c"IOAccelerator".as_ptr());
    let mut itr: io_iterator_t = 0;
    let result = IOServiceGetMatchingServices(kIOMasterPortDefault, match_dict, &mut itr);

    if result != 0 {
      return Err("Failed to get IOAccelerator services".into());
    }

    let entry: io_registry_entry_t;

    while {
      entry = IOIteratorNext(itr);
      entry != 0
    } {
      let mut service_dict: CFMutableDictionaryRef = null_mut();

      if IORegistryEntryCreateCFProperties(entry, &mut service_dict, kCFAllocatorDefault, 0) != 0 {
        return Err("Failed to get properties for IOAccelerator".into());
      }

      let dict: CFDictionary<CFString, CFTypeRef> =
        CFDictionary::wrap_under_create_rule(service_dict);
      let vendor = dict.find(CFString::from_static_string("vendor-id"));
      let model = dict.find(CFString::from_static_string("model"));
      let family = "N/A";

      if vendor.is_none() || model.is_none() {
        return Err("Failed to get properties for IOAccelerator".into());
      }

      let vendor_ptr = vendor.unwrap();
      let model_ptr = model.unwrap();
      let model: CFString =
        CFString::wrap_under_get_rule(model_ptr.as_void_ptr() as *const __CFString);
      let vendor: CFData = CFData::wrap_under_get_rule(vendor_ptr.as_void_ptr() as *const __CFData);
      let mut vendor = vendor.bytes().to_vec();
      vendor.reverse();

      let vendor = vendor
        .iter()
        .map(|b| {
          if *b != 0 {
            return format!("{:02x}", b);
          }

          "".to_string()
        })
        .collect::<Vec<String>>()
        .join("");

      // Is this needed? Idk
      // CFRelease(service_dict as *mut _);
      IOObjectRelease(entry);
      IOObjectRelease(itr);

      return Ok(Box::new(MacGpu {
        vendor,
        model: model.to_string(),
        family: family.to_string(),
        device_id: 0x0,
      }));
    }
  }

  Err("No GPU found".into())
}

pub fn performance_stat(stat: &'static str, data_type: DataType) -> Result<u64, Box<dyn Error>> {
  unsafe {
    // Get devices
    let match_dict = IOServiceMatching(c"IOAccelerator".as_ptr());
    let mut itr: io_iterator_t = 0;
    let result = IOServiceGetMatchingServices(kIOMasterPortDefault, match_dict, &mut itr);

    if result != 0 {
      return Err("Failed to get IOAccelerator services".into());
    }

    let entry: io_registry_entry_t;

    while {
      entry = IOIteratorNext(itr);
      entry != 0
    } {
      let mut service_dict: CFMutableDictionaryRef = null_mut();

      if IORegistryEntryCreateCFProperties(entry, &mut service_dict, kCFAllocatorDefault, 0) != 0 {
        IOObjectRelease(entry);
        return Err("Failed to get properties for IOAccelerator".into());
      }

      let dict: CFDictionary<CFString, CFTypeRef> =
        CFDictionary::wrap_under_create_rule(service_dict);
      let perf_properties = dict.find(CFString::from_static_string("PerformanceStatistics"));

      if perf_properties.is_none() {
        CFRelease(service_dict as *mut _);
        IOObjectRelease(entry);
        return Err("Failed to get properties for IOAccelerator".into());
      }

      let perf_properties: CFDictionary<CFString, CFTypeRef> = CFDictionary::wrap_under_create_rule(
        perf_properties.unwrap().as_void_ptr() as *const __CFDictionary,
      );
      let stat = perf_properties.find(CFString::from_static_string(stat));

      if stat.is_none() {
        CFRelease(service_dict as *mut _);
        IOObjectRelease(entry);
        return Err("Failed to get properties for IOAccelerator".into());
      }

      let stat = stat.unwrap();

      CFRelease(service_dict as CFTypeRef);
      IOObjectRelease(entry);
      IOObjectRelease(itr);

      match data_type {
        DataType::Number => {
          let stat: CFNumber =
            CFNumber::wrap_under_get_rule(stat.as_void_ptr() as *const __CFNumber);
          let stat_u64 = stat.to_i64().unwrap_or(0);
          return Ok(stat_u64.try_into().unwrap_or(0));
        }
        _ => {
          return Err("Unsupported data type".into());
        }
      }
    }
  }

  Err("Failed to get ".into())
}
