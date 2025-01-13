#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ver: Ver,
    fifo_size: FifoSize,
    ctrl: Ctrl,
    int_en: IntEn,
    int_fl: IntFl,
    ds_timing_codes: DsTimingCodes,
    _reserved6: [u8; 0x18],
    fifo_data: FifoData,
}
impl RegisterBlock {
    #[doc = "0x00 - Hardware Version."]
    #[inline(always)]
    pub const fn ver(&self) -> &Ver {
        &self.ver
    }
    #[doc = "0x04 - FIFO Depth."]
    #[inline(always)]
    pub const fn fifo_size(&self) -> &FifoSize {
        &self.fifo_size
    }
    #[doc = "0x08 - Control Register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - Interupt Enable Register."]
    #[inline(always)]
    pub const fn int_en(&self) -> &IntEn {
        &self.int_en
    }
    #[doc = "0x10 - Interupt Flag Register."]
    #[inline(always)]
    pub const fn int_fl(&self) -> &IntFl {
        &self.int_fl
    }
    #[doc = "0x14 - DS Timing Code Register."]
    #[inline(always)]
    pub const fn ds_timing_codes(&self) -> &DsTimingCodes {
        &self.ds_timing_codes
    }
    #[doc = "0x30 - FIFO DATA Register."]
    #[inline(always)]
    pub const fn fifo_data(&self) -> &FifoData {
        &self.fifo_data
    }
}
#[doc = "VER (rw) register accessor: Hardware Version.\n\nYou can [`read`](crate::Reg::read) this register and get [`ver::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ver::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ver`]
module"]
#[doc(alias = "VER")]
pub type Ver = crate::Reg<ver::VerSpec>;
#[doc = "Hardware Version."]
pub mod ver;
#[doc = "FIFO_SIZE (rw) register accessor: FIFO Depth.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_size`]
module"]
#[doc(alias = "FIFO_SIZE")]
pub type FifoSize = crate::Reg<fifo_size::FifoSizeSpec>;
#[doc = "FIFO Depth."]
pub mod fifo_size;
#[doc = "CTRL (rw) register accessor: Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register."]
pub mod ctrl;
#[doc = "INT_EN (rw) register accessor: Interupt Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`]
module"]
#[doc(alias = "INT_EN")]
pub type IntEn = crate::Reg<int_en::IntEnSpec>;
#[doc = "Interupt Enable Register."]
pub mod int_en;
#[doc = "INT_FL (rw) register accessor: Interupt Flag Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_fl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_fl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_fl`]
module"]
#[doc(alias = "INT_FL")]
pub type IntFl = crate::Reg<int_fl::IntFlSpec>;
#[doc = "Interupt Flag Register."]
pub mod int_fl;
#[doc = "DS_TIMING_CODES (rw) register accessor: DS Timing Code Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ds_timing_codes::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ds_timing_codes::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ds_timing_codes`]
module"]
#[doc(alias = "DS_TIMING_CODES")]
pub type DsTimingCodes = crate::Reg<ds_timing_codes::DsTimingCodesSpec>;
#[doc = "DS Timing Code Register."]
pub mod ds_timing_codes;
#[doc = "FIFO_DATA (rw) register accessor: FIFO DATA Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_data`]
module"]
#[doc(alias = "FIFO_DATA")]
pub type FifoData = crate::Reg<fifo_data::FifoDataSpec>;
#[doc = "FIFO DATA Register."]
pub mod fifo_data;
