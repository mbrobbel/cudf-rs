#pragma once
#include <memory>
#include <mutex>

#include <cudf/types.hpp>

namespace cudf {
class column;
class column_view;
} // namespace cudf

namespace cudf_rs {
class Column {
public:
  explicit Column(std::unique_ptr<cudf::column> column);
  ~Column();

  cudf::size_type size() const noexcept;
  cudf::column_view const &view() const;

private:
  std::unique_ptr<cudf::column> column_;
  mutable std::once_flag column_view_once_;
  mutable std::unique_ptr<cudf::column_view> column_view_;
};

// To support UniquePtr<Column>
void _column(std::unique_ptr<Column>);

} // namespace cudf_rs
