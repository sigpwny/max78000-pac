#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cnt: Cnt,
    cmp: Cmp,
    pwm: Pwm,
    intfl: Intfl,
    ctrl0: Ctrl0,
    nolcmp: Nolcmp,
    ctrl1: Ctrl1,
    wkfl: Wkfl,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer Counter Register."]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x04 - Timer Compare Register."]
    #[inline(always)]
    pub const fn cmp(&self) -> &Cmp {
        &self.cmp
    }
    #[doc = "0x08 - Timer PWM Register."]
    #[inline(always)]
    pub const fn pwm(&self) -> &Pwm {
        &self.pwm
    }
    #[doc = "0x0c - Timer Interrupt Status Register."]
    #[inline(always)]
    pub const fn intfl(&self) -> &Intfl {
        &self.intfl
    }
    #[doc = "0x10 - Timer Control Register."]
    #[inline(always)]
    pub const fn ctrl0(&self) -> &Ctrl0 {
        &self.ctrl0
    }
    #[doc = "0x14 - Timer Non-Overlapping Compare Register."]
    #[inline(always)]
    pub const fn nolcmp(&self) -> &Nolcmp {
        &self.nolcmp
    }
    #[doc = "0x18 - Timer Configuration Register."]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x1c - Timer Wakeup Status Register."]
    #[inline(always)]
    pub const fn wkfl(&self) -> &Wkfl {
        &self.wkfl
    }
}
#[doc = "CNT (rw) register accessor: Timer Counter Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Timer Counter Register."]
pub mod cnt;
#[doc = "CMP (rw) register accessor: Timer Compare Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp`]
module"]
#[doc(alias = "CMP")]
pub type Cmp = crate::Reg<cmp::CmpSpec>;
#[doc = "Timer Compare Register."]
pub mod cmp;
#[doc = "PWM (rw) register accessor: Timer PWM Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm`]
module"]
#[doc(alias = "PWM")]
pub type Pwm = crate::Reg<pwm::PwmSpec>;
#[doc = "Timer PWM Register."]
pub mod pwm;
#[doc = "INTFL (rw) register accessor: Timer Interrupt Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfl`]
module"]
#[doc(alias = "INTFL")]
pub type Intfl = crate::Reg<intfl::IntflSpec>;
#[doc = "Timer Interrupt Status Register."]
pub mod intfl;
#[doc = "CTRL0 (rw) register accessor: Timer Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl0`]
module"]
#[doc(alias = "CTRL0")]
pub type Ctrl0 = crate::Reg<ctrl0::Ctrl0Spec>;
#[doc = "Timer Control Register."]
pub mod ctrl0;
#[doc = "NOLCMP (rw) register accessor: Timer Non-Overlapping Compare Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`nolcmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nolcmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nolcmp`]
module"]
#[doc(alias = "NOLCMP")]
pub type Nolcmp = crate::Reg<nolcmp::NolcmpSpec>;
#[doc = "Timer Non-Overlapping Compare Register."]
pub mod nolcmp;
#[doc = "CTRL1 (rw) register accessor: Timer Configuration Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "Timer Configuration Register."]
pub mod ctrl1;
#[doc = "WKFL (rw) register accessor: Timer Wakeup Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`wkfl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkfl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkfl`]
module"]
#[doc(alias = "WKFL")]
pub type Wkfl = crate::Reg<wkfl::WkflSpec>;
#[doc = "Timer Wakeup Status Register."]
pub mod wkfl;
