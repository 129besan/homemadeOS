use core::cell::UnsafeCell;

pub struct Once<T> {
    initialized: bool,
    data: UnsafeCell<core::mem::MaybeUninit<T>>,
}

unsafe impl<T: Send> Sync for Once<T> {}

impl<T> Once<T> {
    pub const fn new() -> Self {
        Once {
            initialized: false,
            data: UnsafeCell::new(core::mem::MaybeUninit::uninit()),
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn call_once<F>(&mut self, f: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        if !self.initialized {
            let val = f();
            unsafe {
                (*self.data.get()).as_mut_ptr().write(val);
            }
            self.initialized = true;
        }
        unsafe { &mut *(*self.data.get()).as_mut_ptr() }
    }

    pub fn get(&self) -> Option<&T> {
        if self.initialized {
            unsafe { Some(&*(*self.data.get()).as_mut_ptr()) }
        } else {
            None
        }
    }
}
