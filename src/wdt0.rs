#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    rst: Rst,
    clksel: Clksel,
    cnt: Cnt,
}
impl RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control Register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Windowed Watchdog Timer Reset Register."]
    #[inline(always)]
    pub const fn rst(&self) -> &Rst {
        &self.rst
    }
    #[doc = "0x08 - Windowed Watchdog Timer Clock Select Register."]
    #[inline(always)]
    pub const fn clksel(&self) -> &Clksel {
        &self.clksel
    }
    #[doc = "0x0c - Windowed Watchdog Timer Count Register."]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
}
#[doc = "CTRL (rw) register accessor: Watchdog Timer Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Watchdog Timer Control Register."]
pub mod ctrl;
#[doc = "RST (w) register accessor: Windowed Watchdog Timer Reset Register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst`]
module"]
#[doc(alias = "RST")]
pub type Rst = crate::Reg<rst::RstSpec>;
#[doc = "Windowed Watchdog Timer Reset Register."]
pub mod rst;
#[doc = "CLKSEL (rw) register accessor: Windowed Watchdog Timer Clock Select Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksel`]
module"]
#[doc(alias = "CLKSEL")]
pub type Clksel = crate::Reg<clksel::ClkselSpec>;
#[doc = "Windowed Watchdog Timer Clock Select Register."]
pub mod clksel;
#[doc = "CNT (r) register accessor: Windowed Watchdog Timer Count Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Windowed Watchdog Timer Count Register."]
pub mod cnt;
