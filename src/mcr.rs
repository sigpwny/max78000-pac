#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    eccen: Eccen,
    ipo_mtrim: IpoMtrim,
    outen: Outen,
    cmp_ctrl: CmpCtrl,
    ctrl: Ctrl,
    _reserved5: [u8; 0x0c],
    gpio3_ctrl: Gpio3Ctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - ECC Enable Register"]
    #[inline(always)]
    pub const fn eccen(&self) -> &Eccen {
        &self.eccen
    }
    #[doc = "0x04 - IPO Manual Register"]
    #[inline(always)]
    pub const fn ipo_mtrim(&self) -> &IpoMtrim {
        &self.ipo_mtrim
    }
    #[doc = "0x08 - Output Enable Register"]
    #[inline(always)]
    pub const fn outen(&self) -> &Outen {
        &self.outen
    }
    #[doc = "0x0c - Comparator Control Register."]
    #[inline(always)]
    pub const fn cmp_ctrl(&self) -> &CmpCtrl {
        &self.cmp_ctrl
    }
    #[doc = "0x10 - Miscellaneous Control Register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x20 - GPIO3 Pin Control Register."]
    #[inline(always)]
    pub const fn gpio3_ctrl(&self) -> &Gpio3Ctrl {
        &self.gpio3_ctrl
    }
}
#[doc = "ECCEN (rw) register accessor: ECC Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccen`]
module"]
#[doc(alias = "ECCEN")]
pub type Eccen = crate::Reg<eccen::EccenSpec>;
#[doc = "ECC Enable Register"]
pub mod eccen;
#[doc = "IPO_MTRIM (rw) register accessor: IPO Manual Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipo_mtrim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipo_mtrim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipo_mtrim`]
module"]
#[doc(alias = "IPO_MTRIM")]
pub type IpoMtrim = crate::Reg<ipo_mtrim::IpoMtrimSpec>;
#[doc = "IPO Manual Register"]
pub mod ipo_mtrim;
#[doc = "OUTEN (rw) register accessor: Output Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`outen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outen`]
module"]
#[doc(alias = "OUTEN")]
pub type Outen = crate::Reg<outen::OutenSpec>;
#[doc = "Output Enable Register"]
pub mod outen;
#[doc = "CMP_CTRL (rw) register accessor: Comparator Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_ctrl`]
module"]
#[doc(alias = "CMP_CTRL")]
pub type CmpCtrl = crate::Reg<cmp_ctrl::CmpCtrlSpec>;
#[doc = "Comparator Control Register."]
pub mod cmp_ctrl;
#[doc = "CTRL (rw) register accessor: Miscellaneous Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Miscellaneous Control Register."]
pub mod ctrl;
#[doc = "GPIO3_CTRL (rw) register accessor: GPIO3 Pin Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio3_ctrl`]
module"]
#[doc(alias = "GPIO3_CTRL")]
pub type Gpio3Ctrl = crate::Reg<gpio3_ctrl::Gpio3CtrlSpec>;
#[doc = "GPIO3 Pin Control Register."]
pub mod gpio3_ctrl;
