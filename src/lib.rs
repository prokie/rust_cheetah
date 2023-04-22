#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn find_devices() -> i32 {
    let my_speed_ptr: *mut u16 = &mut 0;
    unsafe { ch_find_devices(0, my_speed_ptr) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_devices() {
        assert!(find_devices() == 0);
    }
}
