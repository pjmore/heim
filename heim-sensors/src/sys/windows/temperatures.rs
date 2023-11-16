use heim_common::prelude::*;
use wmi::WMIConnection;

use crate::TemperatureSensor;
#[derive(Deserialize)]

pub (crate) struct MsAcpi_ThermalZoneTemperature{
    pub Active: bool,
    pub CriticalTripPoint: u32,
    pub CurrentTemperature: u32,
    pub InstanceName: String
}

thread_local! {
    static COM_LIB: COMLibrary = COMLibrary::new().unwrap();
}
pub async fn temperatures() -> impl Stream<Item = Result<TemperatureSensor>> + Send {
    let com_lib = COM_LIB.with(|com| *com);
    let conn = WMIConnection::new(com_lib).unwrap();

    let query = conn.async_raw_query("");
    let o = query.try_flatten_stream().into_stream();
    while let Some(t) = o.next().await{
    }
    // TODO: Stub
    stream::iter(vec![])
}


