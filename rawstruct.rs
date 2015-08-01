use core::mem;
use core::ptr;
use core::clone::Clone;

pub trait RawStruct<T: Clone> {
    fn mem_base() -> *mut T;

    fn from_mem() -> T {
       let mut ptr = Self::mem_base();
       unsafe {
           (*ptr).clone()
       }
   }

   fn to_struct(&self) -> T;
   fn write(&self) {
       let mut ptr = Self::mem_base();
       unsafe {
           ptr::write(ptr, Self::to_struct(self))
       }
   }
}
