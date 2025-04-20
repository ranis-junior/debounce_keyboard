use std::io::Error;
use std::mem;
use windows::Win32::Devices::DeviceAndDriverInstallation::{DIGCF_DEVICEINTERFACE, DIGCF_PRESENT, HDEVINFO, SP_DEVINFO_DATA, SPDRP_DEVICEDESC, SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo, SetupDiGetClassDevsW, SetupDiGetDevicePropertyW, SetupDiGetDeviceRegistryPropertyW, SPDRP_FRIENDLYNAME, SPDRP_HARDWAREID};
use windows::Win32::Devices::HumanInterfaceDevice::GUID_DEVINTERFACE_KEYBOARD;
use windows::Win32::Devices::Properties::{
    DEVPKEY_Device_Driver, DEVPKEY_Device_FriendlyName, DEVPROP_TYPE_GUID, DEVPROP_TYPE_STRING,
    DEVPROPTYPE,
};
use windows::Win32::Foundation::{ERROR_NO_MORE_ITEMS, GetLastError, MAX_PATH};

fn get_keyboards_description() -> Result<Vec<(String, String)>, String> {
    let devices = unsafe {
        SetupDiGetClassDevsW(
            Some(&GUID_DEVINTERFACE_KEYBOARD), // GUID for keyboard
            None,
            None,
            DIGCF_PRESENT | DIGCF_DEVICEINTERFACE,
        )
    };

    let mut results = Vec::new();
    match devices {
        Ok(device_info_set) => {
            if device_info_set.is_invalid() {
                return Err("Could not get devices".to_string());
            }

            // Enumerate devices in the device information set
            let mut index: u32 = 0;

            loop {
                let mut dev_info_data: SP_DEVINFO_DATA = SP_DEVINFO_DATA::default();
                dev_info_data.cbSize = mem::size_of::<SP_DEVINFO_DATA>() as u32;

                let result =
                    unsafe { SetupDiEnumDeviceInfo(device_info_set, index, &mut dev_info_data) };
                match result {
                    Ok(_) => {
                        match get_driver_id(device_info_set, &dev_info_data) {
                            Ok(guid) => {
                                match get_device_friendly_name(device_info_set, &dev_info_data) {
                                    Ok(device_name) => {
                                        results.push((guid, device_name));
                                    }
                                    Err(err) => {
                                        eprintln!("Error {:?}", err);
                                    }
                                };
                            }
                            Err(err) => {
                                eprintln!("Error {:?}", err);
                            }
                        };
                        index += 1;
                    }
                    Err(err) => {
                        eprintln!("Error {:?}", err);
                        let x = unsafe { GetLastError() };
                        if x == ERROR_NO_MORE_ITEMS {
                            break;
                        }
                        continue;
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error {:?}", err);
            return Err(err.message());
        }
    };
    Ok(results)
}

fn get_driver_id(
    device_info_set: HDEVINFO,
    dev_info_data: &SP_DEVINFO_DATA,
) -> Result<String, String> {
    let mut data_type: DEVPROPTYPE = DEVPROP_TYPE_GUID;
    let mut data: Vec<u8> = vec![0u8; MAX_PATH as usize];
    let buffer: Option<&mut [u8]> = Some(&mut data[..]);
    let ptr: *mut u32 = 0 as *mut u32;
    let reqsize: Option<*mut u32> = Some(ptr);

    let result = unsafe {
        SetupDiGetDevicePropertyW(
            device_info_set,
            dev_info_data,
            &DEVPKEY_Device_Driver,
            &mut data_type,
            buffer,
            reqsize,
            0,
        )
    };

    match result {
        Ok(_) => {
            let mut vec_u16: Vec<u16> = Vec::new();
            let mut i = 0;
            while i < data.len() {
                vec_u16.push((data[i + 1] as u16) << 8 | data[i] as u16);
                i += 2; // Convert to u16 manually because it is utf16
            }
            let b = String::from_utf16_lossy(&vec_u16);
            let b = b.trim_end_matches('\0');
            Ok(b.to_string())
        }
        Err(e) => {
            eprintln!("Error {:?}", e);
            unsafe { SetupDiDestroyDeviceInfoList(device_info_set).unwrap() };
            Err(e.message())
        }
    }
}

fn get_device_friendly_name(
    device_info_set: HDEVINFO,
    dev_info_data: &SP_DEVINFO_DATA,
) -> Result<String, String> {
    let mut data_type: DEVPROPTYPE = DEVPROP_TYPE_STRING;
    let mut data: Vec<u8> = vec![0u8; MAX_PATH as usize];
    let buffer: Option<&mut [u8]> = Some(&mut data[..]);
    let ptr: *mut u32 = 0 as *mut u32;
    let reqsize: Option<*mut u32> = Some(ptr);

    let result = unsafe {
        SetupDiGetDevicePropertyW(
            device_info_set,
            dev_info_data,
            &DEVPKEY_Device_FriendlyName,
            &mut data_type,
            buffer,
            reqsize,
            0,
        )
    };

    match result {
        Ok(_) => {
            // let data1 = u32::from_le_bytes(data[0..4].try_into().unwrap());
            // let data2 = u16::from_le_bytes(data[4..6].try_into().unwrap());
            // let data3 = u16::from_le_bytes(data[6..8].try_into().unwrap());
            // let mut data4: [u8; 8] = [0; 8];
            // data4.copy_from_slice(&data[8..16]);
            //
            // let x = GUID::from_values(data1, data2, data3, data4);
            // let x = format!("{{{:?}}}", x).to_lowercase();
            // println!("GUID {}", x);
            Ok(String::from_utf8_lossy(&data).to_string())
        }
        Err(err) => unsafe {
            eprintln!("Error {:?}", err);
            // calling fallback mode
            get_device_registry_description(device_info_set, dev_info_data)
        },
    }
}

fn get_device_registry_description(
    device_info_set: HDEVINFO,
    dev_info_data: &SP_DEVINFO_DATA,
) -> Result<String, String> {
    let mut buffer = [0u8; 512];
    let mut reqsize = 0u32;
    let result = unsafe {
        SetupDiGetDeviceRegistryPropertyW(
            device_info_set,
            dev_info_data,
            SPDRP_DEVICEDESC,
            None,
            Some(&mut buffer),
            Some(&mut reqsize),
        )
    };
    match result {
        Ok(_) => {
            let mut vec_u16: Vec<u16> = Vec::new();
            let mut i = 0;
            while i < buffer.len() {
                vec_u16.push((buffer[i + 1] as u16) << 8 | buffer[i] as u16);
                i += 2; // Convert to u16 manually because it is utf16
            }
            let b = String::from_utf16_lossy(&vec_u16);
            let b = b.trim_end_matches('\0');
            
            println!("UTF-16: {}", b);
            let buffer = &buffer[..reqsize as usize];
            Ok(String::from_utf8_lossy(buffer).to_string())
        },
        Err(e) => {
            eprintln!("Error {}", e.message());
            Err(e.message())
        },
    }
}
fn close_device_information_set(device_info_set: HDEVINFO) {
    let result = unsafe { SetupDiDestroyDeviceInfoList(device_info_set) };
    if let Err(e) = result {
        eprintln!("Error {:?}", e);
    }
}

fn get_error() -> String {
    format!("{:?}", unsafe { GetLastError() })
}

fn main() {
    let ids = get_keyboards_description();
    println!("{:?}", ids);
}
