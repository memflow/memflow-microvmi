use std::sync::Mutex;

use log::{info, warn};

use memflow::*;
use memflow_derive::connector;

use microvmi::{
    self,
    api::{DriverInitParam, DriverType, Introspectable},
};

pub struct MicroVMI {
    driver: Mutex<Box<dyn Introspectable>>,
}

// TODO: implement me
impl Clone for MicroVMI {
    fn clone(&self) -> Self {
        panic!("unable to clone microvmi connector");
    }
}

unsafe impl Send for MicroVMI {}

impl MicroVMI {
    pub fn new(domain_name: &str, init_option: Option<DriverInitParam>) -> Result<Self> {
        let driver = microvmi::init(domain_name, None, init_option);
        Ok(Self {
            driver: Mutex::new(driver),
        })
    }

    pub fn with_type(
        ty: DriverType,
        domain_name: &str,
        init_option: Option<DriverInitParam>,
    ) -> Result<Self> {
        let driver = microvmi::init(domain_name, Some(ty), init_option);
        Ok(Self {
            driver: Mutex::new(driver),
        })
    }
}

impl PhysicalMemory for MicroVMI {
    fn phys_read_raw_list(&mut self, data: &mut [PhysicalReadData]) -> Result<()> {
        let drv = self.driver.lock().unwrap();
        for read in data.iter_mut() {
            drv.read_physical(read.0.as_u64(), read.1).ok();
        }
        Ok(())
    }

    fn phys_write_raw_list(&mut self, data: &[PhysicalWriteData]) -> Result<()> {
        // mutability issue
        panic!("not implemented yet")
        /*
        let drv = self.driver.lock().unwrap();
        for write in data.iter() {
            drv.write_physical(write.0.as_u64(), write.1).ok();
        }
        Ok(())
        */
    }

    fn metadata(&self) -> PhysicalMemoryMetadata {
        let drv = self.driver.lock().unwrap();
        PhysicalMemoryMetadata {
            size: drv.get_max_physical_addr().unwrap_or_default() as usize,
            readonly: false,
        }
    }
}

/// Creates a new MicroVMI Connector instance.
#[connector(name = "microvmi")]
pub fn create_connector(args: &ConnectorArgs) -> Result<MicroVMI> {
    let name = args
        .get("name")
        .or_else(|| args.get_default())
        .ok_or(Error::Connector("argument 'name' missing"))?;
    let option = if let Some(option) = args.get("option") {
        Some(DriverInitParam::KVMiSocket(option.to_string()))
    } else {
        None
    };

    if let Some(ty) = args.get("type") {
        MicroVMI::with_type(driver_type_from_str(ty), name, option)
    } else {
        MicroVMI::new(name, option)
    }
}

/// Converts a str into a microvmi DriverType
fn driver_type_from_str(ty: &str) -> DriverType {
    match ty {
        #[cfg(feature = "hyper-v")]
        "hyper-v" => DriverType::HyperV,
        #[cfg(feature = "kvm")]
        "kvm" => DriverType::KVM,
        #[cfg(feature = "virtualbox")]
        "virtualbox" => DriverType::VirtualBox,
        #[cfg(feature = "xen")]
        "xen" => DriverType::Xen,
        _ => {
            warn!("microvmi driver type not found: {}", ty);
            DriverType::Dummy
        }
    }
}
