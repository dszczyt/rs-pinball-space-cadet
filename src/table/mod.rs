use num_derive::FromPrimitive;

pub mod bumper;
pub mod score;

#[derive(FromPrimitive)]
pub enum Component {
    Bumper = 1005,
}
