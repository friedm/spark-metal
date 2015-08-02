use core::ptr;
use core::clone::Clone;

pub trait RawStruct<T: Clone> {
    fn from_mem(base: usize) -> T {
       let ptr = base as *const T;
       let result = unsafe {
           ptr::read::<T>(ptr)
       };
       unsafe { super::util::do_nothing(); }
       result
   }

   fn to_struct(&self) -> T;
   fn write(&self, write_to: usize) {
       let ptr = write_to as *mut T;
       unsafe {
           ptr::write(ptr, Self::to_struct(self));
       }
       unsafe { super::util::do_nothing(); }
   }
}
