#[macro_export]
macro_rules! wdf_struct_size {
    ($StructName:ident) => {{
        use paste::paste;
        use wdk_sys::{WdfClientVersionHigherThanFramework, WdfStructureCount, WdfStructures,_WDFSTRUCTENUM::*};
        if WdfClientVersionHigherThanFramework != 0 {
            let idx = paste!([<INDEX_ $StructName>]) as u32;
            if idx < WdfStructureCount {
                unsafe { *WdfStructures.add(idx as usize) as u32 }
            } else {
                u32::MAX
            }
        } else {
            core::mem::size_of::<$StructName>() as u32
        }
    }};
}
