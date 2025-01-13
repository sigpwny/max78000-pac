#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cnt: Cnt,
    cmp: Cmp,
    _reserved2: [u8; 0x04],
    intr: Intr,
    ctrl: Ctrl,
    nolcmp: Nolcmp,
    preset: Preset,
    reload: Reload,
    snapshot: Snapshot,
}
impl RegisterBlock {
    #[doc = "0x00 - Count. This register stores the current timer count."]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x04 - Compare. This register stores the compare value, which is used to set the maximum count value to initiate a reload of the timer to 0x0001."]
    #[inline(always)]
    pub const fn cmp(&self) -> &Cmp {
        &self.cmp
    }
    #[doc = "0x0c - Clear Interrupt. Writing a value (0 or 1) to a bit in this register clears the associated interrupt."]
    #[inline(always)]
    pub const fn intr(&self) -> &Intr {
        &self.intr
    }
    #[doc = "0x10 - Timer Control Register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x14 - Timer Non-Overlapping Compare Register."]
    #[inline(always)]
    pub const fn nolcmp(&self) -> &Nolcmp {
        &self.nolcmp
    }
    #[doc = "0x18 - Preset register."]
    #[inline(always)]
    pub const fn preset(&self) -> &Preset {
        &self.preset
    }
    #[doc = "0x1c - Reload register."]
    #[inline(always)]
    pub const fn reload(&self) -> &Reload {
        &self.reload
    }
    #[doc = "0x20 - Snapshot register."]
    #[inline(always)]
    pub const fn snapshot(&self) -> &Snapshot {
        &self.snapshot
    }
}
#[doc = "CNT (rw) register accessor: Count. This register stores the current timer count.\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Count. This register stores the current timer count."]
pub mod cnt;
#[doc = "CMP (rw) register accessor: Compare. This register stores the compare value, which is used to set the maximum count value to initiate a reload of the timer to 0x0001.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp`]
module"]
#[doc(alias = "CMP")]
pub type Cmp = crate::Reg<cmp::CmpSpec>;
#[doc = "Compare. This register stores the compare value, which is used to set the maximum count value to initiate a reload of the timer to 0x0001."]
pub mod cmp;
#[doc = "INTR (rw) register accessor: Clear Interrupt. Writing a value (0 or 1) to a bit in this register clears the associated interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
#[doc(alias = "INTR")]
pub type Intr = crate::Reg<intr::IntrSpec>;
#[doc = "Clear Interrupt. Writing a value (0 or 1) to a bit in this register clears the associated interrupt."]
pub mod intr;
#[doc = "CTRL (rw) register accessor: Timer Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Timer Control Register."]
pub mod ctrl;
#[doc = "NOLCMP (rw) register accessor: Timer Non-Overlapping Compare Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`nolcmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nolcmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nolcmp`]
module"]
#[doc(alias = "NOLCMP")]
pub type Nolcmp = crate::Reg<nolcmp::NolcmpSpec>;
#[doc = "Timer Non-Overlapping Compare Register."]
pub mod nolcmp;
#[doc = "PRESET (rw) register accessor: Preset register.\n\nYou can [`read`](crate::Reg::read) this register and get [`preset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`preset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preset`]
module"]
#[doc(alias = "PRESET")]
pub type Preset = crate::Reg<preset::PresetSpec>;
#[doc = "Preset register."]
pub mod preset;
#[doc = "RELOAD (rw) register accessor: Reload register.\n\nYou can [`read`](crate::Reg::read) this register and get [`reload::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reload::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reload`]
module"]
#[doc(alias = "RELOAD")]
pub type Reload = crate::Reg<reload::ReloadSpec>;
#[doc = "Reload register."]
pub mod reload;
#[doc = "SNAPSHOT (rw) register accessor: Snapshot register.\n\nYou can [`read`](crate::Reg::read) this register and get [`snapshot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snapshot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snapshot`]
module"]
#[doc(alias = "SNAPSHOT")]
pub type Snapshot = crate::Reg<snapshot::SnapshotSpec>;
#[doc = "Snapshot register."]
pub mod snapshot;
