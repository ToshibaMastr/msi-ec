use serde::{Deserialize, Serialize};
use zbus::zvariant::{Type, Value};

#[derive(Deserialize, Serialize, Type, Value, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OnOff {
    Off,
    On,
}

#[derive(Deserialize, Serialize, Type, Value, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FanMode {
    Auto,
    Silent,
    Basic,
    Advanced,
}

#[derive(Deserialize, Serialize, Type, Value, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ShiftMode {
    Eco,
    Comfort,
    Sport,
    Turbo,
}
