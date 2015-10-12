extern crate libc;

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
    fn update(&mut self, reader: &Reader);
}

}

struct MyBackend {
    some_data: i32,
}

impl prodbg::Backend for MyBackend {

    fn update(&mut self, reader: &prodbg::Reader)
    {
        reader.read_u8();
        self.some_data = 0;
    }
}


#[test]
fn it_works() {
}
