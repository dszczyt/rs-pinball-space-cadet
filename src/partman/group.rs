use std::io::{self, Read};


use thiserror::Error;

use super::entry::{Entry, EntryType};

#[derive(Debug)]
pub struct Group {
    pub nb_entries: usize,
    /*pub group_id: i64,
    pub group_name: String,*/
    entries: Vec<Entry>,
}

impl Group {
    pub fn from_reader(mut rdr: &mut impl Read) -> io::Result<Self> {
        let mut group = Group {
            nb_entries: 0,
            entries: vec![],
        };
        let mut nb_entries = [0; 1];
        rdr.read_exact(&mut nb_entries)?;
        group.nb_entries = u8::from_le_bytes(nb_entries).into();
        for _ in 0..group.nb_entries {
            match Entry::from_reader(&mut rdr) {
                Ok(entry) => {
                    group.entries.push(entry);
                }
                Err(_) => break,
            }
        }
        Ok(group)
    }

    pub fn get_entry(&self, entry_type: EntryType) -> Option<&Entry> {
        self.entries
            .iter()
            .find(|&entry| entry.entry_type == entry_type)
    }
}

#[derive(Error, Debug)]
pub enum DatFileError {
    #[error("partman: Use specific get for bitmaps")]
    BitmapsNotSupported,

    #[error("entry not found: {0}")]
    EntryNotFound(usize),
}

/*
struct DatFile {
    pub app_name: String,
    pub description: String,
    pub groups: Vec<Group>,
}

impl DatFile {
    fn entry_nth(
        &self,
        group_index: usize,
        target_entry_type: FieldTypes,
        skip_first_n: usize,
    ) -> Result<Option<&Entry>> {
        let group = match self.groups.get(group_index) {
            Some(group) => group,
            None => return Err(DatFileError::EntryNotFound(group_index).into()),
        };

        let mut skip_count: usize = 0;

        Ok(group
            .entries
            .iter()
            .take_while(|entry| entry.entry_type <= target_entry_type)
            .find(|entry| {
                entry.entry_type == target_entry_type && {
                    skip_count += 1;
                    skip_count
                } == skip_first_n
            }))
    }

    pub fn field_nth(
        &self,
        group_index: usize,
        target_entry_type: FieldTypes,
        skip_first_n: usize,
    ) -> Result<Option<String>> {
        match target_entry_type {
            FieldTypes::Bitmap8bit | FieldTypes::Bitmap16bit => {
                return Err(DatFileError::BitmapsNotSupported.into())
            }
            _ => {}
        }

        self.entry_nth(group_index, target_entry_type, skip_first_n)
            .map(|a| a.map(|entry| entry.buffer.clone()))
    }

    pub fn field_size_nth(
        &self,
        group_index: usize,
        target_entry_type: FieldTypes,
        skip_first_n: usize,
    ) -> Result<Option<usize>> {
        self.entry_nth(group_index, target_entry_type, skip_first_n)
            .map(|a| a.map(|entry| entry.field_size))
    }

    pub fn field_size(
        &self,
        group_index: usize,
        target_entry_type: FieldTypes,
    ) -> Result<Option<usize>> {
        self.field_size_nth(group_index, target_entry_type, 0)
    }
}
*/
