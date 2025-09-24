use cudf_sys::UniquePtr;

/// See [`cudf_sys::column::column_view`].
pub struct ColumnView<'column>(pub(crate) &'column cudf_sys::column::column_view);

impl ColumnView<'_> {
    /// Returns the number of rows in this column
    pub fn len(&self) -> usize {
        self.0.size() as usize
    }

    /// Returns `true` if there are no rows in this column.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// See [`cudf_sys::column::Column`].
pub struct Column(UniquePtr<cudf_sys::column::Column>);

impl Column {
    /// Returns the number of rows in this column
    pub fn len(&self) -> usize {
        self.0.size() as usize
    }

    /// Returns `true` if there are no rows in this column.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
