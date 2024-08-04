#![no_std]

mod util;
#[cfg(not(test))]
extern crate wdk_panic;

use core::ptr::{null, null_mut, NonNull};

#[cfg(not(test))]
use wdk_alloc::WDKAllocator;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WDKAllocator = WDKAllocator;

use wdk::nt_success;
use wdk_sys::{
    WdfDriverGlobals, WdfFunctions_01033, DRIVER_OBJECT, NTSTATUS, PCUNICODE_STRING, PFN_WDFDRIVERCREATE, PFN_WDF_DRIVER_DEVICE_ADD, WDFDEVICE_INIT, WDFDRIVER, WDF_DRIVER_CONFIG, WDF_OBJECT_ATTRIBUTES, WDF_PNPPOWER_EVENT_CALLBACKS, _WDFFUNCENUM::WdfDriverCreateTableIndex, _WDF_EXECUTION_LEVEL::WdfExecutionLevelInheritFromParent, _WDF_SYNCHRONIZATION_SCOPE::WdfSynchronizationScopeInheritFromParent
};

struct NtStatus(NTSTATUS);

type NtResult<T> = core::result::Result<T, NtStatus>;

struct WdfObjectAttr(WDF_OBJECT_ATTRIBUTES);

impl WdfObjectAttr {
    unsafe fn new() -> Self {
        Self(WDF_OBJECT_ATTRIBUTES {
            Size: wdf_struct_size!(WDF_OBJECT_ATTRIBUTES) as u32,
            EvtCleanupCallback: None,
            EvtDestroyCallback: None,
            ExecutionLevel: WdfExecutionLevelInheritFromParent,
            SynchronizationScope: WdfSynchronizationScopeInheritFromParent,
            ParentObject: null_mut(),
            ContextSizeOverride: 0,
            ContextTypeInfo: null(),
        })
    }
}

struct WdfDriverConfig(WDF_DRIVER_CONFIG);

impl WdfDriverConfig {
    unsafe fn new(driver_device_add: PFN_WDF_DRIVER_DEVICE_ADD) -> Self {
        Self(WDF_DRIVER_CONFIG {
            Size: wdf_struct_size!(WDF_DRIVER_CONFIG),
            EvtDriverDeviceAdd: driver_device_add,
            EvtDriverUnload: None,
            DriverInitFlags: 0,
            DriverPoolTag: 0,
        })
    }
}

struct WdfDriver(WDFDRIVER);

impl WdfDriver {
    unsafe fn try_new(
        driver_object: &mut DRIVER_OBJECT,
        registry_path: PCUNICODE_STRING,
        driver_attr: &mut WDF_OBJECT_ATTRIBUTES,
        driver_config: &mut WDF_DRIVER_CONFIG,
    ) -> NtResult<Self> {
        // FIXME: WdfFunctions should not hard encode version
        let wdf_driver_create = *WdfFunctions_01033
            .add(WdfDriverCreateTableIndex as usize)
            .cast::<PFN_WDFDRIVERCREATE>();
        let mut driver: WDFDRIVER = null_mut();
        let status = wdf_driver_create.unwrap()(
            WdfDriverGlobals,
            driver_object,
            registry_path,
            driver_attr,
            driver_config,
            &mut driver,
        );
        if nt_success(status) {
            Ok(Self(driver))
        } else {
            Err(NtStatus(status))
        }
    }
}

struct WdfPnpPowerEventCallback(WDF_PNPPOWER_EVENT_CALLBACKS);

impl WdfPnpPowerEventCallback {
    unsafe fn new() -> Self {
        Self(WDF_PNPPOWER_EVENT_CALLBACKS {
            Size: wdf_struct_size!(WDF_PNPPOWER_EVENT_CALLBACKS),
            ..Default::default()
        })
    }
}

unsafe extern "C" fn device_add(_driver: WDFDRIVER, device_init: *mut WDFDEVICE_INIT) -> NTSTATUS {
    let mut pnp_power_callbacks = WdfPnpPowerEventCallback::new();
    pnp_power_callbacks.0.EvtDeviceD0Entry = None;
    todo!();
    0
}

unsafe fn driver_main(driver_object: &mut DRIVER_OBJECT, registry_path: PCUNICODE_STRING) -> NtResult<()> {
    let mut config = WdfDriverConfig::new(Some(device_add));
    let mut attrs = WdfObjectAttr::new();
    //let driver = WdfDriver::try_new(driver_object, registry_path, &mut attrs.0, &mut config.0);

    Ok(())
}

#[export_name = "DriverEntry"] // WDF expects a symbol with the name DriverEntry
pub unsafe extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    match driver_main(driver, registry_path) {
        Ok(()) => 0,
        Err(status) => status.0,
    }
}
