#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    status: Status,
    int_en: IntEn,
    int_fl: IntFl,
    clkdiv: Clkdiv,
    osr: Osr,
    txpeek: Txpeek,
    pnr: Pnr,
    fifo: Fifo,
    _reserved9: [u8; 0x0c],
    dma: Dma,
    wken: Wken,
    wkfl: Wkfl,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - Interrupt Enable control register"]
    #[inline(always)]
    pub const fn int_en(&self) -> &IntEn {
        &self.int_en
    }
    #[doc = "0x0c - Interrupt status flags Control register"]
    #[inline(always)]
    pub const fn int_fl(&self) -> &IntFl {
        &self.int_fl
    }
    #[doc = "0x10 - Clock Divider register"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x14 - Over Sampling Rate register"]
    #[inline(always)]
    pub const fn osr(&self) -> &Osr {
        &self.osr
    }
    #[doc = "0x18 - TX FIFO Output Peek register"]
    #[inline(always)]
    pub const fn txpeek(&self) -> &Txpeek {
        &self.txpeek
    }
    #[doc = "0x1c - Pin register"]
    #[inline(always)]
    pub const fn pnr(&self) -> &Pnr {
        &self.pnr
    }
    #[doc = "0x20 - FIFO Read/Write register"]
    #[inline(always)]
    pub const fn fifo(&self) -> &Fifo {
        &self.fifo
    }
    #[doc = "0x30 - DMA Configuration register"]
    #[inline(always)]
    pub const fn dma(&self) -> &Dma {
        &self.dma
    }
    #[doc = "0x34 - Wake up enable Control register"]
    #[inline(always)]
    pub const fn wken(&self) -> &Wken {
        &self.wken
    }
    #[doc = "0x38 - Wake up Flags register"]
    #[inline(always)]
    pub const fn wkfl(&self) -> &Wkfl {
        &self.wkfl
    }
}
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status register"]
pub mod status;
#[doc = "INT_EN (rw) register accessor: Interrupt Enable control register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`]
module"]
#[doc(alias = "INT_EN")]
pub type IntEn = crate::Reg<int_en::IntEnSpec>;
#[doc = "Interrupt Enable control register"]
pub mod int_en;
#[doc = "INT_FL (rw) register accessor: Interrupt status flags Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_fl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_fl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_fl`]
module"]
#[doc(alias = "INT_FL")]
pub type IntFl = crate::Reg<int_fl::IntFlSpec>;
#[doc = "Interrupt status flags Control register"]
pub mod int_fl;
#[doc = "CLKDIV (rw) register accessor: Clock Divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`]
module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Clock Divider register"]
pub mod clkdiv;
#[doc = "OSR (rw) register accessor: Over Sampling Rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`osr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osr`]
module"]
#[doc(alias = "OSR")]
pub type Osr = crate::Reg<osr::OsrSpec>;
#[doc = "Over Sampling Rate register"]
pub mod osr;
#[doc = "TXPEEK (rw) register accessor: TX FIFO Output Peek register\n\nYou can [`read`](crate::Reg::read) this register and get [`txpeek::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpeek::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpeek`]
module"]
#[doc(alias = "TXPEEK")]
pub type Txpeek = crate::Reg<txpeek::TxpeekSpec>;
#[doc = "TX FIFO Output Peek register"]
pub mod txpeek;
#[doc = "PNR (rw) register accessor: Pin register\n\nYou can [`read`](crate::Reg::read) this register and get [`pnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pnr`]
module"]
#[doc(alias = "PNR")]
pub type Pnr = crate::Reg<pnr::PnrSpec>;
#[doc = "Pin register"]
pub mod pnr;
#[doc = "FIFO (rw) register accessor: FIFO Read/Write register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`]
module"]
#[doc(alias = "FIFO")]
pub type Fifo = crate::Reg<fifo::FifoSpec>;
#[doc = "FIFO Read/Write register"]
pub mod fifo;
#[doc = "DMA (rw) register accessor: DMA Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`]
module"]
#[doc(alias = "DMA")]
pub type Dma = crate::Reg<dma::DmaSpec>;
#[doc = "DMA Configuration register"]
pub mod dma;
#[doc = "WKEN (rw) register accessor: Wake up enable Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`wken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wken`]
module"]
#[doc(alias = "WKEN")]
pub type Wken = crate::Reg<wken::WkenSpec>;
#[doc = "Wake up enable Control register"]
pub mod wken;
#[doc = "WKFL (rw) register accessor: Wake up Flags register\n\nYou can [`read`](crate::Reg::read) this register and get [`wkfl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkfl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkfl`]
module"]
#[doc(alias = "WKFL")]
pub type Wkfl = crate::Reg<wkfl::WkflSpec>;
#[doc = "Wake up Flags register"]
pub mod wkfl;
