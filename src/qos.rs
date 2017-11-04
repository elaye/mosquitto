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
