#![allow(non_snake_case)]

extern crate mosquitto_sys;

use std::ffi;

#[derive(Debug)]
pub enum Error {
    ConnPending,
    Success,
    NoMem,
    Protocol,
    Inval,
    NoConn,
    ConnRefused,
    NotFound,
    ConnLost,
    TLS,
    PayloadSize,
    NotSupported,
    Auth,
    ACLDenied,
    Unknown,
    ErrNo,
    EAI,
    Proxy
}

impl Error {
    fn from_i32(errno: i32) -> Error {
        let err;
        if errno >= -1 && errno <= 16 {
            err = unsafe { std::mem::transmute(errno) };
        } else {
            err = mosquitto_sys::mosq_err_t::MOSQ_ERR_UNKNOWN
        }

        Error::from_mosq_err_t(err)
    }

    fn from_mosq_err_t(err: mosquitto_sys::mosq_err_t) -> Error {
        use mosquitto_sys::mosq_err_t;

        match err {
            mosq_err_t::MOSQ_ERR_CONN_PENDING => Error::ConnPending,
            mosq_err_t::MOSQ_ERR_SUCCESS => Error::Success,
            mosq_err_t::MOSQ_ERR_NOMEM => Error::NoMem,
            mosq_err_t::MOSQ_ERR_PROTOCOL => Error::Protocol,
            mosq_err_t::MOSQ_ERR_INVAL => Error::Inval,
            mosq_err_t::MOSQ_ERR_NO_CONN => Error::NoConn,
            mosq_err_t::MOSQ_ERR_CONN_REFUSED => Error::ConnRefused,
            mosq_err_t::MOSQ_ERR_NOT_FOUND => Error::NotFound,
            mosq_err_t::MOSQ_ERR_CONN_LOST => Error::ConnLost,
            mosq_err_t::MOSQ_ERR_TLS => Error::TLS,
            mosq_err_t::MOSQ_ERR_PAYLOAD_SIZE => Error::PayloadSize,
            mosq_err_t::MOSQ_ERR_NOT_SUPPORTED => Error::NotSupported,
            mosq_err_t::MOSQ_ERR_AUTH => Error::Auth,
            mosq_err_t::MOSQ_ERR_ACL_DENIED => Error::ACLDenied,
            mosq_err_t::MOSQ_ERR_UNKNOWN => Error::Unknown,
            mosq_err_t::MOSQ_ERR_ERRNO => Error::ErrNo,
            mosq_err_t::MOSQ_ERR_EAI => Error::EAI,
            mosq_err_t::MOSQ_ERR_PROXY => Error::Proxy
        }
    }
}

#[derive(Debug)]
pub enum QoS {
    AtMostOnce,
    AtLeastOnce,
    ExactlyOnce
}

impl QoS {
    pub fn to_i32(&self) -> i32 {
        match *self {
            QoS::AtMostOnce => 0,
            QoS::AtLeastOnce => 1,
            QoS::ExactlyOnce => 2
        }
    }
}

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
        let ptr;

        unsafe {
            ptr = mosquitto_sys::mosquitto_new(std::ptr::null_mut(), clean_session, std::ptr::null_mut());
        }

        Mosquitto {
            ptr: ptr
        }
    }

    pub fn destroy(&self) {
        unsafe {
            mosquitto_sys::mosquitto_destroy(self.ptr);
        }
    }

    pub fn connect(&self, host: &str, port: i32, keepalive: i32) -> Error {
        let c_host = ffi::CString::new(host.as_bytes()).unwrap();

        let res;
        unsafe {
            res = mosquitto_sys::mosquitto_connect(self.ptr, c_host.as_ptr(), port, keepalive);
        }
        Error::from_i32(res)
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

    pub fn publish(&self, topic: &str, payload: &str, qos: QoS) -> Error {
        let c_topic = ffi::CString::new(topic.as_bytes()).unwrap();
        let c_payload = ffi::CString::new(payload.as_bytes()).unwrap();

        let payload_length = payload.len() as i32;
        let mid = std::ptr::null_mut();

        let res;
        unsafe {
            res = mosquitto_sys::mosquitto_publish(
                self.ptr,
                mid,
                c_topic.as_ptr(),
                payload_length,
                c_payload.as_ptr() as *mut std::os::raw::c_void,
                qos.to_i32(),
                false
            );
        }
        Error::from_i32(res)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
