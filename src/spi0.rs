#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_fifo8: [u8; 0x04],
    ctrl0: Ctrl0,
    ctrl1: Ctrl1,
    ctrl2: Ctrl2,
    sstime: Sstime,
    clkctrl: Clkctrl,
    _reserved6: [u8; 0x04],
    dma: Dma,
    intfl: Intfl,
    inten: Inten,
    wkfl: Wkfl,
    wken: Wken,
    stat: Stat,
}
impl RegisterBlock {
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub const fn fifo8(&self, n: usize) -> &Fifo8 {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub fn fifo8_iter(&self) -> impl Iterator<Item = &Fifo8> {
        (0..4).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(n).cast() })
    }
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub const fn fifo16(&self, n: usize) -> &Fifo16 {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub fn fifo16_iter(&self) -> impl Iterator<Item = &Fifo16> {
        (0..2).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2 * n).cast() })
    }
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub const fn fifo32(&self) -> &Fifo32 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x04 - Register for controlling SPI peripheral."]
    #[inline(always)]
    pub const fn ctrl0(&self) -> &Ctrl0 {
        &self.ctrl0
    }
    #[doc = "0x08 - Register for controlling SPI peripheral."]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x0c - Register for controlling SPI peripheral."]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x10 - Register for controlling SPI peripheral/Slave Select Timing."]
    #[inline(always)]
    pub const fn sstime(&self) -> &Sstime {
        &self.sstime
    }
    #[doc = "0x14 - Register for controlling SPI clock rate."]
    #[inline(always)]
    pub const fn clkctrl(&self) -> &Clkctrl {
        &self.clkctrl
    }
    #[doc = "0x1c - Register for controlling DMA."]
    #[inline(always)]
    pub const fn dma(&self) -> &Dma {
        &self.dma
    }
    #[doc = "0x20 - Register for reading and clearing interrupt flags. All bits are write 1 to clear."]
    #[inline(always)]
    pub const fn intfl(&self) -> &Intfl {
        &self.intfl
    }
    #[doc = "0x24 - Register for enabling interrupts."]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x28 - Register for wake up flags. All bits in this register are write 1 to clear."]
    #[inline(always)]
    pub const fn wkfl(&self) -> &Wkfl {
        &self.wkfl
    }
    #[doc = "0x2c - Register for wake up enable."]
    #[inline(always)]
    pub const fn wken(&self) -> &Wken {
        &self.wken
    }
    #[doc = "0x30 - SPI Status register."]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
}
#[doc = "FIFO32 (rw) register accessor: Register for reading and writing the FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo32`]
module"]
#[doc(alias = "FIFO32")]
pub type Fifo32 = crate::Reg<fifo32::Fifo32Spec>;
#[doc = "Register for reading and writing the FIFO."]
pub mod fifo32;
#[doc = "FIFO16 (rw) register accessor: Register for reading and writing the FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo16`]
module"]
#[doc(alias = "FIFO16")]
pub type Fifo16 = crate::Reg<fifo16::Fifo16Spec>;
#[doc = "Register for reading and writing the FIFO."]
pub mod fifo16;
#[doc = "FIFO8 (rw) register accessor: Register for reading and writing the FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo8`]
module"]
#[doc(alias = "FIFO8")]
pub type Fifo8 = crate::Reg<fifo8::Fifo8Spec>;
#[doc = "Register for reading and writing the FIFO."]
pub mod fifo8;
#[doc = "CTRL0 (rw) register accessor: Register for controlling SPI peripheral.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl0`]
module"]
#[doc(alias = "CTRL0")]
pub type Ctrl0 = crate::Reg<ctrl0::Ctrl0Spec>;
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl0;
#[doc = "CTRL1 (rw) register accessor: Register for controlling SPI peripheral.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: Register for controlling SPI peripheral.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`]
module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl2;
#[doc = "SSTIME (rw) register accessor: Register for controlling SPI peripheral/Slave Select Timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`sstime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sstime`]
module"]
#[doc(alias = "SSTIME")]
pub type Sstime = crate::Reg<sstime::SstimeSpec>;
#[doc = "Register for controlling SPI peripheral/Slave Select Timing."]
pub mod sstime;
#[doc = "CLKCTRL (rw) register accessor: Register for controlling SPI clock rate.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctrl`]
module"]
#[doc(alias = "CLKCTRL")]
pub type Clkctrl = crate::Reg<clkctrl::ClkctrlSpec>;
#[doc = "Register for controlling SPI clock rate."]
pub mod clkctrl;
#[doc = "DMA (rw) register accessor: Register for controlling DMA.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`]
module"]
#[doc(alias = "DMA")]
pub type Dma = crate::Reg<dma::DmaSpec>;
#[doc = "Register for controlling DMA."]
pub mod dma;
#[doc = "INTFL (rw) register accessor: Register for reading and clearing interrupt flags. All bits are write 1 to clear.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfl`]
module"]
#[doc(alias = "INTFL")]
pub type Intfl = crate::Reg<intfl::IntflSpec>;
#[doc = "Register for reading and clearing interrupt flags. All bits are write 1 to clear."]
pub mod intfl;
#[doc = "INTEN (rw) register accessor: Register for enabling interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Register for enabling interrupts."]
pub mod inten;
#[doc = "WKFL (rw) register accessor: Register for wake up flags. All bits in this register are write 1 to clear.\n\nYou can [`read`](crate::Reg::read) this register and get [`wkfl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkfl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkfl`]
module"]
#[doc(alias = "WKFL")]
pub type Wkfl = crate::Reg<wkfl::WkflSpec>;
#[doc = "Register for wake up flags. All bits in this register are write 1 to clear."]
pub mod wkfl;
#[doc = "WKEN (rw) register accessor: Register for wake up enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`wken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wken`]
module"]
#[doc(alias = "WKEN")]
pub type Wken = crate::Reg<wken::WkenSpec>;
#[doc = "Register for wake up enable."]
pub mod wken;
#[doc = "STAT (r) register accessor: SPI Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "SPI Status register."]
pub mod stat;
