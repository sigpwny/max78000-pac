#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sec: Sec,
    ssec: Ssec,
    toda: Toda,
    sseca: Sseca,
    ctrl: Ctrl,
    trim: Trim,
    oscctrl: Oscctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - RTC Second Counter. This register contains the 32-bit second counter."]
    #[inline(always)]
    pub const fn sec(&self) -> &Sec {
        &self.sec
    }
    #[doc = "0x04 - RTC Sub-second Counter. This counter increments at 256Hz. RTC_SEC is incremented when this register rolls over from 0xFF to 0x00."]
    #[inline(always)]
    pub const fn ssec(&self) -> &Ssec {
        &self.ssec
    }
    #[doc = "0x08 - Time-of-day Alarm."]
    #[inline(always)]
    pub const fn toda(&self) -> &Toda {
        &self.toda
    }
    #[doc = "0x0c - RTC sub-second alarm. This register contains the reload value for the sub-second alarm."]
    #[inline(always)]
    pub const fn sseca(&self) -> &Sseca {
        &self.sseca
    }
    #[doc = "0x10 - RTC Control Register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x14 - RTC Trim Register."]
    #[inline(always)]
    pub const fn trim(&self) -> &Trim {
        &self.trim
    }
    #[doc = "0x18 - RTC Oscillator Control Register."]
    #[inline(always)]
    pub const fn oscctrl(&self) -> &Oscctrl {
        &self.oscctrl
    }
}
#[doc = "SEC (rw) register accessor: RTC Second Counter. This register contains the 32-bit second counter.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec`]
module"]
#[doc(alias = "SEC")]
pub type Sec = crate::Reg<sec::SecSpec>;
#[doc = "RTC Second Counter. This register contains the 32-bit second counter."]
pub mod sec;
#[doc = "SSEC (rw) register accessor: RTC Sub-second Counter. This counter increments at 256Hz. RTC_SEC is incremented when this register rolls over from 0xFF to 0x00.\n\nYou can [`read`](crate::Reg::read) this register and get [`ssec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssec`]
module"]
#[doc(alias = "SSEC")]
pub type Ssec = crate::Reg<ssec::SsecSpec>;
#[doc = "RTC Sub-second Counter. This counter increments at 256Hz. RTC_SEC is incremented when this register rolls over from 0xFF to 0x00."]
pub mod ssec;
#[doc = "TODA (rw) register accessor: Time-of-day Alarm.\n\nYou can [`read`](crate::Reg::read) this register and get [`toda::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`toda::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@toda`]
module"]
#[doc(alias = "TODA")]
pub type Toda = crate::Reg<toda::TodaSpec>;
#[doc = "Time-of-day Alarm."]
pub mod toda;
#[doc = "SSECA (rw) register accessor: RTC sub-second alarm. This register contains the reload value for the sub-second alarm.\n\nYou can [`read`](crate::Reg::read) this register and get [`sseca::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sseca::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sseca`]
module"]
#[doc(alias = "SSECA")]
pub type Sseca = crate::Reg<sseca::SsecaSpec>;
#[doc = "RTC sub-second alarm. This register contains the reload value for the sub-second alarm."]
pub mod sseca;
#[doc = "CTRL (rw) register accessor: RTC Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "RTC Control Register."]
pub mod ctrl;
#[doc = "TRIM (rw) register accessor: RTC Trim Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`trim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trim`]
module"]
#[doc(alias = "TRIM")]
pub type Trim = crate::Reg<trim::TrimSpec>;
#[doc = "RTC Trim Register."]
pub mod trim;
#[doc = "OSCCTRL (rw) register accessor: RTC Oscillator Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`oscctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oscctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscctrl`]
module"]
#[doc(alias = "OSCCTRL")]
pub type Oscctrl = crate::Reg<oscctrl::OscctrlSpec>;
#[doc = "RTC Oscillator Control Register."]
pub mod oscctrl;
