use zbus::{Result, fdo, interface};

use crate::msi::MsiEc;
use msi_ec_types::{FanMode, OnOff, ShiftMode};

pub struct MsiService;

#[interface(name = "by.toshibam.MsiEc")]
impl MsiService {
    #[zbus(property)]
    async fn fan_mode(&self) -> fdo::Result<FanMode> {
        MsiEc::fan_mode().map_err(to_fdo_err)
    }

    #[zbus(property)]
    async fn set_fan_mode(&self, mode: FanMode) -> Result<()> {
        MsiEc::set_fan_mode(mode).map_err(to_err)
    }

    #[zbus(property)]
    async fn available_fan_modes(&self) -> fdo::Result<Vec<FanMode>> {
        MsiEc::available_fan_modes().map_err(to_fdo_err)
    }

    #[zbus(property)]
    async fn shift_mode(&self) -> fdo::Result<ShiftMode> {
        MsiEc::shift_mode().map_err(to_fdo_err)
    }

    #[zbus(property)]
    async fn set_shift_mode(&self, mode: ShiftMode) -> Result<()> {
        MsiEc::set_shift_mode(mode).map_err(to_err)
    }

    #[zbus(property)]
    async fn available_shift_modes(&self) -> fdo::Result<Vec<ShiftMode>> {
        MsiEc::available_shift_modes().map_err(to_fdo_err)
    }

    #[zbus(property)]
    async fn cooler_boost(&self) -> fdo::Result<OnOff> {
        MsiEc::cooler_boost().map_err(to_fdo_err)
    }

    #[zbus(property)]
    async fn set_cooler_boost(&self, state: OnOff) -> Result<()> {
        MsiEc::set_cooler_boost(state).map_err(to_err)
    }

    #[zbus(property)]
    async fn super_battery(&self) -> fdo::Result<OnOff> {
        MsiEc::super_battery().map_err(to_fdo_err)
    }

    #[zbus(property)]
    async fn set_super_battery(&self, state: OnOff) -> Result<()> {
        MsiEc::set_super_battery(state).map_err(to_err)
    }
}

fn to_fdo_err<E: std::fmt::Display>(e: E) -> fdo::Error {
    fdo::Error::Failed(e.to_string())
}

fn to_err<E: std::fmt::Display>(e: E) -> zbus::Error {
    zbus::Error::FDO(Box::new(fdo::Error::Failed(e.to_string())))
}
