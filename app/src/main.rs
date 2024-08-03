#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case)]
use app::SwDevice;
use std::mem::size_of_val;
use windows::{
    core::{w, HRESULT, PCWSTR},
    Win32::{
        Devices::Enumeration::Pnp::{
            SWDeviceCapabilitiesDriverRequired, SWDeviceCapabilitiesRemovable,
            SWDeviceCapabilitiesSilentInstall, HSWDEVICE, SW_DEVICE_CREATE_INFO,
        },
        Foundation::{HANDLE, WAIT_OBJECT_0},
        System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
    },
};

unsafe extern "system" fn CreationCallback(
    _hSwDevice: HSWDEVICE,
    _hrCreateResult: HRESULT,
    pContext: *const HANDLE,
    _pszDeviceInstanceId: PCWSTR,
) {
    let hEvent = unsafe { *pContext };

    unsafe { SetEvent(hEvent) }.unwrap();
}

fn main() -> anyhow::Result<()> {
    let hEvent = unsafe { CreateEventW(None, false, false, None)? };
    let mut createInfo = SW_DEVICE_CREATE_INFO::default();
    let description = w!("Idd Sample Driver");

    // These match the Pnp id's in the inf file so OS will load the driver when the device is created
    let instanceId = w!("IddSampleDriver");
    let hardwareIds = w!("IddSampleDriver\0\0");
    let compatibleIds = w!("IddSampleDriver\0\0");

    createInfo.cbSize = size_of_val(&createInfo) as u32;
    createInfo.pszzCompatibleIds = compatibleIds;
    createInfo.pszInstanceId = instanceId;
    createInfo.pszzHardwareIds = hardwareIds;
    createInfo.pszDeviceDescription = description;

    createInfo.CapabilityFlags = (SWDeviceCapabilitiesRemovable.0
        | SWDeviceCapabilitiesSilentInstall.0
        | SWDeviceCapabilitiesDriverRequired.0) as u32;

    // Create the device
    let _hr: SwDevice = unsafe {
        SwDevice::try_new(
            w!("IddSampleDriver"),
            w!("HTREE\\ROOT\\0"),
            &createInfo,
            None,
            Some(CreationCallback),
            Some(&hEvent),
        )?
    };

    // Wait for callback to signal that the device has been created
    println!("Waiting for device to be created....");
    let waitResult = unsafe { WaitForSingleObject(hEvent, 10 * 1000) };
    if waitResult != WAIT_OBJECT_0 {
        return Err(anyhow::anyhow!("Wait for device creation failed"));
    }
    println!("Device created\n");

    loop {}
}
