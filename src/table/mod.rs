use num_derive::FromPrimitive;

pub mod bumper;
mod left_flipper;
pub use left_flipper::LeftFlipper;
pub mod right_flipper;
pub mod score;

#[derive(FromPrimitive)]
pub enum Component {
    Bumper = 1005,
}
