//! Windows-specific routines used across `heim` crates.

mod handle;
mod time;
use wmi::{COMLibrary, WMIConnection};
pub use self::handle::Handle;

pub use wmi;

thread_local! {
    static COM_LIB: COMLibrary = COMLibrary::new().unwrap();
}

pub fn com_lib()->COMLibrary{
    COM_LIB.with(|com| *com)
}
pub fn wmi_conn()->WMIConnection{
    WMIConnection::new(com_lib()).unwrap()
}
pub fn wmi_con_with_namespace_path(namespace_path: &str)->WMIConnection{
    WMIConnection::with_namespace_path(namespace_path, com_lib()).unwrap()
}

