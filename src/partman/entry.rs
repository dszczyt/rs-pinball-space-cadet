use std::io::{self, Read};

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy, Default)]
pub enum EntryType {
    // One 16 bit signed integer
    ShortValue = 1,
    // Sprite bitmap, 8bpp, indexed color
    Bitmap8bit = 2,
    // Group name, char[]. Not all groups have names.
    GroupName = 3,
    // Palette, contains 256 RBGA 4-byte colors.
    Palette = 5,
    // String
    String = 9,
    // Array of 16 bit signed integers
    ShortArray = 10,
    // Array of 32 bit floats
    FloatArray = 11,
    // Sprite depth map, 16bpp, unsigned
    Bitmap16bit = 12,

    #[default]
    Unknown,
}

impl From<u8> for EntryType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::ShortValue,
            1 => Self::Bitmap8bit,
            3 => Self::GroupName,
            5 => Self::Palette,
            9 => Self::String,
            10 => Self::ShortArray,
            11 => Self::FloatArray,
            12 => Self::Bitmap16bit,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Entry {
    pub entry_type: EntryType, // FieldTypes,
    pub size: usize,           // usize,

    pub data: Option<bytes::Bytes>,
    pub value: Option<u16>,
    pub short_array: Option<Vec<i16>>,
}

impl Entry {
    pub fn from_reader(rdr: &mut impl Read) -> io::Result<Self> {
        let mut _entry_type = [0; 1];
        rdr.read_exact(&mut _entry_type)?;
        let entry_type: EntryType = u8::from_le_bytes(_entry_type).into();

        let mut entry = Entry::default();
        entry.entry_type = entry_type;

        match entry_type {
            EntryType::ShortValue => {
                let mut _value = [0; 2];
                rdr.read_exact(&mut _value)?;
                entry.value = Some(u16::from_le_bytes(_value));
            }
            EntryType::ShortArray => {
                let mut _size = [0; 4];
                rdr.read_exact(&mut _size)?;
                entry.size = u32::from_le_bytes(_size) as usize;

                let mut short_array = vec![];

                for _ in 0..entry.size / 2 {
                    let mut _data = [0; 2];
                    rdr.read_exact(&mut _data)?;
                    short_array.push(i16::from_le_bytes(_data));
                }
                // dbg!(&short_array);
                entry.short_array = Some(short_array);
            }
            _entry_type => {
                let mut _size = [0; 4];
                rdr.read_exact(&mut _size)?;
                entry.size = u32::from_le_bytes(_size) as usize;

                let mut _data = Vec::with_capacity(entry.size);
                rdr.take(entry.size as u64).read_to_end(&mut _data)?;

                // if entry_type == EntryType::GroupName || entry_type == EntryType::String {
                //     dbg!((entry_type, str::from_utf8(&_data).unwrap().to_string()));
                // }
                entry.data = Some(_data.into());
            }
        }

        Ok(entry)
    }
}
