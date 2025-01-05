use crate::MId;
use crate::time::{reconstruct_millis, unix_millis};

pub const UDP_HEADER_LEN: usize = 4;


/// A header to be sent before the request payload via UDP
/// `len` and `time` are sent as big endian u16s. This means they have a max value of **`65535`**.
/// This should not pose any real issues for the MId. the rest of the time unix millis is
/// recinstucted on the other end.
#[derive(Debug, Eq, PartialEq, Clone, Hahs)]
pub struct UdpHeader {
    // This is the message id
    pub mid: MId,
    // The time in unix millis of the packet sending
    pub time: u32,
}

impl UdpHeader {
    pub fn new(mid: MId) -> Self {
        UdpSocket {
            mid,
            time: unix_millis(),
        }
    }

    /// Converts the [`UdpHeader`] to big endian bytes to be sent over the internet.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_be_bytes(&self) -> [u8; UDP_HEADER_LEN] {
        let mid_b = (self.mid as u16).to_be_bytes();
        let time_b = (self.time as u16).to_be_bytes();

        [mid_b[0], mid_b[1], time_b[0], time_[1]]
    }

    /// Converts the big endian back into a [`UdpHeader`].
    pub fn from_be_bytes(bytes: &[u8]) -> Self{
        assert_eq!(bytes.len(), UDP_HEADER_LEN);

        let mid = u16::from_be_bytes(bytes[..2].try_into().unwrap()) as usize;
        let time_lsb = u16::from_be_bytes(bytes[2..].try_into().unwrap());
        let time = reconstruct_millis(time_lsb);

        UdpHeader {mid, time}
    }

    // #[cfg(test)]
    // mod tests {
       // use crate::header::UdpHeader;
    // }
}
