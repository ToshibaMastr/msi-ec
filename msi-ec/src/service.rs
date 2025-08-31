use zbus::interface;

use crate::msi::MsiEc;
use msi_ec_types::{FanMode, OnOff, ShiftMode};

pub struct MsiService;

#[interface(name = "by.toshibam.MsiEc")]
impl MsiService {
    async fn get_fan_mode(&self) -> zbus::fdo::Result<FanMode> {
        MsiEc::fan_mode().map_err(to_fdo_err)
    }

    async fn set_fan_mode(&self, mode: FanMode) -> zbus::fdo::Result<()> {
        MsiEc::set_fan_mode(mode).map_err(to_fdo_err)
    }

    async fn get_available_fan_modes(&self) -> zbus::fdo::Result<Vec<FanMode>> {
        MsiEc::available_fan_modes().map_err(to_fdo_err)
    }

    async fn get_shift_mode(&self) -> zbus::fdo::Result<ShiftMode> {
        MsiEc::shift_mode().map_err(to_fdo_err)
    }

    async fn set_shift_mode(&self, mode: ShiftMode) -> zbus::fdo::Result<()> {
        MsiEc::set_shift_mode(mode).map_err(to_fdo_err)
    }

    async fn get_available_shift_modes(&self) -> zbus::fdo::Result<Vec<ShiftMode>> {
        MsiEc::available_shift_modes().map_err(to_fdo_err)
    }

    async fn get_cooler_boost(&self) -> zbus::fdo::Result<OnOff> {
        MsiEc::cooler_boost().map_err(to_fdo_err)
    }

    async fn set_cooler_boost(&self, state: OnOff) -> zbus::fdo::Result<()> {
        MsiEc::set_cooler_boost(state).map_err(to_fdo_err)
    }

    async fn get_super_battery(&self) -> zbus::fdo::Result<OnOff> {
        MsiEc::super_battery().map_err(to_fdo_err)
    }

    async fn set_super_battery(&self, state: OnOff) -> zbus::fdo::Result<()> {
        MsiEc::set_super_battery(state).map_err(to_fdo_err)
    }
}

fn to_fdo_err<E: std::fmt::Display>(e: E) -> zbus::fdo::Error {
    zbus::fdo::Error::Failed(e.to_string())
}
