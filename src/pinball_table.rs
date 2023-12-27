use crate::partman::dat::Dat;

pub struct PinballTable {}

impl PinballTable {
    pub fn new(dat_contents: &Dat) -> Self {
        let table_objects_group = dat_contents
            .get_group_by_name("table_objects".to_string())
            .unwrap()
            .clone();

        Self {}
    }
}
