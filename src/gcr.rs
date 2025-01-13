#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sysctrl: Sysctrl,
    rst0: Rst0,
    clkctrl: Clkctrl,
    pm: Pm,
    _reserved4: [u8; 0x08],
    pclkdiv: Pclkdiv,
    _reserved5: [u8; 0x08],
    pclkdis0: Pclkdis0,
    memctrl: Memctrl,
    memz: Memz,
    _reserved8: [u8; 0x10],
    sysst: Sysst,
    rst1: Rst1,
    pclkdis1: Pclkdis1,
    eventen: Eventen,
    revision: Revision,
    sysie: Sysie,
    _reserved14: [u8; 0x0c],
    eccerr: Eccerr,
    eccced: Eccced,
    eccie: Eccie,
    eccaddr: Eccaddr,
    _reserved18: [u8; 0x0c],
    gpr: Gpr,
}
impl RegisterBlock {
    #[doc = "0x00 - System Control."]
    #[inline(always)]
    pub const fn sysctrl(&self) -> &Sysctrl {
        &self.sysctrl
    }
    #[doc = "0x04 - Reset."]
    #[inline(always)]
    pub const fn rst0(&self) -> &Rst0 {
        &self.rst0
    }
    #[doc = "0x08 - Clock Control."]
    #[inline(always)]
    pub const fn clkctrl(&self) -> &Clkctrl {
        &self.clkctrl
    }
    #[doc = "0x0c - Power Management."]
    #[inline(always)]
    pub const fn pm(&self) -> &Pm {
        &self.pm
    }
    #[doc = "0x18 - Peripheral Clock Divider."]
    #[inline(always)]
    pub const fn pclkdiv(&self) -> &Pclkdiv {
        &self.pclkdiv
    }
    #[doc = "0x24 - Peripheral Clock Disable."]
    #[inline(always)]
    pub const fn pclkdis0(&self) -> &Pclkdis0 {
        &self.pclkdis0
    }
    #[doc = "0x28 - Memory Clock Control Register."]
    #[inline(always)]
    pub const fn memctrl(&self) -> &Memctrl {
        &self.memctrl
    }
    #[doc = "0x2c - Memory Zeroize Control."]
    #[inline(always)]
    pub const fn memz(&self) -> &Memz {
        &self.memz
    }
    #[doc = "0x40 - System Status Register."]
    #[inline(always)]
    pub const fn sysst(&self) -> &Sysst {
        &self.sysst
    }
    #[doc = "0x44 - Reset 1."]
    #[inline(always)]
    pub const fn rst1(&self) -> &Rst1 {
        &self.rst1
    }
    #[doc = "0x48 - Peripheral Clock Disable."]
    #[inline(always)]
    pub const fn pclkdis1(&self) -> &Pclkdis1 {
        &self.pclkdis1
    }
    #[doc = "0x4c - Event Enable Register."]
    #[inline(always)]
    pub const fn eventen(&self) -> &Eventen {
        &self.eventen
    }
    #[doc = "0x50 - Revision Register."]
    #[inline(always)]
    pub const fn revision(&self) -> &Revision {
        &self.revision
    }
    #[doc = "0x54 - System Status Interrupt Enable Register."]
    #[inline(always)]
    pub const fn sysie(&self) -> &Sysie {
        &self.sysie
    }
    #[doc = "0x64 - ECC Error Register"]
    #[inline(always)]
    pub const fn eccerr(&self) -> &Eccerr {
        &self.eccerr
    }
    #[doc = "0x68 - ECC Not Double Error Detect Register"]
    #[inline(always)]
    pub const fn eccced(&self) -> &Eccced {
        &self.eccced
    }
    #[doc = "0x6c - ECC IRQ Enable Register"]
    #[inline(always)]
    pub const fn eccie(&self) -> &Eccie {
        &self.eccie
    }
    #[doc = "0x70 - ECC Error Address Register"]
    #[inline(always)]
    pub const fn eccaddr(&self) -> &Eccaddr {
        &self.eccaddr
    }
    #[doc = "0x80 - General Purpose Register."]
    #[inline(always)]
    pub const fn gpr(&self) -> &Gpr {
        &self.gpr
    }
}
#[doc = "SYSCTRL (rw) register accessor: System Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysctrl`]
module"]
#[doc(alias = "SYSCTRL")]
pub type Sysctrl = crate::Reg<sysctrl::SysctrlSpec>;
#[doc = "System Control."]
pub mod sysctrl;
#[doc = "RST0 (rw) register accessor: Reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`rst0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst0`]
module"]
#[doc(alias = "RST0")]
pub type Rst0 = crate::Reg<rst0::Rst0Spec>;
#[doc = "Reset."]
pub mod rst0;
#[doc = "CLKCTRL (rw) register accessor: Clock Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctrl`]
module"]
#[doc(alias = "CLKCTRL")]
pub type Clkctrl = crate::Reg<clkctrl::ClkctrlSpec>;
#[doc = "Clock Control."]
pub mod clkctrl;
#[doc = "PM (rw) register accessor: Power Management.\n\nYou can [`read`](crate::Reg::read) this register and get [`pm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pm`]
module"]
#[doc(alias = "PM")]
pub type Pm = crate::Reg<pm::PmSpec>;
#[doc = "Power Management."]
pub mod pm;
#[doc = "PCLKDIV (rw) register accessor: Peripheral Clock Divider.\n\nYou can [`read`](crate::Reg::read) this register and get [`pclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pclkdiv`]
module"]
#[doc(alias = "PCLKDIV")]
pub type Pclkdiv = crate::Reg<pclkdiv::PclkdivSpec>;
#[doc = "Peripheral Clock Divider."]
pub mod pclkdiv;
#[doc = "PCLKDIS0 (rw) register accessor: Peripheral Clock Disable.\n\nYou can [`read`](crate::Reg::read) this register and get [`pclkdis0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pclkdis0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pclkdis0`]
module"]
#[doc(alias = "PCLKDIS0")]
pub type Pclkdis0 = crate::Reg<pclkdis0::Pclkdis0Spec>;
#[doc = "Peripheral Clock Disable."]
pub mod pclkdis0;
#[doc = "MEMCTRL (rw) register accessor: Memory Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`memctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memctrl`]
module"]
#[doc(alias = "MEMCTRL")]
pub type Memctrl = crate::Reg<memctrl::MemctrlSpec>;
#[doc = "Memory Clock Control Register."]
pub mod memctrl;
#[doc = "MEMZ (rw) register accessor: Memory Zeroize Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`memz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memz`]
module"]
#[doc(alias = "MEMZ")]
pub type Memz = crate::Reg<memz::MemzSpec>;
#[doc = "Memory Zeroize Control."]
pub mod memz;
#[doc = "SYSST (rw) register accessor: System Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sysst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysst`]
module"]
#[doc(alias = "SYSST")]
pub type Sysst = crate::Reg<sysst::SysstSpec>;
#[doc = "System Status Register."]
pub mod sysst;
#[doc = "RST1 (rw) register accessor: Reset 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rst1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst1`]
module"]
#[doc(alias = "RST1")]
pub type Rst1 = crate::Reg<rst1::Rst1Spec>;
#[doc = "Reset 1."]
pub mod rst1;
#[doc = "PCLKDIS1 (rw) register accessor: Peripheral Clock Disable.\n\nYou can [`read`](crate::Reg::read) this register and get [`pclkdis1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pclkdis1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pclkdis1`]
module"]
#[doc(alias = "PCLKDIS1")]
pub type Pclkdis1 = crate::Reg<pclkdis1::Pclkdis1Spec>;
#[doc = "Peripheral Clock Disable."]
pub mod pclkdis1;
#[doc = "EVENTEN (rw) register accessor: Event Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`eventen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eventen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eventen`]
module"]
#[doc(alias = "EVENTEN")]
pub type Eventen = crate::Reg<eventen::EventenSpec>;
#[doc = "Event Enable Register."]
pub mod eventen;
#[doc = "REVISION (r) register accessor: Revision Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`revision::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revision`]
module"]
#[doc(alias = "REVISION")]
pub type Revision = crate::Reg<revision::RevisionSpec>;
#[doc = "Revision Register."]
pub mod revision;
#[doc = "SYSIE (rw) register accessor: System Status Interrupt Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sysie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysie`]
module"]
#[doc(alias = "SYSIE")]
pub type Sysie = crate::Reg<sysie::SysieSpec>;
#[doc = "System Status Interrupt Enable Register."]
pub mod sysie;
#[doc = "ECCERR (rw) register accessor: ECC Error Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccerr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccerr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccerr`]
module"]
#[doc(alias = "ECCERR")]
pub type Eccerr = crate::Reg<eccerr::EccerrSpec>;
#[doc = "ECC Error Register"]
pub mod eccerr;
#[doc = "ECCCED (rw) register accessor: ECC Not Double Error Detect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccced::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccced::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccced`]
module"]
#[doc(alias = "ECCCED")]
pub type Eccced = crate::Reg<eccced::EcccedSpec>;
#[doc = "ECC Not Double Error Detect Register"]
pub mod eccced;
#[doc = "ECCIE (rw) register accessor: ECC IRQ Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccie`]
module"]
#[doc(alias = "ECCIE")]
pub type Eccie = crate::Reg<eccie::EccieSpec>;
#[doc = "ECC IRQ Enable Register"]
pub mod eccie;
#[doc = "ECCADDR (rw) register accessor: ECC Error Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccaddr`]
module"]
#[doc(alias = "ECCADDR")]
pub type Eccaddr = crate::Reg<eccaddr::EccaddrSpec>;
#[doc = "ECC Error Address Register"]
pub mod eccaddr;
#[doc = "GPR (rw) register accessor: General Purpose Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpr`]
module"]
#[doc(alias = "GPR")]
pub type Gpr = crate::Reg<gpr::GprSpec>;
#[doc = "General Purpose Register."]
pub mod gpr;
