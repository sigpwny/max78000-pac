#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lpcn: Lpcn,
    lpwkst0: Lpwkst0,
    lpwken0: Lpwken0,
    lpwkst1: Lpwkst1,
    lpwken1: Lpwken1,
    lpwkst2: Lpwkst2,
    lpwken2: Lpwken2,
    lpwkst3: Lpwkst3,
    lpwken3: Lpwken3,
    _reserved9: [u8; 0x0c],
    lppwst: Lppwst,
    lppwen: Lppwen,
    _reserved11: [u8; 0x10],
    gp0: Gp0,
    gp1: Gp1,
}
impl RegisterBlock {
    #[doc = "0x00 - Low Power Control Register."]
    #[inline(always)]
    pub const fn lpcn(&self) -> &Lpcn {
        &self.lpcn
    }
    #[doc = "0x04 - Low Power I/O Wakeup Status Register 0. This register indicates the low power wakeup status for GPIO0."]
    #[inline(always)]
    pub const fn lpwkst0(&self) -> &Lpwkst0 {
        &self.lpwkst0
    }
    #[doc = "0x08 - Low Power I/O Wakeup Enable Register 0. This register enables low power wakeup functionality for GPIO0."]
    #[inline(always)]
    pub const fn lpwken0(&self) -> &Lpwken0 {
        &self.lpwken0
    }
    #[doc = "0x0c - Low Power I/O Wakeup Status Register 1. This register indicates the low power wakeup status for GPIO1."]
    #[inline(always)]
    pub const fn lpwkst1(&self) -> &Lpwkst1 {
        &self.lpwkst1
    }
    #[doc = "0x10 - Low Power I/O Wakeup Enable Register 1. This register enables low power wakeup functionality for GPIO1."]
    #[inline(always)]
    pub const fn lpwken1(&self) -> &Lpwken1 {
        &self.lpwken1
    }
    #[doc = "0x14 - Low Power I/O Wakeup Status Register 2. This register indicates the low power wakeup status for GPIO2."]
    #[inline(always)]
    pub const fn lpwkst2(&self) -> &Lpwkst2 {
        &self.lpwkst2
    }
    #[doc = "0x18 - Low Power I/O Wakeup Enable Register 2. This register enables low power wakeup functionality for GPIO2."]
    #[inline(always)]
    pub const fn lpwken2(&self) -> &Lpwken2 {
        &self.lpwken2
    }
    #[doc = "0x1c - Low Power I/O Wakeup Status Register 3. This register indicates the low power wakeup status for GPIO3."]
    #[inline(always)]
    pub const fn lpwkst3(&self) -> &Lpwkst3 {
        &self.lpwkst3
    }
    #[doc = "0x20 - Low Power I/O Wakeup Enable Register 3. This register enables low power wakeup functionality for GPIO3."]
    #[inline(always)]
    pub const fn lpwken3(&self) -> &Lpwken3 {
        &self.lpwken3
    }
    #[doc = "0x30 - Low Power Peripheral Wakeup Status Register."]
    #[inline(always)]
    pub const fn lppwst(&self) -> &Lppwst {
        &self.lppwst
    }
    #[doc = "0x34 - Low Power Peripheral Wakeup Enable Register."]
    #[inline(always)]
    pub const fn lppwen(&self) -> &Lppwen {
        &self.lppwen
    }
    #[doc = "0x48 - General Purpose Register 0"]
    #[inline(always)]
    pub const fn gp0(&self) -> &Gp0 {
        &self.gp0
    }
    #[doc = "0x4c - General Purpose Register 1"]
    #[inline(always)]
    pub const fn gp1(&self) -> &Gp1 {
        &self.gp1
    }
}
#[doc = "LPCN (rw) register accessor: Low Power Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpcn`]
module"]
#[doc(alias = "LPCN")]
pub type Lpcn = crate::Reg<lpcn::LpcnSpec>;
#[doc = "Low Power Control Register."]
pub mod lpcn;
#[doc = "LPWKST0 (rw) register accessor: Low Power I/O Wakeup Status Register 0. This register indicates the low power wakeup status for GPIO0.\n\nYou can [`read`](crate::Reg::read) this register and get [`lpwkst0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpwkst0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpwkst0`]
module"]
#[doc(alias = "LPWKST0")]
pub type Lpwkst0 = crate::Reg<lpwkst0::Lpwkst0Spec>;
#[doc = "Low Power I/O Wakeup Status Register 0. This register indicates the low power wakeup status for GPIO0."]
pub mod lpwkst0;
#[doc = "LPWKEN0 (rw) register accessor: Low Power I/O Wakeup Enable Register 0. This register enables low power wakeup functionality for GPIO0.\n\nYou can [`read`](crate::Reg::read) this register and get [`lpwken0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpwken0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpwken0`]
module"]
#[doc(alias = "LPWKEN0")]
pub type Lpwken0 = crate::Reg<lpwken0::Lpwken0Spec>;
#[doc = "Low Power I/O Wakeup Enable Register 0. This register enables low power wakeup functionality for GPIO0."]
pub mod lpwken0;
pub use lpwken0 as lpwken1;
pub use lpwken0 as lpwken2;
pub use lpwken0 as lpwken3;
pub use lpwkst0 as lpwkst1;
pub use lpwkst0 as lpwkst2;
pub use lpwkst0 as lpwkst3;
pub use Lpwken0 as Lpwken1;
pub use Lpwken0 as Lpwken2;
pub use Lpwken0 as Lpwken3;
pub use Lpwkst0 as Lpwkst1;
pub use Lpwkst0 as Lpwkst2;
pub use Lpwkst0 as Lpwkst3;
#[doc = "LPPWST (rw) register accessor: Low Power Peripheral Wakeup Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lppwst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lppwst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lppwst`]
module"]
#[doc(alias = "LPPWST")]
pub type Lppwst = crate::Reg<lppwst::LppwstSpec>;
#[doc = "Low Power Peripheral Wakeup Status Register."]
pub mod lppwst;
#[doc = "LPPWEN (rw) register accessor: Low Power Peripheral Wakeup Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lppwen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lppwen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lppwen`]
module"]
#[doc(alias = "LPPWEN")]
pub type Lppwen = crate::Reg<lppwen::LppwenSpec>;
#[doc = "Low Power Peripheral Wakeup Enable Register."]
pub mod lppwen;
#[doc = "GP0 (rw) register accessor: General Purpose Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp0`]
module"]
#[doc(alias = "GP0")]
pub type Gp0 = crate::Reg<gp0::Gp0Spec>;
#[doc = "General Purpose Register 0"]
pub mod gp0;
#[doc = "GP1 (rw) register accessor: General Purpose Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp1`]
module"]
#[doc(alias = "GP1")]
pub type Gp1 = crate::Reg<gp1::Gp1Spec>;
#[doc = "General Purpose Register 1"]
pub mod gp1;
