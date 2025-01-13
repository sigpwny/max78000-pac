#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    enable: Enable,
    resync: Resync,
    intfl: Intfl,
    inten: Inten,
    safe_en: SafeEn,
    safe_dis: SafeDis,
}
impl RegisterBlock {
    #[doc = "0x00 - Global Enable/Disable Controls for All Pulse Trains"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x04 - Global Resync (All Pulse Trains) Control"]
    #[inline(always)]
    pub const fn resync(&self) -> &Resync {
        &self.resync
    }
    #[doc = "0x08 - Pulse Train Interrupt Flags"]
    #[inline(always)]
    pub const fn intfl(&self) -> &Intfl {
        &self.intfl
    }
    #[doc = "0x0c - Pulse Train Interrupt Enable/Disable"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x10 - Pulse Train Global Safe Enable."]
    #[inline(always)]
    pub const fn safe_en(&self) -> &SafeEn {
        &self.safe_en
    }
    #[doc = "0x14 - Pulse Train Global Safe Disable."]
    #[inline(always)]
    pub const fn safe_dis(&self) -> &SafeDis {
        &self.safe_dis
    }
}
#[doc = "ENABLE (rw) register accessor: Global Enable/Disable Controls for All Pulse Trains\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Global Enable/Disable Controls for All Pulse Trains"]
pub mod enable;
#[doc = "RESYNC (rw) register accessor: Global Resync (All Pulse Trains) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`resync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resync`]
module"]
#[doc(alias = "RESYNC")]
pub type Resync = crate::Reg<resync::ResyncSpec>;
#[doc = "Global Resync (All Pulse Trains) Control"]
pub mod resync;
#[doc = "INTFL (rw) register accessor: Pulse Train Interrupt Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfl`]
module"]
#[doc(alias = "INTFL")]
pub type Intfl = crate::Reg<intfl::IntflSpec>;
#[doc = "Pulse Train Interrupt Flags"]
pub mod intfl;
#[doc = "INTEN (rw) register accessor: Pulse Train Interrupt Enable/Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Pulse Train Interrupt Enable/Disable"]
pub mod inten;
#[doc = "SAFE_EN (w) register accessor: Pulse Train Global Safe Enable.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`safe_en::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@safe_en`]
module"]
#[doc(alias = "SAFE_EN")]
pub type SafeEn = crate::Reg<safe_en::SafeEnSpec>;
#[doc = "Pulse Train Global Safe Enable."]
pub mod safe_en;
#[doc = "SAFE_DIS (w) register accessor: Pulse Train Global Safe Disable.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`safe_dis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@safe_dis`]
module"]
#[doc(alias = "SAFE_DIS")]
pub type SafeDis = crate::Reg<safe_dis::SafeDisSpec>;
#[doc = "Pulse Train Global Safe Disable."]
pub mod safe_dis;
