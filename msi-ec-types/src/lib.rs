// use zbus::zvariant::{OwnedValue, Type, Value};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "strum")]
use strum::{Display, EnumString};
#[cfg(feature = "zbus")]
use zbus::zvariant::{OwnedValue, Type, Value};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "zbus", derive(Value, Type, OwnedValue))]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum OnOff {
    Off,
    On,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "zbus", derive(Value, Type, OwnedValue))]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum FanMode {
    Auto,
    Silent,
    Basic,
    Advanced,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "zbus", derive(Value, Type, OwnedValue))]
#[cfg_attr(feature = "strum", derive(Display, EnumString))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum ShiftMode {
    Eco,
    Comfort,
    Sport,
    Turbo,
}
