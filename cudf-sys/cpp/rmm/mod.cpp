#include "rmm/mod.hpp"

namespace cudf_rs {
DeviceId DeviceId::get_current() {
  return DeviceId{rmm::get_current_cuda_device()};
}

int32_t DeviceId::value() const { return id_.value(); }

std::size_t available_device_memory() {
  return rmm::available_device_memory().first;
}

std::size_t total_device_memory() {
  return rmm::available_device_memory().second;
}

} // namespace cudf_rs
