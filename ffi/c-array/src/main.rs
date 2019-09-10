use std::os::raw::c_int;

extern "C" {
    fn make_array(array: *mut *mut c_int, length: *mut c_int) -> c_int;
}

fn main() {
    let mut array: *mut c_int = std::ptr::null_mut();
    let mut length: c_int = 0;
    unsafe {
        let result = make_array(&mut array as *mut *mut c_int, &mut length as *mut c_int);
        if result != 0 {
            println!("failed",);
            return;
        }
        let array = Vec::from_raw_parts(array, length as usize, length as usize);
        println!("array: {:?}", array);
    };
}
