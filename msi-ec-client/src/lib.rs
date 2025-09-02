use msi_ec_types::{FanMode, OnOff, ShiftMode};
use zbus::{Result, proxy};

#[proxy(
    interface = "by.toshibam.MsiEc",
    default_service = "by.toshibam.MsiEc",
    default_path = "/by/toshibam/MsiEc"
)]
pub trait MsiEc {
    //. fan_mode

    #[zbus(property)]
    fn fan_mode(&self) -> Result<FanMode>;

    #[zbus(property)]
    fn set_fan_mode(&self, mode: FanMode) -> Result<()>;

    #[zbus(property)]
    fn available_fan_modes(&self) -> Result<Vec<FanMode>>;

    //. shift_mode

    #[zbus(property)]
    fn shift_mode(&self) -> Result<ShiftMode>;

    #[zbus(property)]
    fn set_shift_mode(&self, mode: ShiftMode) -> Result<()>;

    #[zbus(property)]
    fn available_shift_modes(&self) -> Result<Vec<ShiftMode>>;

    //. cooler_boost

    #[zbus(property)]
    fn cooler_boost(&self) -> Result<OnOff>;

    #[zbus(property)]
    fn set_cooler_boost(&self, state: OnOff) -> Result<()>;

    //. super_battery

    #[zbus(property)]
    fn super_battery(&self) -> Result<OnOff>;

    #[zbus(property)]
    fn set_super_battery(&self, state: OnOff) -> Result<()>;
}
