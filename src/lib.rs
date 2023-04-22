#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref CH_FIND_DEVICES_MUTEX: Mutex<()> = Mutex::new(());
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn find_devices() -> (i32, i32) {
    let _lock = CH_FIND_DEVICES_MUTEX.lock().unwrap();
    let devices = &mut 0;
    let _ret = unsafe { ch_find_devices(0, devices) };
    (_ret, *devices as i32)
}

pub fn find_devices_ext() -> i32 {
    let _lock = CH_FIND_DEVICES_MUTEX.lock().unwrap();
    unsafe { ch_find_devices_ext(0, &mut 0, 0, &mut 0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_devices() {
        let (ret, devices) = find_devices();
        assert_eq!(ret, 0);
        assert_eq!(devices, 0);
    }
}
