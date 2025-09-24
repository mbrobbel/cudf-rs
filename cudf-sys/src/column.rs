#[cxx::bridge]
mod ffi {
    #[namespace = "cudf"]
    unsafe extern "C++" {
        include!("cudf/column/column.hpp");

        type column;
        fn has_nulls(self: &column) -> bool;

        type column_view;
        fn size(self: &column_view) -> i32;
    }

    #[namespace = "cudf_rs"]
    unsafe extern "C++" {
        include!("cudf-sys/include/column.hpp");

        type Column;

        fn size(self: &Column) -> i32;
        fn view(self: &Column) -> Result<&column_view>;

        // To support UniquePtr<Column>
        fn _column(_: UniquePtr<Column>);
    }
}

pub use ffi::*;

unsafe impl Send for ffi::Column {}
unsafe impl Sync for ffi::Column {}
