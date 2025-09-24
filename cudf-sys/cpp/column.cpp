#include "column.hpp"

#include <cudf/column/column.hpp>
#include <cudf/column/column_view.hpp>

namespace cudf_rs {

Column::Column(std::unique_ptr<cudf::column> column)
    : column_(std::move(column)) {}

Column::~Column() = default;

cudf::size_type Column::size() const noexcept { return column_->size(); };

cudf::column_view const &Column::view() const {
  std::call_once(column_view_once_, [this] {
    column_view_ = std::make_unique<cudf::column_view>(column_->view());
  });
  return *column_view_;
}

} // namespace cudf_rs
