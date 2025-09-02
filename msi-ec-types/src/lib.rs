use serde::{Deserialize, Serialize};
use zbus::zvariant::{OwnedValue, Type, Value};

#[derive(Deserialize, Serialize, Type, Value, OwnedValue, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OnOff {
    Off,
    On,
}

#[derive(Deserialize, Serialize, Type, Value, OwnedValue, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FanMode {
    Auto,
    Silent,
    Basic,
    Advanced,
}

#[derive(Deserialize, Serialize, Type, Value, OwnedValue, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ShiftMode {
    Eco,
    Comfort,
    Sport,
    Turbo,
}
