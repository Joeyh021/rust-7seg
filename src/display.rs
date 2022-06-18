use volatile_register::*;

const DISPLAY_ADDR: usize = 0x60040000;

pub struct Display(&'static mut Digits);

#[repr(C)]
struct Digits(
    RW<u32>,
    RW<u32>,
    RW<u32>,
    RW<u32>,
    RW<u32>,
    RW<u32>,
    RW<u32>,
    RW<u32>,
);

impl Display {
    pub fn new() -> Self {
        Display(unsafe { (DISPLAY_ADDR as *mut Digits).as_mut().unwrap() })
    }

    pub fn clear(&mut self) {
        self.set_all(0);
    }

    pub fn set_all(&mut self, num: u32) {
        unsafe {
            self.0 .0.write(num);
            self.0 .1.write(num);
            self.0 .2.write(num);
            self.0 .3.write(num);
            self.0 .4.write(num);
            self.0 .5.write(num);
            self.0 .6.write(num);
            self.0 .7.write(num);
        }
    }
}

//todo: indexmut impl
