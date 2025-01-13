#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    info: Info,
    sz: Sz,
    _reserved2: [u8; 0xf8],
    ctrl: Ctrl,
    _reserved3: [u8; 0x05fc],
    invalidate: Invalidate,
}
impl RegisterBlock {
    #[doc = "0x00 - Cache ID Register."]
    #[inline(always)]
    pub const fn info(&self) -> &Info {
        &self.info
    }
    #[doc = "0x04 - Memory Configuration Register."]
    #[inline(always)]
    pub const fn sz(&self) -> &Sz {
        &self.sz
    }
    #[doc = "0x100 - Cache Control and Status Register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x700 - Invalidate All Registers."]
    #[inline(always)]
    pub const fn invalidate(&self) -> &Invalidate {
        &self.invalidate
    }
}
#[doc = "INFO (r) register accessor: Cache ID Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@info`]
module"]
#[doc(alias = "INFO")]
pub type Info = crate::Reg<info::InfoSpec>;
#[doc = "Cache ID Register."]
pub mod info;
#[doc = "SZ (r) register accessor: Memory Configuration Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sz::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sz`]
module"]
#[doc(alias = "SZ")]
pub type Sz = crate::Reg<sz::SzSpec>;
#[doc = "Memory Configuration Register."]
pub mod sz;
#[doc = "CTRL (rw) register accessor: Cache Control and Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Cache Control and Status Register."]
pub mod ctrl;
#[doc = "INVALIDATE (rw) register accessor: Invalidate All Registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`invalidate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`invalidate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@invalidate`]
module"]
#[doc(alias = "INVALIDATE")]
pub type Invalidate = crate::Reg<invalidate::InvalidateSpec>;
#[doc = "Invalidate All Registers."]
pub mod invalidate;
