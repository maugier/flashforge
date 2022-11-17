use std::{net::{TcpStream, ToSocketAddrs}, io::{Write, BufReader, BufRead}, str::FromStr};
use anyhow::{Error, Result, bail, anyhow};
use itertools::Itertools;

mod scanner;

pub use scanner::Scanner;

pub struct FlashForge {
    sock: TcpStream,
    buf: BufReader<TcpStream>,
}

#[derive(Debug)]
pub struct V3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug)]
pub struct Status {
    pub endstop: V3<bool>,
    pub status: String,
    pub movemode: String,
    pub led: bool,
    pub file: String,
}

#[derive(Debug)]
pub struct Temperature {
    pub current: u16,
    pub target: u16,
}

#[derive(Debug)]
pub struct Temperatures {
    pub nozzle: Option<Temperature>,
    pub    bed: Option<Temperature>,
}


impl FlashForge {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let sock = TcpStream::connect(addr)?;
        let buf = BufReader::new(sock.try_clone()?);
        let forge = Self { sock, buf };

        Ok(forge)
    }

    pub fn command(&mut self, cmd: &str, args: &str) -> Result<String> {
        self.sock.write_all(b"~")?;
        self.sock.write_all(cmd.as_bytes())?;
        if args != "" {
            self.sock.write_all(b" ")?;
            self.sock.write_all(args.as_bytes())?;
        }
        self.sock.write_all(b"\r\n")?;

        let expected = format!("CMD {} Received.\r\n", cmd);

        let mut status = vec![];
        self.buf.read_until(b'\n', &mut status)?;

        if &status != expected.as_bytes() {
            bail!("Unexpected command answer (received: {:?}, expected: {:?})", std::str::from_utf8(&status)?, expected);
        }

        let mut response = vec![];

        loop {
    
            self.buf.read_until(b'\n', &mut response)?;
            let suffix = (response.len() - 4)..;

            if &response[suffix.clone()] == b"ok\r\n" {
                response.drain(suffix);
                return Ok(String::from_utf8(response)?);
            }


        }

    }

    pub fn info(&mut self) -> Result<String> {
        self.command("M115", "")
    }

    pub fn led(&mut self, rgb: (u8,u8,u8)) -> Result<()> {
        todo!()
    }

    pub fn login(&mut self) -> Result<()> {
        let reply = self.command("M601", "S1")?;
        if reply.starts_with("Control Success") { 
            Ok(())
        } else {
            bail!("Unexpected reply to M601: {}", reply)
        }
    }

    pub fn logout(&mut self) -> Result<()> {
        let reply = self.command("M602", "")?;
        if reply.starts_with("Control Release") {
            Ok(())
        } else {
            bail!("Unexpected reply to M602: {}", reply)
        }
    }

    pub fn rename(&mut self, name: &str) -> Result<()> {
        if name.len() > 32 { bail!("New name too long (32 bytes only)") }
        if !name.is_ascii() { bail!("New name is not ascii, better safe than sorry.") }

        self.command("M610", &name)?;
        Ok(())
    }

    pub fn status(&mut self) -> Result<Status> {
        self.command("M119", "")?.parse()
    }

    pub fn temperature(&mut self) -> Result<Temperatures> {

        let (mut nozzle, mut bed) = (None, None);

        for item in self.command("M105", "")?.trim().split_whitespace() {
            let (key, rest) = item.split_once(':').ok_or(anyhow!("Unknown M105 reply format"))?;
            let (current, target) = rest.split_once('/').ok_or(anyhow!("Unknown M105 reply format"))?;

            let temperature = Temperature { current: current.parse()?, target: target.parse()? };

            match key {
                "T0" => { nozzle = Some(temperature)},
                "B"  => { bed = Some(temperature)},
                _ => (),
            }
        }

        Ok(Temperatures { nozzle, bed })

    }


}

impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();

        let (x,y,z) = lines[0]
                .strip_prefix("Endstop: ")
                .ok_or_else(|| anyhow!("no endstop line: {}", &lines[0]))?
                .split_whitespace()
                .tuples().next()
                .ok_or_else(|| anyhow!("Not enough blocks in endstop line: {}", &lines[0]))?;

        let x: u8 = x.strip_prefix("X-max:").ok_or_else(|| anyhow!("no x-max"))?.parse()?;
        let y: u8 = y.strip_prefix("Y-max:").ok_or_else(|| anyhow!("no y-max"))?.parse()?;
        let z: u8 = z.strip_prefix("Z-max:").ok_or_else(|| anyhow!("no z-max"))?.parse()?;

        todo!()

    }
}