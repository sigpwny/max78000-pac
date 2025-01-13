#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    status: Status,
    intfl0: Intfl0,
    inten0: Inten0,
    intfl1: Intfl1,
    inten1: Inten1,
    fifolen: Fifolen,
    rxctrl0: Rxctrl0,
    rxctrl1: Rxctrl1,
    txctrl0: Txctrl0,
    txctrl1: Txctrl1,
    fifo: Fifo,
    mstctrl: Mstctrl,
    clklo: Clklo,
    clkhi: Clkhi,
    hsclk: Hsclk,
    timeout: Timeout,
    _reserved17: [u8; 0x04],
    dma: Dma,
    _reserved_18_slave0: [u8; 0x10],
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register0."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Status Register."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - Interrupt Status Register."]
    #[inline(always)]
    pub const fn intfl0(&self) -> &Intfl0 {
        &self.intfl0
    }
    #[doc = "0x0c - Interrupt Enable Register."]
    #[inline(always)]
    pub const fn inten0(&self) -> &Inten0 {
        &self.inten0
    }
    #[doc = "0x10 - Interrupt Status Register 1."]
    #[inline(always)]
    pub const fn intfl1(&self) -> &Intfl1 {
        &self.intfl1
    }
    #[doc = "0x14 - Interrupt Staus Register 1."]
    #[inline(always)]
    pub const fn inten1(&self) -> &Inten1 {
        &self.inten1
    }
    #[doc = "0x18 - FIFO Configuration Register."]
    #[inline(always)]
    pub const fn fifolen(&self) -> &Fifolen {
        &self.fifolen
    }
    #[doc = "0x1c - Receive Control Register 0."]
    #[inline(always)]
    pub const fn rxctrl0(&self) -> &Rxctrl0 {
        &self.rxctrl0
    }
    #[doc = "0x20 - Receive Control Register 1."]
    #[inline(always)]
    pub const fn rxctrl1(&self) -> &Rxctrl1 {
        &self.rxctrl1
    }
    #[doc = "0x24 - Transmit Control Register 0."]
    #[inline(always)]
    pub const fn txctrl0(&self) -> &Txctrl0 {
        &self.txctrl0
    }
    #[doc = "0x28 - Transmit Control Register 1."]
    #[inline(always)]
    pub const fn txctrl1(&self) -> &Txctrl1 {
        &self.txctrl1
    }
    #[doc = "0x2c - Data Register."]
    #[inline(always)]
    pub const fn fifo(&self) -> &Fifo {
        &self.fifo
    }
    #[doc = "0x30 - Master Control Register."]
    #[inline(always)]
    pub const fn mstctrl(&self) -> &Mstctrl {
        &self.mstctrl
    }
    #[doc = "0x34 - Clock Low Register."]
    #[inline(always)]
    pub const fn clklo(&self) -> &Clklo {
        &self.clklo
    }
    #[doc = "0x38 - Clock high Register."]
    #[inline(always)]
    pub const fn clkhi(&self) -> &Clkhi {
        &self.clkhi
    }
    #[doc = "0x3c - Clock high Register."]
    #[inline(always)]
    pub const fn hsclk(&self) -> &Hsclk {
        &self.hsclk
    }
    #[doc = "0x40 - Timeout Register"]
    #[inline(always)]
    pub const fn timeout(&self) -> &Timeout {
        &self.timeout
    }
    #[doc = "0x48 - DMA Register."]
    #[inline(always)]
    pub const fn dma(&self) -> &Dma {
        &self.dma
    }
    #[doc = "0x4c - Slave Address Register."]
    #[inline(always)]
    pub const fn slave0(&self) -> &Slave0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x4c..0x5c - Slave Address Register."]
    #[inline(always)]
    pub const fn slave_multi(&self, n: usize) -> &SlaveMulti {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(76)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4c..0x5c - Slave Address Register."]
    #[inline(always)]
    pub fn slave_multi_iter(&self) -> impl Iterator<Item = &SlaveMulti> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(76)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x50 - Slave Address Register."]
    #[inline(always)]
    pub const fn slave1(&self) -> &Slave1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x54 - Slave Address Register."]
    #[inline(always)]
    pub const fn slave2(&self) -> &Slave2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x58 - Slave Address Register."]
    #[inline(always)]
    pub const fn slave3(&self) -> &Slave3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(88).cast() }
    }
}
#[doc = "CTRL (rw) register accessor: Control Register0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register0."]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register."]
pub mod status;
#[doc = "INTFL0 (rw) register accessor: Interrupt Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfl0`]
module"]
#[doc(alias = "INTFL0")]
pub type Intfl0 = crate::Reg<intfl0::Intfl0Spec>;
#[doc = "Interrupt Status Register."]
pub mod intfl0;
#[doc = "INTEN0 (rw) register accessor: Interrupt Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten0`]
module"]
#[doc(alias = "INTEN0")]
pub type Inten0 = crate::Reg<inten0::Inten0Spec>;
#[doc = "Interrupt Enable Register."]
pub mod inten0;
#[doc = "INTFL1 (rw) register accessor: Interrupt Status Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfl1`]
module"]
#[doc(alias = "INTFL1")]
pub type Intfl1 = crate::Reg<intfl1::Intfl1Spec>;
#[doc = "Interrupt Status Register 1."]
pub mod intfl1;
#[doc = "INTEN1 (rw) register accessor: Interrupt Staus Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten1`]
module"]
#[doc(alias = "INTEN1")]
pub type Inten1 = crate::Reg<inten1::Inten1Spec>;
#[doc = "Interrupt Staus Register 1."]
pub mod inten1;
#[doc = "FIFOLEN (rw) register accessor: FIFO Configuration Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifolen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifolen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifolen`]
module"]
#[doc(alias = "FIFOLEN")]
pub type Fifolen = crate::Reg<fifolen::FifolenSpec>;
#[doc = "FIFO Configuration Register."]
pub mod fifolen;
#[doc = "RXCTRL0 (rw) register accessor: Receive Control Register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxctrl0`]
module"]
#[doc(alias = "RXCTRL0")]
pub type Rxctrl0 = crate::Reg<rxctrl0::Rxctrl0Spec>;
#[doc = "Receive Control Register 0."]
pub mod rxctrl0;
#[doc = "RXCTRL1 (rw) register accessor: Receive Control Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxctrl1`]
module"]
#[doc(alias = "RXCTRL1")]
pub type Rxctrl1 = crate::Reg<rxctrl1::Rxctrl1Spec>;
#[doc = "Receive Control Register 1."]
pub mod rxctrl1;
#[doc = "TXCTRL0 (rw) register accessor: Transmit Control Register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`txctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txctrl0`]
module"]
#[doc(alias = "TXCTRL0")]
pub type Txctrl0 = crate::Reg<txctrl0::Txctrl0Spec>;
#[doc = "Transmit Control Register 0."]
pub mod txctrl0;
#[doc = "TXCTRL1 (rw) register accessor: Transmit Control Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`txctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txctrl1`]
module"]
#[doc(alias = "TXCTRL1")]
pub type Txctrl1 = crate::Reg<txctrl1::Txctrl1Spec>;
#[doc = "Transmit Control Register 1."]
pub mod txctrl1;
#[doc = "FIFO (rw) register accessor: Data Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`]
module"]
#[doc(alias = "FIFO")]
pub type Fifo = crate::Reg<fifo::FifoSpec>;
#[doc = "Data Register."]
pub mod fifo;
#[doc = "MSTCTRL (rw) register accessor: Master Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mstctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstctrl`]
module"]
#[doc(alias = "MSTCTRL")]
pub type Mstctrl = crate::Reg<mstctrl::MstctrlSpec>;
#[doc = "Master Control Register."]
pub mod mstctrl;
#[doc = "CLKLO (rw) register accessor: Clock Low Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clklo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clklo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clklo`]
module"]
#[doc(alias = "CLKLO")]
pub type Clklo = crate::Reg<clklo::ClkloSpec>;
#[doc = "Clock Low Register."]
pub mod clklo;
#[doc = "CLKHI (rw) register accessor: Clock high Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkhi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkhi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkhi`]
module"]
#[doc(alias = "CLKHI")]
pub type Clkhi = crate::Reg<clkhi::ClkhiSpec>;
#[doc = "Clock high Register."]
pub mod clkhi;
#[doc = "HSCLK (rw) register accessor: Clock high Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hsclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsclk`]
module"]
#[doc(alias = "HSCLK")]
pub type Hsclk = crate::Reg<hsclk::HsclkSpec>;
#[doc = "Clock high Register."]
pub mod hsclk;
#[doc = "TIMEOUT (rw) register accessor: Timeout Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout`]
module"]
#[doc(alias = "TIMEOUT")]
pub type Timeout = crate::Reg<timeout::TimeoutSpec>;
#[doc = "Timeout Register"]
pub mod timeout;
#[doc = "DMA (rw) register accessor: DMA Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`]
module"]
#[doc(alias = "DMA")]
pub type Dma = crate::Reg<dma::DmaSpec>;
#[doc = "DMA Register."]
pub mod dma;
#[doc = "SLAVE_MULTI (rw) register accessor: Slave Address Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slave_multi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave_multi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave_multi`]
module"]
#[doc(alias = "SLAVE_MULTI")]
pub type SlaveMulti = crate::Reg<slave_multi::SlaveMultiSpec>;
#[doc = "Slave Address Register."]
pub mod slave_multi;
#[doc = "SLAVE0 (rw) register accessor: Slave Address Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slave0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave0`]
module"]
#[doc(alias = "SLAVE0")]
pub type Slave0 = crate::Reg<slave0::Slave0Spec>;
#[doc = "Slave Address Register."]
pub mod slave0;
#[doc = "SLAVE1 (rw) register accessor: Slave Address Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slave1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave1`]
module"]
#[doc(alias = "SLAVE1")]
pub type Slave1 = crate::Reg<slave1::Slave1Spec>;
#[doc = "Slave Address Register."]
pub mod slave1;
#[doc = "SLAVE2 (rw) register accessor: Slave Address Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slave2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave2`]
module"]
#[doc(alias = "SLAVE2")]
pub type Slave2 = crate::Reg<slave2::Slave2Spec>;
#[doc = "Slave Address Register."]
pub mod slave2;
#[doc = "SLAVE3 (rw) register accessor: Slave Address Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slave3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave3`]
module"]
#[doc(alias = "SLAVE3")]
pub type Slave3 = crate::Reg<slave3::Slave3Spec>;
#[doc = "Slave Address Register."]
pub mod slave3;
