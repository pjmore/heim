#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use raw_cpuid::{CpuId, Hypervisor};

use crate::Virtualization;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn detect_vm_cpuid() -> Result<Virtualization, ()> {
    let cpuid = match CpuId::new().get_hypervisor_info() {
        Some(info) => info,
        None => return Err(()),
    };

    match cpuid.identify() {
        Hypervisor::Xen => Ok(Virtualization::Xen),
        // https://docs.microsoft.com/en-us/virtualization/hyper-v-on-windows/reference/tlfs
        Hypervisor::HyperV => Ok(Virtualization::HyperV),
        Hypervisor::KVM => Ok(Virtualization::Kvm),
        // https://kb.vmware.com/s/article/1009458
        Hypervisor::VMware => Ok(Virtualization::Vmware),
        Hypervisor::QEMU => Ok(Virtualization::Qemu),
        Hypervisor::Bhyve => Ok(Virtualization::Bhyve),
        Hypervisor::QNX => Ok(Virtualization::Qnx),
        Hypervisor::ACRN => Ok(Virtualization::Acrn),
        Hypervisor::Unknown(_b, _c, _d) => {
            // TODO: Logging
            Err(())
        }
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub fn detect_vm_cpuid() -> Result<Virtualization, ()> {
    Err(())
}
