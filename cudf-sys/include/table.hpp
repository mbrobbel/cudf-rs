#pragma once
#include <memory>
#include <mutex>

#include <cudf/types.hpp>

namespace cudf {
class column;
class column_view;
class table;
class table_view;
} // namespace cudf

namespace cudf_rs {
class Table {
public:
  explicit Table(std::unique_ptr<cudf::table> table);
  ~Table();

  static std::unique_ptr<Table> empty() noexcept;

  cudf::size_type alloc_size() const;
  cudf::column const &get_column(cudf::size_type column_index) const;
  cudf::column_view const &get_column_view(cudf::size_type column_index) const;
  cudf::size_type num_columns() const noexcept;
  cudf::size_type num_rows() const noexcept;
  cudf::table_view const &view() const;

private:
  std::unique_ptr<cudf::table> table_;
  mutable std::once_flag table_view_once_;
  mutable std::unique_ptr<cudf::table_view> table_view_;
};
} // namespace cudf_rs
