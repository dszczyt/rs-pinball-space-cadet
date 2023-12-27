use num_derive::FromPrimitive;

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum ObjectType {
    Plunger = 1001,
    Light = 1002,
    LeftFlipper = 1003,
    RightFlipper = 1004,
    Bumper = 1005,
    YellowTarget = 1006,
    Drain = 1007,
    Bloc = 1011,
    Kout = 1012,
    Gate = 1013,
    Kicker = 1014,
    Roll = 1015,
    OneWay = 1016,
    Sink = 1017,
    Flag = 1018,
    RedTarget = 1019,
    GreenRoll = 1020,
    Ramp = 1021,
    RampHole = 1022,
    Demo = 1023,
    Trip = 1024,
    Lights = 1026,
    BumpersList = 1028,
    //kout (no bmp) (?, similar to 1012?) = 1029,
    FuelBargraph = 1030,
    Sound = 1031,
    TextBox = 1033,
}

pub struct TableObject {
    pub object_type: i16,
    pub group_number: i16,
}
