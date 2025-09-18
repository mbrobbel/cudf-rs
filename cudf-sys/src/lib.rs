#[cxx::bridge(namespace = "cudf_rs")]
mod ffi {
    unsafe extern "C++" {
        include!("cudf-sys/include/cudf_rs.hpp");

        #[namespace = "cudf"]
        type table_view;

        type Table;
        fn num_rows(self: &Table) -> i32;
        fn num_columns(self: &Table) -> i32;
        fn alloc_size(self: &Table) -> Result<i32>;
        fn view(self: &Table) -> Result<&table_view>;
        fn read_csv_to_table(path: &str) -> Result<UniquePtr<Table>>;
    }
}

unsafe impl Send for ffi::Table {}
unsafe impl Sync for ffi::Table {}

pub use ffi::*;
