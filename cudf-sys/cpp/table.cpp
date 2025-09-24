#include "table.hpp"

#include <cudf/column/column.hpp>
#include <cudf/column/column_view.hpp>
#include <cudf/table/table.hpp>
#include <cudf/table/table_view.hpp>

namespace cudf_rs {

Table::Table(std::unique_ptr<cudf::table> table) : table_(std::move(table)) {}

Table::~Table() = default;

std::unique_ptr<Table> Table::empty() noexcept {
  std::vector<std::unique_ptr<cudf::column>> empty(0);
  std::unique_ptr<cudf::table> table =
      std::make_unique<cudf::table>(std::move(empty));
  return std::make_unique<Table>(std::move(table));
}

cudf::size_type Table::alloc_size() const { return table_->alloc_size(); };

cudf::column const &Table::get_column(cudf::size_type column_index) const {
  return table_->get_column(column_index);
}

cudf::column_view const &
Table::get_column_view(cudf::size_type column_index) const {
  return this->view().column(column_index);
}

cudf::size_type Table::num_columns() const noexcept {
  return table_->num_columns();
};

cudf::size_type Table::num_rows() const noexcept { return table_->num_rows(); };

cudf::table_view const &Table::view() const {
  std::call_once(table_view_once_, [this] {
    table_view_ = std::make_unique<cudf::table_view>(table_->view());
  });
  return *table_view_;
}

} // namespace cudf_rs
