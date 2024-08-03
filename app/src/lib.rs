use std::{ffi::c_void, mem::transmute, ptr};

use windows::{
    core::{Param, HRESULT, PCWSTR},
    Win32::Devices::{
        Enumeration::Pnp::{SwDeviceClose, SwDeviceCreate, HSWDEVICE, SW_DEVICE_CREATE_INFO},
        Properties::DEVPROPERTY,
    },
};

pub struct SwDevice {
    sw_device_handler: HSWDEVICE,
}

impl SwDevice {
    pub unsafe fn try_new<CTX>(
        enumerator_name: impl Param<PCWSTR>,
        parent_device_instance: impl Param<PCWSTR>,
        create_info: &SW_DEVICE_CREATE_INFO,
        properties: Option<&[DEVPROPERTY]>,
        callback: Option<unsafe extern "system" fn(HSWDEVICE, HRESULT, *const CTX, PCWSTR)>,
        context: Option<&CTX>,
    ) -> Result<Self, windows::core::Error> {
        unsafe {
            let sw = SwDeviceCreate(
                enumerator_name,
                parent_device_instance,
                ptr::from_ref(create_info),
                properties,
                transmute(callback), /* ! only use to change *const CTX to *const c_void ! */
                context.map(|it| it as *const CTX as *const c_void),
            )?;
            Ok(Self {
                sw_device_handler: sw,
            })
        }
    }
}

impl Drop for SwDevice {
    fn drop(&mut self) {
        unsafe {
            // Stop the device, this will cause the sample to be unloaded
            SwDeviceClose(self.sw_device_handler)
        }
    }
}
