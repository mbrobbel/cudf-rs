#[cxx::bridge]
mod ffi {
    #[namespace = "cudf"]
    #[repr(i32)]
    enum TypeId {
        EMPTY,
        INT8,
        INT16,
        INT32,
        INT64,
        UINT8,
        UINT16,
        UINT32,
        UINT64,
        FLOAT32,
        FLOAT64,
        BOOL8,
        TIMESTAMP_DAYS,
        TIMESTAMP_SECONDS,
        TIMESTAMP_MILLISECONDS,
        TIMESTAMP_MICROSECONDS,
        TIMESTAMP_NANOSECONDS,
        DURATION_DAYS,
        DURATION_SECONDS,
        DURATION_MILLISECONDS,
        DURATION_MICROSECONDS,
        DURATION_NANOSECONDS,
        DICTIONARY32,
        STRING,
        LIST,
        DECIMAL32,
        DECIMAL64,
        DECIMAL128,
        STRUCT,
        NUM_TYPE_IDS,
    }
}

pub use ffi::*;

pub mod rmm;

pub mod column;
pub mod table;

pub use cxx::{Exception, UniquePtr};
