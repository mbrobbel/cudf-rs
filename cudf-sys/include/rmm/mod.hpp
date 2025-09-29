#pragma once

#include <rmm/cuda_device.hpp>

namespace cudf_rs {
struct DeviceId {
  explicit DeviceId(rmm::cuda_device_id id) : id_(id) {}
  static DeviceId get_current();
  std::int32_t value() const;

private:
  rmm::cuda_device_id id_;
};

std::size_t available_device_memory();
std::size_t total_device_memory();

} // namespace cudf_rs
