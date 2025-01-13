#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    status: Status,
    data: Data,
}
impl RegisterBlock {
    #[doc = "0x00 - TRNG Control Register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000."]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
}
#[doc = "CTRL (rw) register accessor: TRNG Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "TRNG Control Register."]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000."]
pub mod status;
#[doc = "DATA (r) register accessor: Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000.\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000."]
pub mod data;
