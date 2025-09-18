#include "cudf_rs.hpp"

#include <cudf/io/csv.hpp>
#include <cudf/io/types.hpp>
#include <cudf/table/table.hpp>
#include <cudf/table/table_view.hpp>

namespace cudf_rs {

Table::Table(std::unique_ptr<cudf::table> table) : table_(std::move(table)) {}
Table::~Table() = default;

cudf::size_type Table::num_columns() const noexcept {
  return table_->num_columns();
};
cudf::size_type Table::num_rows() const noexcept { return table_->num_rows(); };
cudf::size_type Table::alloc_size() const { return table_->alloc_size(); };

cudf::table_view const &Table::view() const {
  std::call_once(table_view_once_, [this] {
    table_view_ = std::make_unique<cudf::table_view>(table_->view());
  });
  return *table_view_;
}

std::unique_ptr<Table> read_csv_to_table(rust::Str path) {
  std::string input_path = std::string(path);
  cudf::io::source_info src = cudf::io::source_info(input_path);
  cudf::io::csv_reader_options options =
      cudf::io::csv_reader_options::builder(src).build();
  cudf::io::table_with_metadata result = cudf::io::read_csv(options);
  return std::make_unique<Table>(std::move(result.tbl));
}

} // namespace cudf_rs
