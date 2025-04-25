use std::ffi::c_void;
use std::mem::zeroed;
use windows::core::{s, Error, PCSTR};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::{GetRawInputData, RegisterRawInputDevices, HRAWINPUT, RAWINPUT, RAWINPUTDEVICE, RAWINPUTHEADER, RIDEV_INPUTSINK, RIDEV_NOLEGACY, RID_INPUT, RIM_TYPEKEYBOARD};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, RegisterClassExA, TranslateMessage,
    CW_USEDEFAULT, HWND_MESSAGE, MSG, WM_INPUT, WNDCLASSEXA, WS_OVERLAPPEDWINDOW,
};

use regex::Regex;
use windows::Win32::Devices::DeviceAndDriverInstallation::{
    SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo, SetupDiGetClassDevsW, SetupDiGetDeviceInstanceIdW, SetupDiGetDevicePropertyW,
    SetupDiGetDeviceRegistryPropertyW, DIGCF_DEVICEINTERFACE, DIGCF_PRESENT,
    HDEVINFO, SPDRP_DEVICEDESC, SP_DEVINFO_DATA,
};
use windows::Win32::Devices::HumanInterfaceDevice::GUID_DEVINTERFACE_KEYBOARD;
use windows::Win32::Devices::Properties::{
    DEVPKEY_Device_Driver, DEVPKEY_Device_FriendlyName, DEVPROPTYPE, DEVPROP_TYPE_GUID,
    DEVPROP_TYPE_STRING,
};
use windows::Win32::Foundation::{GetLastError, ERROR_NO_MORE_ITEMS, HANDLE, MAX_PATH};
use windows::Win32::UI::Input::{
    GetRawInputDeviceInfoW, GetRawInputDeviceList, RAWINPUTDEVICELIST, RIDI_DEVICENAME,
};

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
                dev_info_data.cbSize = size_of::<SP_DEVINFO_DATA>() as u32;

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
    // obtem id de instancia
    let mut instance_id = [0u16; 256];
    let mut reqsize = 0u32;
    let result = unsafe {
        SetupDiGetDeviceInstanceIdW(
            device_info_set,
            dev_info_data,
            Some(&mut instance_id),
            Some(&mut reqsize),
        )
    };

    if result.is_ok() {
        let instance_id_str = String::from_utf16_lossy(&instance_id[..(reqsize as usize - 1)]);
        eprintln!("Instance ID: {}", instance_id_str);
    }

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
            let buffer = &buffer[..reqsize as usize];
            let mut vec_u16: Vec<u16> = Vec::new();
            let mut i = 0;
            while i < buffer.len() {
                vec_u16.push((buffer[i + 1] as u16) << 8 | buffer[i] as u16);
                i += 2; // Convert to u16 manually because it is utf16
            }
            let b = String::from_utf16_lossy(&vec_u16);
            let b = b.trim_end_matches('\0');

            Ok(b.to_string())
        }
        Err(e) => {
            eprintln!("Error {}", e.message());
            Err(e.message())
        }
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

fn get_raw_input_devices() -> Vec<HANDLE> {
    let mut num_devices: u32 = 0;

    // Primeiro, obtenha a quantidade de dispositivos disponíveis
    let result = unsafe {
        GetRawInputDeviceList(
            None,
            &mut num_devices,
            size_of::<RAWINPUTDEVICELIST>() as u32,
        )
    };

    if result != 0 {
        eprintln!("Erro ao obter a quantidade de dispositivos.");
        return vec![];
    }

    // Aloca um buffer para armazenar os dispositivos
    let mut device_list: Vec<RAWINPUTDEVICELIST> = vec![unsafe { zeroed() }; num_devices as usize];

    // Agora, obtenha a lista de dispositivos
    let result = unsafe {
        GetRawInputDeviceList(
            Some(device_list.as_mut_ptr()),
            &mut num_devices,
            size_of::<RAWINPUTDEVICELIST>() as u32,
        )
    };

    if result == u32::MAX {
        eprintln!("Erro ao obter a lista de dispositivos.");
        return vec![];
    }

    // Extrai os handles dos dispositivos
    device_list
        .into_iter()
        .filter(|device| device.dwType == RIM_TYPEKEYBOARD)
        .map(|device| device.hDevice)
        .collect()
}

