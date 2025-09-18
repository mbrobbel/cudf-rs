#pragma once
#include <memory>
#include <mutex>

#include <cudf/types.hpp>

#include "rust/cxx.h"

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

  cudf::size_type num_rows() const noexcept;
  cudf::size_type num_columns() const noexcept;
  cudf::size_type alloc_size() const;

  cudf::table_view const &view() const;

private:
  std::unique_ptr<cudf::table> table_;
  mutable std::once_flag table_view_once_;
  mutable std::unique_ptr<cudf::table_view> table_view_;
};

std::unique_ptr<Table> read_csv_to_table(rust::Str path);

} // namespace cudf_rs
