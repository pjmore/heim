
use std::{os::unix::prelude::OsStrExt, path::PathBuf, ffi::{OsStr, OsString}, io::ErrorKind};
use heim_common::{prelude::*, Context};
use heim_common::prelude::future::try_join3;
use heim_common::units::{RationalFrequency, Rational32, frequency::cycle_per_minute};
use heim_runtime as rt;
use crate::FanSensor;


#[inline]
fn file_name(prefix: &OsStr, postfix: &[u8]) -> OsString {
    let mut name = OsString::with_capacity(prefix.len() + postfix.len());
    name.push(prefix);
    name.push(OsStr::from_bytes(postfix));
    name
}

pub fn fans() -> impl Stream<Item = Result<FanSensor>> {
    // TODO: It would be nice to have async glob matchers :(
    // Basically we are searching for `/sys/class/hwmon/temp*_*` files here
    rt::fs::read_dir(rt::linux::sysfs_root().join("class/hwmon"))
        .try_flatten_stream()
        .try_filter(|entry| future::ready(entry.file_name().as_bytes().starts_with(b"hwmon")))
        .and_then(|entry| {
            let inner = rt::fs::read_dir(entry.path())
                .try_flatten_stream()
                .try_filter(|entry| {
                    let name = entry.file_name();
                    let bytes = name.as_bytes();

                    future::ready(bytes.starts_with(b"fan") && bytes.ends_with(b"_input"))
                });

            future::ok(inner)
        })
        .try_flatten()
        .map_err(Error::from)
        .and_then(|entry| hwmon_sensor(entry.path()))
}


async fn hwmon_sensor(input: PathBuf) -> Result<FanSensor> {
    // It is guaranteed by `hwmon` and `hwmon_sensor` directory traversals,
    // that it is not a root directory and it points to a file.
    // Otherwise it is an implementation bug.
    let root = input.parent().unwrap();
    let name = input.file_name().unwrap();
    let offset = name.len() - b"input".len(); 
    let prefix = OsStr::from_bytes(&name.as_bytes()[..offset]);


    let path = root.join("name");
    let unit_name = async {
        let mut name = rt::fs::read_to_string(path).await?;
        let _ = name.pop(); // Remove traling newline
        Ok(name)
    };


    let path = root.join(file_name(prefix, b"label"));
    let label = async {
        let lable_res = rt::fs::read_to_string(path).await;
        match lable_res {
            Err(_)=>Ok(None),
            Ok(mut s)=>{
                let _ = s.pop();
                Result::<Option<String>>::Ok(Some(s))
            }
        }
    };

    let path = root.join(file_name(prefix, b"input"));
    let rpm = async{
        let mut content = rt::fs::read_to_string(path.clone()).await?;
        let _ = content.pop();
        let res: i32 = content.parse().map_err(|err: std::num::ParseIntError| Error::new(std::io::Error::new(ErrorKind::InvalidData, err), Context::File { path: path }))?;
        Ok(RationalFrequency::new::<cycle_per_minute>(Rational32::new(res, 1)))
    };
    let (unit_name, label, rpm) = try_join3(unit_name, label, rpm).await?;
    Ok(FanSensor{
        unit: unit_name,
        label: label,
        current: rpm
    })
    
} 