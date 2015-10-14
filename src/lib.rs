extern crate libc;

use std::mem::transmute;
use prodbg::Backend;

#[repr(C)]

pub mod prodbg
{

pub struct CPDReaderAPI {
    private_data: *mut ::libc::c_void,
    read_u8: extern fn(data: *mut ::libc::c_void),
}



pub struct Reader {
    c_reader_api: *mut CPDReaderAPI,
}

impl Reader {
    pub fn read_u8(&self) {
        unsafe {
            ((*self.c_reader_api).read_u8)((*self.c_reader_api).private_data)
        }
    }
}

pub trait Backend {
    fn update(&mut self);
    //fn update(&mut self, reader: &Reader);
}

#[repr(C)]
pub struct CBackendCallbacks {
    pub create_instance: fn() -> *mut ::libc::c_void, 
    pub destroy_instance: fn(*mut ::libc::c_void), 
    pub update: fn(*mut ::libc::c_void), 
}

}

struct MyBackend {
    some_data: i32,
}

impl MyBackend {
    pub fn new() -> MyBackend {
        MyBackend { some_data: 0 }
    }
}

impl prodbg::Backend for MyBackend {

    //fn update(&mut self, reader: &prodbg::Reader)
    fn update(&mut self)
    {
        //reader.read_u8();
        self.some_data = 0;
    }
}

// these needs to be generated for each type


fn call_create_instance() -> *mut ::libc::c_void {
    let instance = unsafe { transmute(Box::new(MyBackend::new())) };
    instance
}

fn call_destroy_instance(ptr: *mut ::libc::c_void) {
    let instance: Box<MyBackend> = unsafe{ transmute(ptr) };
    // implicitly dropped
}

fn call_update_instance(ptr: *mut ::libc::c_void) { 
    let backend: &mut MyBackend = unsafe { &mut *(ptr as *mut MyBackend) };
    backend.update()
    // update backend here
}

#[no_mangle]
pub static mut g_backend: prodbg::CBackendCallbacks = prodbg::CBackendCallbacks { 
    create_instance: call_create_instance, 
    destroy_instance: call_destroy_instance, 
    update: call_update_instance 
};

#[test]
fn it_works() {
}
