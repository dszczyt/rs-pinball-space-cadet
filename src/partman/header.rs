use std::io::{self, Read};
use std::str::{self};

#[derive(Debug, Default)]
pub struct Header {
    pub signature: [u8; 21],
    pub app_name: String,
    pub description: String,
    pub size: u32,
    pub nb_groups: u16,
    pub body_size: u32,
    pub _unknown: u16,
}

// impl Default for File {
//     fn default() -> Self {
//         Self {
//             file_signature: Default::default(),
//             app_name: [0; 50],
//             description: [0, 100],
//         }
//     }
// }

impl Header {
    pub fn from_reader(reader: &mut impl Read) -> io::Result<Self> {
        let mut header = Header::default();

        reader.read_exact(&mut header.signature)?;

        let mut app_name = [0; 50];
        reader.read_exact(&mut app_name)?;
        header.app_name = str::from_utf8(&app_name)
            .unwrap()
            .trim_end_matches('\0')
            .to_string();

        let mut description = [0; 100];
        reader.read_exact(&mut description)?;
        header.description = str::from_utf8(&description)
            .unwrap()
            .trim_end_matches('\0')
            .to_string();

        let mut file_size = [0; 4];
        reader.read_exact(&mut file_size)?;
        header.size = u32::from_le_bytes(file_size);

        let mut nb_groups = [0; 2];
        reader.read_exact(&mut nb_groups)?;
        header.nb_groups = u16::from_le_bytes(nb_groups);

        let mut body_size = [0; 4];
        reader.read_exact(&mut body_size)?;
        header.body_size = u32::from_le_bytes(body_size);

        let mut _unknown = [0; 2];
        reader.read_exact(&mut _unknown)?;
        header._unknown = u16::from_le_bytes(_unknown);

        Ok(header)
    }
}
