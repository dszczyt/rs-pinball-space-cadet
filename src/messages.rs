use num_derive::FromPrimitive;

#[derive(Debug, Clone, PartialEq, FromPrimitive)]
pub enum Message {
    LeftFlipperInputPressed = 1000,
    LeftFlipperInputReleased = 1001,
    RightFlipperInputPressed = 1002,
    RightFlipperInputReleased = 1003,

    Quit = -1,
}

pub trait MessageHandler {
    fn handle(&mut self, message: Message) -> Result<(), String>;
}
