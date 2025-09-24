use cudf_sys::UniquePtr;

use crate::column::ColumnView;

/// See [`cudf_sys::table::Table`].
pub struct Table(UniquePtr<cudf_sys::table::Table>);

impl Default for Table {
    fn default() -> Self {
        Self(cudf_sys::table::Table::empty())
    }
}

impl Table {
    /// Returns the column at the given index.
    pub fn column(&self, index: usize) -> Option<ColumnView<'_>> {
        self.0.get_column_view(index as i32).map(ColumnView).ok()
    }

    /// Returns an iterator over the columns in this table.
    pub fn columns(&self) -> Columns<'_> {
        Columns {
            table: self,
            position: 0,
        }
    }

    /// Returns the allocation size of this table in bytes.
    pub fn alloc_bytes(&self) -> usize {
        self.0.alloc_size().unwrap_or_else(|e| panic!("{e}")) as usize
    }

    /// Returns the number of columns in this table.
    pub fn columns_len(&self) -> usize {
        self.0.num_columns() as usize
    }

    /// Returns the number of rows in this table.
    pub fn len(&self) -> usize {
        self.0.num_rows() as usize
    }

    /// Returns `true` if there are no rows in this table.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// An iterator over the columns in a table.
///
/// See [`Table::columns`].
pub struct Columns<'table> {
    table: &'table Table,
    position: usize,
}

impl<'table> Iterator for Columns<'table> {
    type Item = ColumnView<'table>;

    fn next(&mut self) -> Option<Self::Item> {
        self.table.column(self.position).inspect(|_| {
            self.position += 1;
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.table.len() - self.position;
        (remaining, Some(remaining))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let table = Table::default();
        assert!(table.is_empty());
        assert_eq!(table.columns_len(), 0);
        assert!(table.column(0).is_none());
        assert_eq!(table.columns().size_hint(), (0, Some(0)));
        assert!(table.columns().next().is_none());
        assert_eq!(table.alloc_bytes(), 0);
    }
}
