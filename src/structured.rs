use std::io::BufRead;
use anyhow::{bail, Result};
use byteorder::{ReadBytesExt, BigEndian};

#[derive(Debug,PartialEq,Eq)]
pub enum Object {
    String(String),
    Vector(Vec<Object>),
    Blob(Vec<u8>),
}

impl Object {
    pub fn read<R: BufRead>(reader: &mut R) -> Result<Object> {

        let tag = reader.read_u32::<BigEndian>()?;

        match tag {
            0x44aaaa44 => { // vector
                let count = reader.read_u32::<BigEndian>()?;
                if count > 65536 { bail!("oversized vector")}
                let mut v = vec![];
                for _ in 0..count {
                    v.push(Object::read(reader)?);
                }
                Ok(Object::Vector(v))
            },
            0x3a3aa3a3 => {
                let size = reader.read_u32::<BigEndian>()?;
                let mut buf = vec![0; size as usize];
                reader.read_exact(&mut buf)?;
                Ok(Object::String(String::from_utf8(buf)?))
            }, // string
            0x2a2aa2a2 => todo!(),
            unknown => bail!("Unkown tag {}", unknown)
        }        
    }

    pub fn into_vec(self) -> Option<Vec<Object>> {
        if let Object::Vector(v) = self { Some(v) } else { None }
    }

    pub fn into_string(self) -> Option<String> {
        if let Object::String(s) = self { Some(s) } else { None }
    }

}


#[test]
fn test_deserialize() {
    let packet = [0x44, 0xaa, 0xaa, 0x44, 0x00, 0x00, 0x00, 0x05, 
                  0x3a, 0x3a, 0xa3, 0xa3, 0x00, 0x00, 0x00, 0x0c, 
                  0x2f, 0x64, 0x61, 0x74, 0x61, 0x2f, 0x4e, 0x4d, 
                  0x33, 0x2e, 0x67, 0x78, 0x3a, 0x3a, 0xa3, 0xa3, 
                  0x00, 0x00, 0x00, 0x17, 0x2f, 0x64, 0x61, 0x74, 
                  0x61, 0x2f, 0x6e, 0x6f, 0x7a, 0x7a, 0x6c, 0x65, 
                  0x5f, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x72, 
                  0x2e, 0x67, 0x78, 0x3a, 0x3a, 0xa3, 0xa3, 0x00, 
                  0x00, 0x00, 0x13, 0x2f, 0x64, 0x61, 0x74, 0x61, 
                  0x2f, 0x46, 0x69, 0x67, 0x68, 0x74, 0x65, 0x72, 
                  0x5f, 0x30, 0x31, 0x2e, 0x67, 0x78, 0x3a, 0x3a, 
                  0xa3, 0xa3, 0x00, 0x00, 0x00, 0x17, 0x2f, 0x64, 
                  0x61, 0x74, 0x61, 0x2f, 0x42, 0x75, 0x66, 0x66, 
                  0x5f, 0x42, 0x65, 0x65, 0x72, 0x5f, 0x6d, 0x75, 
                  0x67, 0x73, 0x2e, 0x67, 0x78, 0x3a, 0x3a, 0xa3, 
                  0xa3, 0x00, 0x00, 0x00, 0x15, 0x2f, 0x64, 0x61, 
                  0x74, 0x61, 0x2f, 0x32, 0x30, 0x6d, 0x6d, 0x5f, 
                  0x42, 0x6f, 0x78, 0x2d, 0x50, 0x4c, 0x41, 0x2e, 
                  0x67, 0x78];

    let mut reader: &[u8] = &packet;

    let actual = Object::read(&mut reader).unwrap();
    let expected = Object::Vector(vec![
        Object::String("/data/NM3.gx".into()),
        Object::String("/data/nozzle_remover.gx".into()),
        Object::String("/data/Fighter_01.gx".into()),
        Object::String("/data/Buff_Beer_mugs.gx".into()),
        Object::String("/data/20mm_Box-PLA.gx".into()),
    ]);

    assert_eq!(actual, expected);
    assert_eq!(reader, b"");

}
