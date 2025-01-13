#[repr(C)]
#[doc = "DMA Channel registers."]
#[doc(alias = "CH")]
pub struct Ch {
    ctrl: Ctrl,
    status: Status,
    src: Src,
    dst: Dst,
    cnt: Cnt,
    srcrld: Srcrld,
    dstrld: Dstrld,
    cntrld: Cntrld,
}
impl Ch {
    #[doc = "0x00 - DMA Channel Control Register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - DMA Channel Status Register."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - Source Device Address. If SRCINC=1, the counter bits are incremented by 1,2, or 4, depending on the data width of each AHB cycle. For peripheral transfers, some or all of the actual address bits are fixed. If SRCINC=0, this register remains constant. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with the contents of DMA_SRC_RLD."]
    #[inline(always)]
    pub const fn src(&self) -> &Src {
        &self.src
    }
    #[doc = "0x0c - Destination Device Address. For peripheral transfers, some or all of the actual address bits are fixed. If DSTINC=1, this register is incremented on every AHB write out of the DMA FIFO. They are incremented by 1, 2, or 4, depending on the data width of each AHB cycle. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with DMA_DST_RLD."]
    #[inline(always)]
    pub const fn dst(&self) -> &Dst {
        &self.dst
    }
    #[doc = "0x10 - DMA Counter. The user loads this register with the number of bytes to transfer. This counter decreases on every AHB cycle into the DMA FIFO. The decrement will be 1, 2, or 4 depending on the data width of each AHB cycle. When the counter reaches 0, a count-to-zero condition is triggered."]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x14 - Source Address Reload Value. The value of this register is loaded into DMA0_SRC upon a count-to-zero condition."]
    #[inline(always)]
    pub const fn srcrld(&self) -> &Srcrld {
        &self.srcrld
    }
    #[doc = "0x18 - Destination Address Reload Value. The value of this register is loaded into DMA0_DST upon a count-to-zero condition."]
    #[inline(always)]
    pub const fn dstrld(&self) -> &Dstrld {
        &self.dstrld
    }
    #[doc = "0x1c - DMA Channel Count Reload Register."]
    #[inline(always)]
    pub const fn cntrld(&self) -> &Cntrld {
        &self.cntrld
    }
}
#[doc = "CTRL (rw) register accessor: DMA Channel Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "DMA Channel Control Register."]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: DMA Channel Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "DMA Channel Status Register."]
pub mod status;
#[doc = "SRC (rw) register accessor: Source Device Address. If SRCINC=1, the counter bits are incremented by 1,2, or 4, depending on the data width of each AHB cycle. For peripheral transfers, some or all of the actual address bits are fixed. If SRCINC=0, this register remains constant. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with the contents of DMA_SRC_RLD.\n\nYou can [`read`](crate::Reg::read) this register and get [`src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src`]
module"]
#[doc(alias = "SRC")]
pub type Src = crate::Reg<src::SrcSpec>;
#[doc = "Source Device Address. If SRCINC=1, the counter bits are incremented by 1,2, or 4, depending on the data width of each AHB cycle. For peripheral transfers, some or all of the actual address bits are fixed. If SRCINC=0, this register remains constant. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with the contents of DMA_SRC_RLD."]
pub mod src;
#[doc = "DST (rw) register accessor: Destination Device Address. For peripheral transfers, some or all of the actual address bits are fixed. If DSTINC=1, this register is incremented on every AHB write out of the DMA FIFO. They are incremented by 1, 2, or 4, depending on the data width of each AHB cycle. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with DMA_DST_RLD.\n\nYou can [`read`](crate::Reg::read) this register and get [`dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst`]
module"]
#[doc(alias = "DST")]
pub type Dst = crate::Reg<dst::DstSpec>;
#[doc = "Destination Device Address. For peripheral transfers, some or all of the actual address bits are fixed. If DSTINC=1, this register is incremented on every AHB write out of the DMA FIFO. They are incremented by 1, 2, or 4, depending on the data width of each AHB cycle. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with DMA_DST_RLD."]
pub mod dst;
#[doc = "CNT (rw) register accessor: DMA Counter. The user loads this register with the number of bytes to transfer. This counter decreases on every AHB cycle into the DMA FIFO. The decrement will be 1, 2, or 4 depending on the data width of each AHB cycle. When the counter reaches 0, a count-to-zero condition is triggered.\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "DMA Counter. The user loads this register with the number of bytes to transfer. This counter decreases on every AHB cycle into the DMA FIFO. The decrement will be 1, 2, or 4 depending on the data width of each AHB cycle. When the counter reaches 0, a count-to-zero condition is triggered."]
pub mod cnt;
#[doc = "SRCRLD (rw) register accessor: Source Address Reload Value. The value of this register is loaded into DMA0_SRC upon a count-to-zero condition.\n\nYou can [`read`](crate::Reg::read) this register and get [`srcrld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcrld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcrld`]
module"]
#[doc(alias = "SRCRLD")]
pub type Srcrld = crate::Reg<srcrld::SrcrldSpec>;
#[doc = "Source Address Reload Value. The value of this register is loaded into DMA0_SRC upon a count-to-zero condition."]
pub mod srcrld;
#[doc = "DSTRLD (rw) register accessor: Destination Address Reload Value. The value of this register is loaded into DMA0_DST upon a count-to-zero condition.\n\nYou can [`read`](crate::Reg::read) this register and get [`dstrld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstrld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstrld`]
module"]
#[doc(alias = "DSTRLD")]
pub type Dstrld = crate::Reg<dstrld::DstrldSpec>;
#[doc = "Destination Address Reload Value. The value of this register is loaded into DMA0_DST upon a count-to-zero condition."]
pub mod dstrld;
#[doc = "CNTRLD (rw) register accessor: DMA Channel Count Reload Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cntrld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntrld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntrld`]
module"]
#[doc(alias = "CNTRLD")]
pub type Cntrld = crate::Reg<cntrld::CntrldSpec>;
#[doc = "DMA Channel Count Reload Register."]
pub mod cntrld;
