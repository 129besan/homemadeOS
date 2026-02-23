use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

pub struct IrqSafeSpinLock<T> {
    locked: bool,
    data: UnsafeCell<T>,
}

pub struct IrqSafeGuard<'a, T> {
    lock: &'a mut IrqSafeSpinLock<T>,
    flags: u64,
}

unsafe impl<T: Send> Send for IrqSafeSpinLock<T> {}
unsafe impl<T: Send> Sync for IrqSafeSpinLock<T> {}

impl<T> IrqSafeSpinLock<T> {
    pub const fn new(data: T) -> Self {
        IrqSafeSpinLock {
            locked: false,
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&mut self) -> IrqSafeGuard<T> {
        let flags: u64;
        unsafe {
            core::arch::asm!("pushfq; pop {}", out(reg) flags);
            core::arch::asm!("cli");
        }
        while self.locked {
            core::hint::spin_loop();
        }
        self.locked = true;
        IrqSafeGuard { lock: self, flags }
    }
}

impl<'a, T> Deref for IrqSafeGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> DerefMut for IrqSafeGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T> Drop for IrqSafeGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked = false;
        if self.flags & 0x200 != 0 {
            unsafe { core::arch::asm!("sti"); }
        }
    }
}
