pub mod bindings;

pub struct Tables {
    inner: Box<bindings::table_collection>,
}

impl Tables {
    pub fn new(value: i32) -> Self {
        let temp: std::mem::MaybeUninit<bindings::table_collection> =
            std::mem::MaybeUninit::uninit();

        let mut rv = Self {
            inner: unsafe { Box::<bindings::table_collection>::new(temp.assume_init()) },
        };

        rv.inner.table_data = value;

        rv
    }
}

pub struct TreeSeq {
    inner: Box<bindings::tree_sequence>,
}

impl TreeSeq {
    pub fn new(tables: Tables) -> Self {
        let temp: std::mem::MaybeUninit<bindings::tree_sequence> = std::mem::MaybeUninit::uninit();

        let mut rv = Self {
            inner: unsafe { Box::<bindings::tree_sequence>::new(temp.assume_init()) },
        };
        let tables_ptr: *mut bindings::table_collection = Box::into_raw(tables.inner);

        unsafe {
            bindings::init_tree_sequence(tables_ptr, &mut *rv.inner);
        }

        rv
    }

    pub fn data(&self) -> i32 {
        unsafe { (*self.inner.tables).table_data }
    }
}

impl Drop for TreeSeq {
    fn drop(&mut self) {
        unsafe {
            bindings::free_tree_sequence(&mut *self.inner);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer() {
        let tables = Tables::new(101);
        let trees = TreeSeq::new(tables);
        assert_eq!(trees.data(), 101);
    }
}
