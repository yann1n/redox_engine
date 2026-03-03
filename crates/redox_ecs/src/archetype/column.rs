use std::alloc::{self, Layout};
use std::ptr::{self, NonNull};
use crate::component::ComponentInfo;

/// A column of component data stored in a dense array.
pub struct Column {
    info: ComponentInfo,
    data: NonNull<u8>,
    len: usize,
    cap: usize,
}

// SAFETY: Column stores raw bytes and metadata. Immutable access from multiple threads
// is safe because data is never mutated after creation (except during controlled moves).
unsafe impl Send for Column {}
unsafe impl Sync for Column {}

impl Column {
    /// Creates a new empty column for the given component type.
    pub fn new(info: ComponentInfo) -> Self {
        Self {
            info,
            data: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    /// Pushes a component value onto the column.
    ///
    /// # Safety
    /// `component_ptr` must point to a valid instance of the component type.
    pub unsafe fn push(&mut self, component_ptr: *const u8) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            let dst = self.data.as_ptr().add(self.len * self.info.size);
            ptr::copy_nonoverlapping(component_ptr, dst, self.info.size);
        }
        self.len += 1;
    }

    /// Removes the element at `index` by swapping with the last element.
    ///
    /// Returns `true` if the last element was moved into the vacated slot.
    ///
    /// # Safety
    /// `index` must be within bounds.
    pub unsafe fn swap_remove(&mut self, index: usize, _drop: bool) -> bool {
        let size = self.info.size;
        let p = self.data.as_ptr();
        let last_index = self.len - 1;

        if index != last_index {
            unsafe {
                ptr::copy_nonoverlapping(p.add(last_index * size), p.add(index * size), size);
            }
            self.len -= 1;
            true
        } else {
            self.len -= 1;
            false
        }
    }

    /// Returns a raw pointer to the element at `index`.
    ///
    /// # Safety
    /// `index` must be within bounds.
    pub unsafe fn get(&self, index: usize) -> *mut u8 {
        unsafe {
            self.data.as_ptr().add(index * self.info.size)
        }
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (8, Layout::from_size_align(8 * self.info.size, self.info.align).unwrap())
        } else {
            let new_cap = self.cap * 2;
            let new_layout = Layout::from_size_align(new_cap * self.info.size, self.info.align).unwrap();
            (new_cap, new_layout)
        };

        let new_ptr = unsafe {
            if self.cap == 0 {
                alloc::alloc(new_layout)
            } else {
                let old_layout = Layout::from_size_align(self.cap * self.info.size, self.info.align).unwrap();
                alloc::realloc(self.data.as_ptr(), old_layout, new_layout.size())
            }
        };

        self.data = NonNull::new(new_ptr).expect("Allocation failed");
        self.cap = new_cap;
    }

    /// Returns the number of elements in the column.
    pub fn len(&self) -> usize {
        self.len
    }
}

impl Drop for Column {
    fn drop(&mut self) {
        if self.cap > 0 {
            let layout = Layout::from_size_align(self.cap * self.info.size, self.info.align).unwrap();
            unsafe {
                alloc::dealloc(self.data.as_ptr(), layout);
            }
        }
    }
}