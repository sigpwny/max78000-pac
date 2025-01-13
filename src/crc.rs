#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    _reserved_1_datain8: [u8; 0x04],
    poly: Poly,
    val: Val,
}
impl RegisterBlock {
    #[doc = "0x00 - CRC Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - CRC Data Input"]
    #[inline(always)]
    pub const fn datain8(&self) -> &Datain8 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - CRC Data Input"]
    #[inline(always)]
    pub const fn datain16(&self) -> &Datain16 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - CRC Data Input"]
    #[inline(always)]
    pub const fn datain32(&self) -> &Datain32 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - CRC Polynomial"]
    #[inline(always)]
    pub const fn poly(&self) -> &Poly {
        &self.poly
    }
    #[doc = "0x0c - Current CRC Value"]
    #[inline(always)]
    pub const fn val(&self) -> &Val {
        &self.val
    }
}
#[doc = "CTRL (rw) register accessor: CRC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "CRC Control"]
pub mod ctrl;
#[doc = "DATAIN32 (rw) register accessor: CRC Data Input\n\nYou can [`read`](crate::Reg::read) this register and get [`datain32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datain32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datain32`]
module"]
#[doc(alias = "DATAIN32")]
pub type Datain32 = crate::Reg<datain32::Datain32Spec>;
#[doc = "CRC Data Input"]
pub mod datain32;
#[doc = "DATAIN16 (rw) register accessor: CRC Data Input\n\nYou can [`read`](crate::Reg::read) this register and get [`datain16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datain16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datain16`]
module"]
#[doc(alias = "DATAIN16")]
pub type Datain16 = crate::Reg<datain16::Datain16Spec>;
#[doc = "CRC Data Input"]
pub mod datain16;
#[doc = "DATAIN8 (rw) register accessor: CRC Data Input\n\nYou can [`read`](crate::Reg::read) this register and get [`datain8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datain8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datain8`]
module"]
#[doc(alias = "DATAIN8")]
pub type Datain8 = crate::Reg<datain8::Datain8Spec>;
#[doc = "CRC Data Input"]
pub mod datain8;
#[doc = "POLY (rw) register accessor: CRC Polynomial\n\nYou can [`read`](crate::Reg::read) this register and get [`poly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@poly`]
module"]
#[doc(alias = "POLY")]
pub type Poly = crate::Reg<poly::PolySpec>;
#[doc = "CRC Polynomial"]
pub mod poly;
#[doc = "VAL (rw) register accessor: Current CRC Value\n\nYou can [`read`](crate::Reg::read) this register and get [`val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@val`]
module"]
#[doc(alias = "VAL")]
pub type Val = crate::Reg<val::ValSpec>;
#[doc = "Current CRC Value"]
pub mod val;
