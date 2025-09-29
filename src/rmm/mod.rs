/// A cuda device identifier.
pub struct DeviceId(cudf_sys::rmm::DeviceId);

impl DeviceId {
    /// Return the device id of the current deivce.
    ///
    /// The current device is the device on which the calling thread executes
    /// device code.
    pub fn get_current() -> Self {
        Self(cudf_sys::rmm::DeviceId::get_current())
    }

    /// Returns the value of this device id.
    pub fn value(&self) -> i32 {
        self.0.value()
    }
}

/// Returns the number of CUDA devices in the system.
pub fn get_num_devices() -> i32 {
    cudf_sys::rmm::get_num_devices()
}

/// Returns the available device memory in bytes for the current device.
pub fn available_device_memory() -> usize {
    cudf_sys::rmm::available_device_memory().unwrap_or_else(|e| panic!("{e}"))
}

/// Returns the total device memory in bytes for the current device.
pub fn total_device_memory() -> usize {
    cudf_sys::rmm::total_device_memory().unwrap_or_else(|e| panic!("{e}"))
}

/// Returns the approximate specified percent of available device memory on the
/// current CUDA device, aligned (down) to the nearest CUDA allocation size.
pub fn percent_of_free_device_memory(percent: i32) -> usize {
    cudf_sys::rmm::percent_of_free_device_memory(percent).unwrap_or_else(|e| panic!("{e}"))
}
