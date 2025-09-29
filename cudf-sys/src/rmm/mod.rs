#[repr(C)]
#[allow(non_camel_case_types)]
pub struct DeviceId {
    id_: i32,
}

unsafe impl ExternType for DeviceId {
    type Id = cxx::type_id!("cudf_rs::DeviceId");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge]
mod ffi {
    #[namespace = "rmm"]
    unsafe extern "C++" {
        include!("rmm/cuda_device.hpp");

        #[rust_name = "get_num_devices"]
        fn get_num_cuda_devices() -> i32;
        fn percent_of_free_device_memory(percent: i32) -> Result<usize>;
    }

    #[namespace = "cudf_rs"]
    unsafe extern "C++" {
        include!("cudf-sys/include/rmm/mod.hpp");

        type DeviceId = crate::rmm::DeviceId;
        #[Self = DeviceId]
        fn get_current() -> DeviceId;
        fn value(self: &DeviceId) -> i32;

        // Splitting this up: https://github.com/dtolnay/cxx/issues/943
        fn available_device_memory() -> Result<usize>;
        fn total_device_memory() -> Result<usize>;
    }
}

use cxx::ExternType;
pub use ffi::*;
