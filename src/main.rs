use std::mem;
use std::mem::zeroed;
use windows::Win32::Devices::DeviceAndDriverInstallation::{DIGCF_ALLCLASSES, DIGCF_PRESENT, SP_DEVINFO_DATA, SetupDiDestroyDeviceInfoList, SetupDiEnumDeviceInfo, SetupDiGetClassDevsW, SetupDiGetDevicePropertyW, DIGCF_DEVICEINTERFACE};
use windows::Win32::Devices::Properties::{
    DEVPKEY_Device_FriendlyName, DEVPROP_TYPE_DEVPROPKEY, DEVPROPTYPE,
};
use windows::Win32::Foundation::*;
use windows::Win32::Foundation::{GetLastError, HANDLE};
use windows::Win32::UI::Input::{
    GetRawInputDeviceInfoW, GetRawInputDeviceList, RAWINPUTDEVICELIST, RIDI_DEVICENAME,
    RIM_TYPEKEYBOARD,
};
use windows::core::{Error, HRESULT, PCWSTR};
use windows::Win32::Devices::HumanInterfaceDevice::GUID_DEVINTERFACE_KEYBOARD;

fn get_device_friendly_name(device_path: &Vec<u16>) -> String {
    // 1. Obter o nome do dispositivo (ex: \\?\HID#VID_046D&PID_C53F&MI_00#7&316c6a2f&0&0000#{884b96c3-56ef-11d1-bc8c-00a0c91405dd})
    // 2. Extrair o ID do dispositivo do caminho
    let device_id = extract_device_id(device_path);

    // 3. Obter a descrição amigável usando SetupDi
    let device_id = r"HID\VID_046D&PID_C53D&MI_00"
        .encode_utf16()
        .collect::<Vec<u16>>();
    get_friendly_name_from_device_id(device_id);
    "".into()
}
fn extract_device_id(device_path: &Vec<u16>) -> Vec<u16> {
    // O formato é algo como: \\?\HID#VID_046D&PID_C53F&MI_00#7&316c6a2f&0&0000#{884b96c3-56ef-11d1-bc8c-00a0c91405dd}
    // Queremos extrair: HID\VID_046D&PID_C53F&MI_00
    let device_path = String::from_utf16(device_path).unwrap();
    println!("{}", device_path);
    let device_path = device_path
        .trim_start_matches(r"\\?\")
        .split('#')
        .take(2)
        .collect::<Vec<_>>()
        .join("\\");
    println!("{}", device_path);
    vec![1, 2]
}

pub fn get_friendly_name_from_device_id(device_id: Vec<u16>) -> String {
    unsafe {
        let device_info = SetupDiGetClassDevsW(
            Some(&GUID_DEVINTERFACE_KEYBOARD),
            Some(&PCWSTR(device_id.as_ptr())),
            None,
            DIGCF_DEVICEINTERFACE | DIGCF_ALLCLASSES,
        )
        .unwrap();
        check_win32_error().unwrap();

        let mut device_data = SP_DEVINFO_DATA::default();
        device_data.cbSize = size_of::<SP_DEVINFO_DATA>() as u32;

        if SetupDiEnumDeviceInfo(device_info, 0, &mut device_data).is_err() {
            SetupDiDestroyDeviceInfoList(device_info).unwrap();
            check_win32_error().unwrap();
            panic!("erro");
        }

        let mut required_size = 0u32;
        let mut prop_type = DEVPROPTYPE::default();
        SetupDiGetDevicePropertyW(
            device_info,
            &device_data,
            &DEVPKEY_Device_FriendlyName,
            &mut prop_type,
            None,
            Some(&mut required_size),
            0,
        )
        .unwrap();

        let mut buffer = vec![0; required_size as usize];

        let success = SetupDiGetDevicePropertyW(
            device_info,
            &device_data,
            &DEVPKEY_Device_FriendlyName,
            &mut prop_type,
            Some(&mut buffer),
            None,
            0,
        );

        SetupDiDestroyDeviceInfoList(device_info).unwrap();
        let success = success.unwrap();
        String::from_utf8_lossy(&buffer).to_string()
    }
}

pub unsafe fn check_win32_error() -> windows::core::Result<()> {
    let err = Error::from_win32();
    if err.code() == HRESULT(0) {
        Ok(())
    } else {
        Err(err)
    }
}

fn get_raw_input_device_name(device_handle: HANDLE) -> Option<Vec<u16>> {
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

    Some(buffer)
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

fn main() {
    let devices = get_raw_input_devices();
    for device in devices {
        match get_raw_input_device_name(device) {
            Some(mut name) => unsafe {
                let tst = get_device_friendly_name(&name);
                println!(
                    "Nome do dispositivo: {}",
                    String::from_utf16(&name).unwrap()
                )
            },
            None => println!("Falha ao obter o nome do dispositivo."),
        }
    }
}
