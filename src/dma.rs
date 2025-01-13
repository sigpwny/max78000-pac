#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    inten: Inten,
    intfl: Intfl,
    _reserved2: [u8; 0xf8],
    ch: [Ch; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Control Register."]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x04 - DMA Interrupt Register."]
    #[inline(always)]
    pub const fn intfl(&self) -> &Intfl {
        &self.intfl
    }
    #[doc = "0x100..0x180 - DMA Channel registers."]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &Ch {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x180 - DMA Channel registers."]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &Ch> {
        self.ch.iter()
    }
}
#[doc = "INTEN (rw) register accessor: DMA Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "DMA Control Register."]
pub mod inten;
#[doc = "INTFL (r) register accessor: DMA Interrupt Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfl`]
module"]
#[doc(alias = "INTFL")]
pub type Intfl = crate::Reg<intfl::IntflSpec>;
#[doc = "DMA Interrupt Register."]
pub mod intfl;
#[doc = "DMA Channel registers."]
pub use self::ch::Ch;
#[doc = r"Cluster"]
#[doc = "DMA Channel registers."]
pub mod ch;
