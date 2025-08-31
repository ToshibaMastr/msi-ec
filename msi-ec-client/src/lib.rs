use msi_ec_types::{FanMode, OnOff, ShiftMode};
use zbus::proxy;

#[proxy(
    interface = "by.toshibam.MsiEc",
    default_service = "by.toshibam.MsiEc",
    default_path = "/by/toshibam/MsiEc"
)]
pub trait MsiEc {
    fn get_fan_mode(&self) -> zbus::Result<FanMode>;
    fn set_fan_mode(&self, mode: FanMode) -> zbus::Result<()>;
    fn get_available_fan_modes(&self) -> zbus::Result<Vec<FanMode>>;

    fn get_shift_mode(&self) -> zbus::Result<ShiftMode>;
    fn set_shift_mode(&self, mode: ShiftMode) -> zbus::Result<()>;
    fn get_available_shift_modes(&self) -> zbus::Result<Vec<ShiftMode>>;

    fn get_cooler_boost(&self) -> zbus::Result<OnOff>;
    fn set_cooler_boost(&self, state: OnOff) -> zbus::Result<()>;

    fn get_super_battery(&self) -> zbus::Result<OnOff>;
    fn set_super_battery(&self, state: OnOff) -> zbus::Result<()>;
}
