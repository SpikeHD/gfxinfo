use std::{error::Error, rc::Rc};

use crate::{Gpu, GpuInfo};

#[derive(Debug)]
pub struct IntelGpu {
    nvml: Rc<()>,

    vendor: String,
    model: String,
    family: String,
    device_id: u32,
}

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

    #[cfg(feature = "gpu_info")]
    fn info(&self) -> Box<dyn GpuInfo> {
        Box::new(IntelGpuInfo {
            nvml: self.nvml.clone(),
        })
    }
}

#[cfg(feature = "gpu_info")]
struct IntelGpuInfo {
    nvml: Rc<()>,
}

#[cfg(feature = "gpu_info")]
impl GpuInfo for IntelGpuInfo {
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

pub fn active_gpu() -> Result<IntelGpu, Box<dyn Error>> {
    todo!()
}
