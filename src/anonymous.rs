use std::ptr::write as ptr_write;
use std::alloc::{alloc, Layout};

pub trait Anonymous{ fn id() -> u16; }

#[derive(Debug)]
pub(crate) struct AnonymousCell{
    // using u16 allows for 65,536 objects while taking half 
    // the space of a u32
    pub(crate) id: u16,
    data: *mut u8
}

impl AnonymousCell{
    pub fn new<T: Anonymous>(data: T) -> Self{
        let data = unsafe{
            let layout = Layout::new::<T>();
            let ptr = alloc(layout) as *mut T;
            ptr_write(ptr, data);
            ptr as *mut u8
        };
        Self { id: T::id(), data }
    }

    pub unsafe fn get_ref<T: Anonymous>(&self) -> &T{
        let data_ptr = self.data as *const T;
        data_ptr.as_ref().expect("missing ptr")
    }

    pub unsafe fn get_mut<T: Anonymous>(&mut self) -> &mut T{
        let data_ptr = self.data as *mut T;
        data_ptr.as_mut().expect("Missing ptr")
    }

    // Insert new value while returning old value
    pub unsafe fn exchange_value<T: Anonymous + Copy>(&mut self, new: T) -> T{
        let held = *(self.data as *mut T);
        ptr_write(self.data as *mut T, new);
        held
    }

    // Insert new value without returning old value
    pub unsafe fn insert_value<T: Anonymous>(&mut self, new: T){
        ptr_write(self.data as *mut T, new);
    }

    pub unsafe fn as_raw<T: Anonymous>(&self) -> *const T{
        self.data as *const T
    }

    pub unsafe fn as_raw_mut<T: Anonymous>(&mut self) -> *mut T{
        self.data as *mut T
    }
}

impl Anonymous for i8{ fn id() -> u16 { 0 } }
impl Anonymous for i16{ fn id() -> u16 { 1 } }
impl Anonymous for i32{ fn id() -> u16 { 2 } }
impl Anonymous for i64{ fn id() -> u16 { 3 } }
impl Anonymous for i128{ fn id() -> u16 { 4 } }
impl Anonymous for isize { fn id() -> u16 { 5 } }

impl Anonymous for u8{ fn id() -> u16 { 6 } }
impl Anonymous for u16{ fn id() -> u16 { 7 } }
impl Anonymous for u32{ fn id() -> u16 { 8 } }
impl Anonymous for u64{ fn id() -> u16 { 9 } }
impl Anonymous for u128{ fn id() -> u16 { 10 } }
impl Anonymous for usize{ fn id() -> u16 { 11 } }

impl Anonymous for f32{ fn id() -> u16 { 12 } }
impl Anonymous for f64{ fn id() -> u16 { 13 } }
impl Anonymous for char{ fn id() -> u16 { 14 } }
impl Anonymous for bool{ fn id() -> u16 { 15 } }

impl<T> Anonymous for Option<T> where T: Anonymous{ fn id() -> u16 { T::id() + 100 } }