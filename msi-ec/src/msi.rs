use anyhow::Result;
use msi_ec_types::{FanMode, OnOff, ShiftMode};
use serde::{Serialize, de::DeserializeOwned};
use std::{fs, path::Path};

const PATH: &str = "/sys/devices/platform/msi-ec/";

pub struct MsiEc;
impl MsiEc {
    fn read_plain_vec<T: DeserializeOwned>(file: &str) -> Result<Vec<T>> {
        Ok(Self::read(file)?
            .lines()
            .map(serde_plain::from_str)
            .collect::<Result<Vec<_>, _>>()?)
    }

    fn read_plain<T: DeserializeOwned>(file: &str) -> Result<T> {
        Ok(serde_plain::from_str(&Self::read(file)?)?)
    }

    fn write_plain<T: Serialize>(file: &str, value: &T) -> Result<()> {
        Self::write(file, &serde_plain::to_string(value)?)
    }

    fn read(file: &str) -> Result<String> {
        let path = Path::new(PATH).join(file);
        Ok(fs::read_to_string(&path)?.trim().to_string())
    }

    fn write(file: &str, content: &str) -> Result<()> {
        let path = Path::new(PATH).join(file);
        Ok(fs::write(path, content)?)
    }
}

impl MsiEc {
    pub fn cooler_boost() -> Result<OnOff> {
        Self::read_plain("cooler_boost")
    }

    pub fn set_cooler_boost(state: OnOff) -> Result<()> {
        Self::write_plain("cooler_boost", &state)
    }
}

impl MsiEc {
    pub fn super_battery() -> Result<OnOff> {
        Self::read_plain("super_battery")
    }

    pub fn set_super_battery(state: OnOff) -> Result<()> {
        Self::write_plain("super_battery", &state)
    }
}

impl MsiEc {
    pub fn available_fan_modes() -> Result<Vec<FanMode>> {
        Self::read_plain_vec("available_fan_modes")
    }

    pub fn fan_mode() -> Result<FanMode> {
        Self::read_plain("fan_mode")
    }

    pub fn set_fan_mode(mode: FanMode) -> Result<()> {
        Self::write_plain("fan_mode", &mode)
    }
}

impl MsiEc {
    pub fn available_shift_modes() -> Result<Vec<ShiftMode>> {
        Self::read_plain_vec("available_shift_modes")
    }

    pub fn shift_mode() -> Result<ShiftMode> {
        Self::read_plain("shift_mode")
    }

    pub fn set_shift_mode(mode: ShiftMode) -> Result<()> {
        Self::write_plain("shift_mode", &mode)
    }
}
