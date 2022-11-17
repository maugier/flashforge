use std::{net::{UdpSocket, IpAddr}, time::Duration, io::ErrorKind};

use anyhow::Result;

pub struct Scanner {
    sock: UdpSocket
}

pub struct ScanResult {
    pub address: IpAddr,
    pub machine_name: String,
}

impl Scanner {
    pub fn scan(millis: u64) -> Result<Self> {
        let sock = UdpSocket::bind(("0.0.0.0", 18001))?;
        sock.set_read_timeout(Some(Duration::from_millis(millis)))?;
        sock.send_to(b"\xc0\xa8\x01\x0c\x46\x51\x00\x00", "225.0.0.9:19000")?;
        Ok(Scanner { sock })
    }
}

impl Iterator for Scanner {
    type Item = Result<ScanResult>;

    fn next(&mut self) -> Option<Self::Item> {

        let mut reply = [0; 512];
        let (size, peer) = match self.sock.recv_from(&mut reply) {
            Ok(v) => v,
            Err(e) if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut => return None,
            Err(e) => return Some(Err(e.into())),
        };
    
        let reply: &[u8] = &reply[..size];
    
        let bin_name = reply[..128].splitn(2, |&b| b == 0x00).next().unwrap();

        let machine_name = match std::str::from_utf8(bin_name) {
            Ok(name) => name.to_owned(),
            Err(e) => return Some(Err(e.into())),
        };
        
        Some(Ok(ScanResult { address: peer.ip(), machine_name }))
    }
}
