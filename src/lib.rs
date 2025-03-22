use std::fmt::Debug;

use dyn_clone::{clone_trait_object, DynClone};

#[cfg(feature = "amd")]
mod amd;
#[cfg(feature = "nvidia")]
mod nvidia;
#[cfg(feature = "intel")]
mod intel;

pub trait Gpu: Send + Sync + Debug + DynClone + GpuInfo {
  /// Get the GPU vendor (ie. AMD, Nvidia).
  fn vendor(&self) -> &str;
  // Get the GPU model (ie. Radeon RX 6800, GeForce RTX 3070).
  fn model(&self) -> &str;
  /// Get the GPU family (ie. Vega, Navi).
  fn family(&self) -> &str;
  /// Get the GPU device ID.
  fn device_id(&self) -> &u32;
}

/// Trait for providing GPU information. If any numbers return 0, they are likely unsupported or otherwise not available.
#[cfg(feature = "gpu_info")]
pub trait GpuInfo {
  /// Get the total amount of VRAM in bytes.
  fn total_vram(&self) -> u64;
  /// Get the amount of used VRAM in bytes.
  fn used_vram(&self) -> u64;
  /// Get the load percentage.
  fn load_pct(&self) -> u32;
  /// Get the GPU temperature in degrees mcelsius.
  fn temperature(&self) -> u32;
}

clone_trait_object!(Gpu);

/// Attempts to get the currently active GPU, using vendor-specific methods.
pub fn active_gpu() -> Box<dyn Gpu> {
  #[cfg(feature = "amd")]
  {
    let gpu = amd::active_gpu();
    if let Ok(gpu) = gpu {
      return Box::new(gpu);
    }
  }

  #[cfg(feature = "nvidia")]
  {
    todo!()
  }

  #[allow(unreachable_code)]
  #[cfg(feature = "intel")]
  {
    todo!()
  }
}