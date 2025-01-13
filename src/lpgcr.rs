#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    rst: Rst,
    pclkdis: Pclkdis,
}
impl RegisterBlock {
    #[doc = "0x08 - Low Power Reset Register."]
    #[inline(always)]
    pub const fn rst(&self) -> &Rst {
        &self.rst
    }
    #[doc = "0x0c - Low Power Peripheral Clock Disable Register."]
    #[inline(always)]
    pub const fn pclkdis(&self) -> &Pclkdis {
        &self.pclkdis
    }
}
#[doc = "RST (rw) register accessor: Low Power Reset Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst`]
module"]
#[doc(alias = "RST")]
pub type Rst = crate::Reg<rst::RstSpec>;
#[doc = "Low Power Reset Register."]
pub mod rst;
#[doc = "PCLKDIS (rw) register accessor: Low Power Peripheral Clock Disable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pclkdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pclkdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pclkdis`]
module"]
#[doc(alias = "PCLKDIS")]
pub type Pclkdis = crate::Reg<pclkdis::PclkdisSpec>;
#[doc = "Low Power Peripheral Clock Disable Register."]
pub mod pclkdis;
