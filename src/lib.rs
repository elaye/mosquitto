#![allow(non_snake_case)]

extern crate mosquitto_sys;

pub mod qos;
pub mod error;

use std::ffi;

use error::MosqError;
use qos::QoS;

type MosqResult = Result<String, String>;

pub fn init() {
    unsafe {
        mosquitto_sys::mosquitto_lib_init();
    }
}

pub fn cleanup() {
    unsafe {
        mosquitto_sys::mosquitto_lib_cleanup();
    }
}

pub struct Mosquitto {
    pub ptr: *mut mosquitto_sys::mosquitto
}

impl Mosquitto {
    pub fn new(clean_session: bool) -> Mosquitto {
        let ptr = unsafe {
            mosquitto_sys::mosquitto_new(std::ptr::null_mut(), clean_session, std::ptr::null_mut())
        };

        Mosquitto {
            ptr: ptr
        }
    }

    pub fn destroy(&self) {
        unsafe {
            mosquitto_sys::mosquitto_destroy(self.ptr);
        }
    }

    pub fn connect(&self, host: &str, port: i32, keepalive: i32) -> MosqResult {
        let c_host = ffi::CString::new(host.as_bytes()).unwrap();

        let res = unsafe {
            mosquitto_sys::mosquitto_connect(self.ptr, c_host.as_ptr(), port, keepalive)
        };
        MosqError::from_i32(res).to_res()
    }

    pub fn disconnect(&self) {
        unsafe {
            mosquitto_sys::mosquitto_disconnect(self.ptr);
        }
    }

    pub fn main_loop(&self) {
        let timeout = -1;
        let max_packets = 1;
        unsafe {
            mosquitto_sys::mosquitto_loop(self.ptr, timeout, max_packets);
        }
    }

    pub fn publish(&self, topic: &str, payload: &str, qos: QoS) -> MosqResult {
        let c_topic = ffi::CString::new(topic.as_bytes()).unwrap();
        let c_payload = ffi::CString::new(payload.as_bytes()).unwrap();

        let payload_length = payload.len() as i32;
        let mid = std::ptr::null_mut();

        let res = unsafe {
            mosquitto_sys::mosquitto_publish(
                self.ptr,
                mid,
                c_topic.as_ptr(),
                payload_length,
                c_payload.as_ptr() as *mut std::os::raw::c_void,
                qos.to_i32(),
                false
            )
        };
        MosqError::from_i32(res).to_res()
    }
}

impl Drop for Mosquitto {
    fn drop(&mut self) {
        self.disconnect();
        self.destroy();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
