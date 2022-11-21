use std::{
    ffi::CString,
    io::{self, Read},
};

use super::{group::Group, header::Header};

#[derive(Debug)]
pub struct Dat {
    pub header: Header,
    pub groups: Vec<Group>,
}

impl Dat {
    pub fn from_reader(mut rdr: &mut impl Read) -> io::Result<Self> {
        let mut dat = Self {
            header: Header::from_reader(&mut rdr)?,
            groups: vec![],
        };

        loop {
            match Group::from_reader(&mut rdr) {
                Ok(group) => {
                    dat.groups.push(group);
                }
                Err(_) => break,
            }
        }
        Ok(dat)
    }

    pub fn get_group_by_name(&self, name: String) -> Option<&Group> {
        self.groups.iter().find(|&group| {
            group
                .get_entry(super::entry::EntryType::GroupName)
                .map_or(false, |entry| {
                    CString::from_vec_with_nul(entry.data.clone().unwrap().0.into())
                        .unwrap()
                        .into_string()
                        .unwrap()
                        == name
                })
        })
    }
}