fn get_raw_input_device_instance_id(device_handle: HANDLE) -> Option<String> {
    let mut size: u32 = 0;

    // Primeiro, obtenha o tamanho necessário do buffer
    let result =
        unsafe { GetRawInputDeviceInfoW(Some(device_handle), RIDI_DEVICENAME, None, &mut size) };

    if result != 0 {
        eprintln!(
            "Erro ao obter o tamanho do nome do dispositivo: {:?}",
            unsafe { GetLastError() }
        );
        return None;
    }

    // Alocar buffer com o tamanho necessário
    let mut buffer: Vec<u16> = vec![0; size as usize];

    // Agora, obtenha o nome real do dispositivo
    let result = unsafe {
        GetRawInputDeviceInfoW(
            Some(device_handle),
            RIDI_DEVICENAME,
            Some(buffer.as_mut_ptr() as *mut _),
            &mut size,
        )
    };

    if result == u32::MAX {
        eprintln!("Erro ao obter o nome do dispositivo: {:?}", unsafe {
            GetLastError()
        });
        return None;
    }
    let id = String::from_utf16(&buffer).unwrap();
    let id = format_device_path(&id);
    // regex para deixar no formato certo

    Some(id)
}

fn format_device_path(id: &str) -> String {
    let re = Regex::new(r"^\W{4}([^{]*)\{").unwrap();
    let cap = re.captures(&id).unwrap();

    cap[1].trim_end_matches("#").replace("#", r"\")
}

// fn main() {
//     let ids = get_keyboards_description();
//     println!("{:?}", ids);
//     let devices = get_raw_input_devices();
//     for device in devices {
//         match get_raw_input_device_instance_id(device) {
//             Some(id) => unsafe { println!("ID do dispositivo: {}", id) },
//             None => println!("Falha ao obter o nome do dispositivo."),
//         }
//     }
// }

unsafe extern "system" fn wind_proc(
    h_wnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if msg == WM_INPUT {
        let mut data_size: u32 = 0;
        if GetRawInputData(
            HRAWINPUT(l_param.0 as *mut c_void),
            RID_INPUT,
            None,
            &mut data_size,
            size_of::<RAWINPUTHEADER>() as u32,
        ) != 0
        {
            return DefWindowProcA(h_wnd, msg, w_param, l_param);
        }

        let mut buffer = vec![0u8; data_size as usize];

        if GetRawInputData(
            HRAWINPUT(l_param.0 as *mut c_void),
            RID_INPUT,
            Some(buffer.as_mut_ptr() as *mut c_void),
            &mut data_size,
            size_of::<RAWINPUTHEADER>() as u32,
        ) != data_size
        {
            return DefWindowProcA(h_wnd, msg, w_param, l_param);
        }

        let raw_input = &*(buffer.as_ptr() as *const RAWINPUT);

        if raw_input.header.dwType == RIM_TYPEKEYBOARD.0 {
            let keyboard = raw_input.data.keyboard;
            println!(
                "Tecla: {:?} | hDevice: {:?}",
                keyboard.VKey, raw_input.header.hDevice
            );
        }
    }

    DefWindowProcA(h_wnd, msg, w_param, l_param)
}

fn define_window_class(lpsz_class_name: PCSTR, h_instance: HINSTANCE) {
    let wcx: WNDCLASSEXA = WNDCLASSEXA {
        cbSize: size_of::<WNDCLASSEXA>() as u32,
        lpfnWndProc: Some(wind_proc),
        hInstance: h_instance,
        lpszClassName: lpsz_class_name,
        ..unsafe { zeroed() }
    };
    unsafe { RegisterClassExA(&wcx) };
    println!("Successfully registered class {:?}", wcx.lpszClassName);
}

fn create_window(lpsz_class_name: PCSTR, h_instance: HINSTANCE) -> Result<HWND, Error> {
    let h_wnd = unsafe {
        CreateWindowExA(
            Default::default(),
            lpsz_class_name,
            None,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            0,
            0,
            Some(HWND_MESSAGE),
            None,
            Some(h_instance),
            None,
        )
    }?;

    println!("Successfully created window {:?}", h_wnd);
    Ok(h_wnd)
}

fn register_raw_input_device(h_wnd: HWND) -> Result<(), Error> {
    let rid = RAWINPUTDEVICE {
        usUsagePage: 0x01,
        usUsage: 0x06, // keyboard
        dwFlags: RIDEV_INPUTSINK | RIDEV_NOLEGACY,
        hwndTarget: h_wnd,
    };

    unsafe {
        RegisterRawInputDevices(&[rid], size_of::<RAWINPUTDEVICE>() as u32)?;
    }
    println!("Successfully registered device");
    Ok(())
}

fn run_message_loop() {
    let mut msg: MSG = MSG::default();
    while unsafe { GetMessageA(&mut msg, Some(HWND::default()), 0, 0) } == true {
        unsafe {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}

fn main() {
    let lpsz_class_name = s!("RawInputClass");
    let h_instance = HINSTANCE::default();

    define_window_class(lpsz_class_name, h_instance);
    let h_wnd = create_window(lpsz_class_name, h_instance).expect("Failed to create window");
    register_raw_input_device(h_wnd).expect("Failed to register raw input device");
    run_message_loop();
}
