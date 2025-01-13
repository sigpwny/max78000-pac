#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl0ch0: Ctrl0ch0,
    _reserved1: [u8; 0x0c],
    ctrl1ch0: Ctrl1ch0,
    _reserved2: [u8; 0x0c],
    filtch0: Filtch0,
    _reserved3: [u8; 0x0c],
    dmach0: Dmach0,
    _reserved4: [u8; 0x0c],
    fifoch0: Fifoch0,
    _reserved5: [u8; 0x0c],
    intfl: Intfl,
    inten: Inten,
    extsetup: Extsetup,
    wken: Wken,
    wkfl: Wkfl,
}
impl RegisterBlock {
    #[doc = "0x00 - Global mode channel."]
    #[inline(always)]
    pub const fn ctrl0ch0(&self) -> &Ctrl0ch0 {
        &self.ctrl0ch0
    }
    #[doc = "0x10 - Local channel Setup."]
    #[inline(always)]
    pub const fn ctrl1ch0(&self) -> &Ctrl1ch0 {
        &self.ctrl1ch0
    }
    #[doc = "0x20 - Filter."]
    #[inline(always)]
    pub const fn filtch0(&self) -> &Filtch0 {
        &self.filtch0
    }
    #[doc = "0x30 - DMA Control."]
    #[inline(always)]
    pub const fn dmach0(&self) -> &Dmach0 {
        &self.dmach0
    }
    #[doc = "0x40 - I2S Fifo."]
    #[inline(always)]
    pub const fn fifoch0(&self) -> &Fifoch0 {
        &self.fifoch0
    }
    #[doc = "0x50 - ISR Status."]
    #[inline(always)]
    pub const fn intfl(&self) -> &Intfl {
        &self.intfl
    }
    #[doc = "0x54 - Interrupt Enable."]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x58 - Ext Control."]
    #[inline(always)]
    pub const fn extsetup(&self) -> &Extsetup {
        &self.extsetup
    }
    #[doc = "0x5c - Wakeup Enable."]
    #[inline(always)]
    pub const fn wken(&self) -> &Wken {
        &self.wken
    }
    #[doc = "0x60 - Wakeup Flags."]
    #[inline(always)]
    pub const fn wkfl(&self) -> &Wkfl {
        &self.wkfl
    }
}
#[doc = "CTRL0CH0 (rw) register accessor: Global mode channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl0ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl0ch0`]
module"]
#[doc(alias = "CTRL0CH0")]
pub type Ctrl0ch0 = crate::Reg<ctrl0ch0::Ctrl0ch0Spec>;
#[doc = "Global mode channel."]
pub mod ctrl0ch0;
#[doc = "CTRL1CH0 (rw) register accessor: Local channel Setup.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1ch0`]
module"]
#[doc(alias = "CTRL1CH0")]
pub type Ctrl1ch0 = crate::Reg<ctrl1ch0::Ctrl1ch0Spec>;
#[doc = "Local channel Setup."]
pub mod ctrl1ch0;
#[doc = "FILTCH0 (rw) register accessor: Filter.\n\nYou can [`read`](crate::Reg::read) this register and get [`filtch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filtch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filtch0`]
module"]
#[doc(alias = "FILTCH0")]
pub type Filtch0 = crate::Reg<filtch0::Filtch0Spec>;
#[doc = "Filter."]
pub mod filtch0;
#[doc = "DMACH0 (rw) register accessor: DMA Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmach0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmach0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach0`]
module"]
#[doc(alias = "DMACH0")]
pub type Dmach0 = crate::Reg<dmach0::Dmach0Spec>;
#[doc = "DMA Control."]
pub mod dmach0;
#[doc = "FIFOCH0 (rw) register accessor: I2S Fifo.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoch0`]
module"]
#[doc(alias = "FIFOCH0")]
pub type Fifoch0 = crate::Reg<fifoch0::Fifoch0Spec>;
#[doc = "I2S Fifo."]
pub mod fifoch0;
#[doc = "INTFL (rw) register accessor: ISR Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfl`]
module"]
#[doc(alias = "INTFL")]
pub type Intfl = crate::Reg<intfl::IntflSpec>;
#[doc = "ISR Status."]
pub mod intfl;
#[doc = "INTEN (rw) register accessor: Interrupt Enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt Enable."]
pub mod inten;
#[doc = "EXTSETUP (rw) register accessor: Ext Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`extsetup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extsetup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extsetup`]
module"]
#[doc(alias = "EXTSETUP")]
pub type Extsetup = crate::Reg<extsetup::ExtsetupSpec>;
#[doc = "Ext Control."]
pub mod extsetup;
#[doc = "WKEN (rw) register accessor: Wakeup Enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`wken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wken`]
module"]
#[doc(alias = "WKEN")]
pub type Wken = crate::Reg<wken::WkenSpec>;
#[doc = "Wakeup Enable."]
pub mod wken;
#[doc = "WKFL (rw) register accessor: Wakeup Flags.\n\nYou can [`read`](crate::Reg::read) this register and get [`wkfl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkfl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkfl`]
module"]
#[doc(alias = "WKFL")]
pub type Wkfl = crate::Reg<wkfl::WkflSpec>;
#[doc = "Wakeup Flags."]
pub mod wkfl;
