extern crate mosquitto_sys;
use std;

#[derive(Debug)]
pub enum MosqError {
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

impl MosqError {
    pub fn from_i32(errno: i32) -> MosqError {
        let err;
        if errno >= -1 && errno <= 16 {
            err = unsafe { std::mem::transmute(errno) };
        } else {
            err = mosquitto_sys::mosq_err_t::MOSQ_ERR_UNKNOWN
        }

        MosqError::from_mosq_err_t(err)
    }

    fn from_mosq_err_t(err: mosquitto_sys::mosq_err_t) -> MosqError {
        use mosquitto_sys::mosq_err_t;

        match err {
            mosq_err_t::MOSQ_ERR_CONN_PENDING => MosqError::ConnPending,
            mosq_err_t::MOSQ_ERR_SUCCESS => MosqError::Success,
            mosq_err_t::MOSQ_ERR_NOMEM => MosqError::NoMem,
            mosq_err_t::MOSQ_ERR_PROTOCOL => MosqError::Protocol,
            mosq_err_t::MOSQ_ERR_INVAL => MosqError::Inval,
            mosq_err_t::MOSQ_ERR_NO_CONN => MosqError::NoConn,
            mosq_err_t::MOSQ_ERR_CONN_REFUSED => MosqError::ConnRefused,
            mosq_err_t::MOSQ_ERR_NOT_FOUND => MosqError::NotFound,
            mosq_err_t::MOSQ_ERR_CONN_LOST => MosqError::ConnLost,
            mosq_err_t::MOSQ_ERR_TLS => MosqError::TLS,
            mosq_err_t::MOSQ_ERR_PAYLOAD_SIZE => MosqError::PayloadSize,
            mosq_err_t::MOSQ_ERR_NOT_SUPPORTED => MosqError::NotSupported,
            mosq_err_t::MOSQ_ERR_AUTH => MosqError::Auth,
            mosq_err_t::MOSQ_ERR_ACL_DENIED => MosqError::ACLDenied,
            mosq_err_t::MOSQ_ERR_UNKNOWN => MosqError::Unknown,
            mosq_err_t::MOSQ_ERR_ERRNO => MosqError::ErrNo,
            mosq_err_t::MOSQ_ERR_EAI => MosqError::EAI,
            mosq_err_t::MOSQ_ERR_PROXY => MosqError::Proxy
        }
    }

    pub fn to_res(&self) -> Result<String, String> {
        match *self {
            MosqError::Success => Ok(String::from("Success")),
            MosqError::ConnPending => Err(String::from("Error: connection pending")),
            MosqError::NoMem => Err(String::from("Error: no memory")),
            MosqError::Protocol => Err(String::from("Error: protocol")),
            MosqError::Inval => Err(String::from("Error: invalid")),
            MosqError::NoConn => Err(String::from("Error: no connection")),
            MosqError::ConnRefused => Err(String::from("Error: connection refused")),
            MosqError::NotFound => Err(String::from("Error: not found")),
            MosqError::ConnLost => Err(String::from("Error: connection lost")),
            MosqError::TLS => Err(String::from("Error: TLS error")),
            MosqError::PayloadSize => Err(String::from("Error: payload size error")),
            MosqError::NotSupported => Err(String::from("Error: not supported")),
            MosqError::Auth => Err(String::from("Error: authentication")),
            MosqError::ACLDenied => Err(String::from("Error: ACL denied")),
            MosqError::Unknown => Err(String::from("Error: unknown")),
            MosqError::ErrNo => Err(String::from("Error: ErrNo")),
            MosqError::EAI => Err(String::from("Error: EAI")),
            MosqError::Proxy => Err(String::from("Error: proxy"))
        }
    }
}
