#[cxx::bridge]
mod ffi {
    #[namespace = "cudf"]
    unsafe extern "C++" {
        type column = crate::column::column;
        type column_view = crate::column::column_view;
        type table_view;
    }

    #[namespace = "cudf_rs"]
    unsafe extern "C++" {
        include!("cudf-sys/include/table.hpp");

        type Table;

        #[Self = "Table"]
        fn empty() -> UniquePtr<Table>;

        fn alloc_size(self: &Table) -> Result<i32>;
        fn get_column(self: &Table, column_index: i32) -> Result<&column>;
        fn get_column_view(self: &Table, column_index: i32) -> Result<&column_view>;
        fn num_columns(self: &Table) -> i32;
        fn num_rows(self: &Table) -> i32;
        fn view(self: &Table) -> Result<&table_view>;
    }
}

pub use ffi::*;

unsafe impl Send for ffi::Table {}
unsafe impl Sync for ffi::Table {}
