#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    rtc: Rtc,
    _reserved1: [u8; 0x28],
    simo: Simo,
    _reserved2: [u8; 0x04],
    ipolo: Ipolo,
    ctrl: Ctrl,
    inro: Inro,
}
impl RegisterBlock {
    #[doc = "0x08 - RTC Trim System Initialization Register."]
    #[inline(always)]
    pub const fn rtc(&self) -> &Rtc {
        &self.rtc
    }
    #[doc = "0x34 - SIMO Trim System Initialization Register."]
    #[inline(always)]
    pub const fn simo(&self) -> &Simo {
        &self.simo
    }
    #[doc = "0x3c - IPO Low Trim System Initialization Register."]
    #[inline(always)]
    pub const fn ipolo(&self) -> &Ipolo {
        &self.ipolo
    }
    #[doc = "0x40 - Control Trim System Initialization Register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x44 - RTC Trim System Initialization Register."]
    #[inline(always)]
    pub const fn inro(&self) -> &Inro {
        &self.inro
    }
}
#[doc = "RTC (rw) register accessor: RTC Trim System Initialization Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc`]
module"]
#[doc(alias = "RTC")]
pub type Rtc = crate::Reg<rtc::RtcSpec>;
#[doc = "RTC Trim System Initialization Register."]
pub mod rtc;
#[doc = "SIMO (r) register accessor: SIMO Trim System Initialization Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`simo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@simo`]
module"]
#[doc(alias = "SIMO")]
pub type Simo = crate::Reg<simo::SimoSpec>;
#[doc = "SIMO Trim System Initialization Register."]
pub mod simo;
#[doc = "IPOLO (r) register accessor: IPO Low Trim System Initialization Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipolo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipolo`]
module"]
#[doc(alias = "IPOLO")]
pub type Ipolo = crate::Reg<ipolo::IpoloSpec>;
#[doc = "IPO Low Trim System Initialization Register."]
pub mod ipolo;
#[doc = "CTRL (rw) register accessor: Control Trim System Initialization Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Trim System Initialization Register."]
pub mod ctrl;
#[doc = "INRO (rw) register accessor: RTC Trim System Initialization Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`inro::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inro::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inro`]
module"]
#[doc(alias = "INRO")]
pub type Inro = crate::Reg<inro::InroSpec>;
#[doc = "RTC Trim System Initialization Register."]
pub mod inro;
