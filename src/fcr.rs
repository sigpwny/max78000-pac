#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fctrl0: Fctrl0,
    autocal0: Autocal0,
    autocal1: Autocal1,
    autocal2: Autocal2,
    urvbootaddr: Urvbootaddr,
    urvctrl: Urvctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Function Control 0."]
    #[inline(always)]
    pub const fn fctrl0(&self) -> &Fctrl0 {
        &self.fctrl0
    }
    #[doc = "0x04 - Automatic Calibration 0."]
    #[inline(always)]
    pub const fn autocal0(&self) -> &Autocal0 {
        &self.autocal0
    }
    #[doc = "0x08 - Automatic Calibration 1."]
    #[inline(always)]
    pub const fn autocal1(&self) -> &Autocal1 {
        &self.autocal1
    }
    #[doc = "0x0c - Automatic Calibration 2"]
    #[inline(always)]
    pub const fn autocal2(&self) -> &Autocal2 {
        &self.autocal2
    }
    #[doc = "0x10 - RISC-V Boot Address."]
    #[inline(always)]
    pub const fn urvbootaddr(&self) -> &Urvbootaddr {
        &self.urvbootaddr
    }
    #[doc = "0x14 - RISC-V Control Register."]
    #[inline(always)]
    pub const fn urvctrl(&self) -> &Urvctrl {
        &self.urvctrl
    }
}
#[doc = "FCTRL0 (rw) register accessor: Function Control 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrl0`]
module"]
#[doc(alias = "FCTRL0")]
pub type Fctrl0 = crate::Reg<fctrl0::Fctrl0Spec>;
#[doc = "Function Control 0."]
pub mod fctrl0;
#[doc = "AUTOCAL0 (rw) register accessor: Automatic Calibration 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`autocal0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocal0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocal0`]
module"]
#[doc(alias = "AUTOCAL0")]
pub type Autocal0 = crate::Reg<autocal0::Autocal0Spec>;
#[doc = "Automatic Calibration 0."]
pub mod autocal0;
#[doc = "AUTOCAL1 (rw) register accessor: Automatic Calibration 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`autocal1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocal1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocal1`]
module"]
#[doc(alias = "AUTOCAL1")]
pub type Autocal1 = crate::Reg<autocal1::Autocal1Spec>;
#[doc = "Automatic Calibration 1."]
pub mod autocal1;
#[doc = "AUTOCAL2 (rw) register accessor: Automatic Calibration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`autocal2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocal2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocal2`]
module"]
#[doc(alias = "AUTOCAL2")]
pub type Autocal2 = crate::Reg<autocal2::Autocal2Spec>;
#[doc = "Automatic Calibration 2"]
pub mod autocal2;
#[doc = "URVBOOTADDR (rw) register accessor: RISC-V Boot Address.\n\nYou can [`read`](crate::Reg::read) this register and get [`urvbootaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urvbootaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urvbootaddr`]
module"]
#[doc(alias = "URVBOOTADDR")]
pub type Urvbootaddr = crate::Reg<urvbootaddr::UrvbootaddrSpec>;
#[doc = "RISC-V Boot Address."]
pub mod urvbootaddr;
#[doc = "URVCTRL (rw) register accessor: RISC-V Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`urvctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`urvctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@urvctrl`]
module"]
#[doc(alias = "URVCTRL")]
pub type Urvctrl = crate::Reg<urvctrl::UrvctrlSpec>;
#[doc = "RISC-V Control Register."]
pub mod urvctrl;
