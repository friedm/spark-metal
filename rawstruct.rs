use core::mem;
use core::ptr;
use core::clone::Clone;

pub trait RawStruct<T: Clone> {
    fn from_mem(base: usize) -> T {
       let mut ptr = base as *mut T;
       unsafe {
           (*ptr).clone()
       }
   }

   fn to_struct(&self) -> T;
   fn write(&self, write_to: usize) {
       let mut ptr = write_to as *mut T;
       unsafe {
           ptr::write(ptr, Self::to_struct(self))
       }
   }
}
