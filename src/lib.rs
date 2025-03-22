use std::fmt::Debug;

use dyn_clone::{clone_trait_object, DynClone};

#[cfg(feature = "amd")]
mod amd;
#[cfg(feature = "nvidia")]
mod nvidia;
#[cfg(feature = "intel")]
mod intel;

pub trait Gpu: Send + Sync + Debug + DynClone {
  fn vendor(&self) -> &str;
  fn model(&self) -> &str;
  fn family(&self) -> &str;
  fn device_id(&self) -> &u32;
}

pub trait GpuInfo {
  fn total_vram(&self) -> u64;
  fn used_vram(&self) -> u64;
  fn load_pct(&self) -> u32;
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