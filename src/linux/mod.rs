#[cfg(feature = "amd")]
pub mod amd;
#[cfg(feature = "intel")]
pub mod intel;
#[cfg(feature = "nvidia")]
pub mod nvidia;

pub fn active_gpu() -> Box<dyn crate::Gpu> {
  #[cfg(feature = "amd")]
  {
    let gpu = amd::active_gpu();
    if let Ok(gpu) = gpu {
      return Box::new(gpu);
    }
  }

  #[cfg(feature = "nvidia")]
  {
    let gpu = nvidia::active_gpu();
    if let Ok(gpu) = gpu {
      return Box::new(gpu);
    }
  }

  #[cfg(feature = "intel")]
  {
    todo!()
  }
}