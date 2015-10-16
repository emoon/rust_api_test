extern crate libc;

use prodbg::*;

#[repr(C)]

pub mod prodbg
{

use std::mem::transmute;
use libc::*;

pub struct CPDReaderAPI {
    private_data: *mut c_void,
    read_u8: extern fn(data: *mut c_void),
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
    fn new() -> Self;
    fn update(&mut self);
    //fn update(&mut self, reader: &Reader);
}

#[repr(C)]
pub struct CBackendCallbacks {
    pub create_instance: fn() -> *mut c_void, 
    pub destroy_instance: fn(*mut c_void), 
    pub update: fn(*mut c_void), 
}

pub fn create_instance<T: Backend>() -> *mut c_void {
    let instance = unsafe { transmute(Box::new(T::new())) };
    println!("Lets create instance!");
    instance
}

pub fn destroy_instance<T: Backend>(ptr: *mut c_void) {
    let instance: Box<T> = unsafe{ transmute(ptr) };
    // implicitly dropped
}

pub fn update_instance<T: Backend>(ptr: *mut c_void) { 
    let backend: &mut T = unsafe { &mut *(ptr as *mut T) };
    backend.update()
}

pub struct PluginHandler {
    pub private_data: *mut c_void,
    pub c_register_plugin: extern fn(plugin: *mut c_void, priv_data: *mut c_void),
}

impl PluginHandler {
    fn register_plugin(&self, plugin: &mut CBackendCallbacks) {
        unsafe {
            (self.c_register_plugin)(transmute(plugin), (self.private_data));
        }
    }
}

}

extern fn init_plugin(plugin_handler: &mut prodbg::PluginHandler) {}

#[no_mangle]
pub extern fn InitPlugin(cb: extern fn(plugin: *mut c_void, data: *mut c_void), priv_data: *mut c_void) {
    let mut plugin_handler = prodbg::PluginHandler { 
        private_data : priv_data, 
        c_register_plugin : cb
    };

    init_plugin(&mut plugin_handler);

    //cb(priv_data);
}



macro_rules! define_plugin {
    ($x:ty) => {
        {
            let mut plugin = prodbg::CBackendCallbacks { 
                create_instance: prodbg::create_instance::<$x>, 
                destroy_instance: prodbg::destroy_instance::<$x>, 
                update: prodbg::update_instance::<$x> 
             };

            plugin
        }
    }
}


struct MyBackend {
    some_data: i32,
}

impl prodbg::Backend for MyBackend {

    fn new() -> Self {
        MyBackend { some_data: 0 }
    }

    //fn update(&mut self, reader: &prodbg::Reader)
    fn update(&mut self) {
        println!("update instance! {}", self.some_data);
        self.some_data += 1;
    }
}

pub fn init_plugin(plugin_handler: &mut prodbg::PluginHandler) 
{
    let plugin = define_plugin!(MyBackend);
    plugin_handler.register_plugin(plugin);
}



//fn init_plugin() {
//   let plugin = define_plugin!(MyBackend);
//}

// these needs to be generated for each type



/*
#[no_mangle]
pub static mut g_backend: prodbg::CBackendCallbacks = prodbg::CBackendCallbacks { 
    create_instance: prodbg::create_instance::<MyBackend>, 
    destroy_instance: prodbg::destroy_instance::<MyBackend>, 
    update: prodbg::update_instance::<MyBackend> 
};
*/

#[test]
fn it_works() {
}

