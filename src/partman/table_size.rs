use super::{entry::EntryType, group::Group};

pub struct TableSize {
    pub width: u32,
    pub height: u32,
}

impl From<Group> for TableSize {
    fn from(group: Group) -> Self {
        let entry = group.get_entry(EntryType::ShortArray).unwrap().clone();
        let table_size = entry.clone().short_array.unwrap();
        TableSize {
            width: *table_size.first().unwrap() as u32,
            height: *table_size.get(1).unwrap() as u32,
        }
    }
}
