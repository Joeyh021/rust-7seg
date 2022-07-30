use riscv::singleton;
use volatile_register::*;

const DISPLAY_ADDR: usize = 0x60040000;

#[repr(C)]
pub struct Display(&'static mut [RW<u32>; 8]);

impl Display {
    pub unsafe fn new() -> Self {
        Display((DISPLAY_ADDR as *mut [RW<u32>; 8]).as_mut().unwrap())
    }

    pub fn clear(&mut self) {
        self.set_all(0);
    }

    pub fn set_all(&mut self, num: u32) {
        for i in self.0.iter_mut() {
            unsafe { i.write(num) };
        }
    }

    pub fn get(&self, idx: usize) -> u32 {
        self.0.get(idx).unwrap().read()
    }

    pub fn set(&self, idx: usize, val: u32) {
        unsafe { self.0.get(idx).unwrap().write(val) }
    }
    pub fn take() -> Option<&'static mut Self> {
        singleton!(:Display = unsafe{Display::new()})
    }
}
