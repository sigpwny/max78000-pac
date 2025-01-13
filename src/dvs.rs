#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    stat: Stat,
    direct: Direct,
    mon: Mon,
    adj_up: AdjUp,
    adj_dwn: AdjDwn,
    thres_cmp: ThresCmp,
    tap_sel: [TapSel; 5],
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - Status Fields"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x08 - Direct control of target voltage"]
    #[inline(always)]
    pub const fn direct(&self) -> &Direct {
        &self.direct
    }
    #[doc = "0x0c - Monitor Delay"]
    #[inline(always)]
    pub const fn mon(&self) -> &Mon {
        &self.mon
    }
    #[doc = "0x10 - Up Delay Register"]
    #[inline(always)]
    pub const fn adj_up(&self) -> &AdjUp {
        &self.adj_up
    }
    #[doc = "0x14 - Down Delay Register"]
    #[inline(always)]
    pub const fn adj_dwn(&self) -> &AdjDwn {
        &self.adj_dwn
    }
    #[doc = "0x18 - Up Delay Register"]
    #[inline(always)]
    pub const fn thres_cmp(&self) -> &ThresCmp {
        &self.thres_cmp
    }
    #[doc = "0x1c..0x30 - DVS Tap Select Register"]
    #[inline(always)]
    pub const fn tap_sel(&self, n: usize) -> &TapSel {
        &self.tap_sel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x30 - DVS Tap Select Register"]
    #[inline(always)]
    pub fn tap_sel_iter(&self) -> impl Iterator<Item = &TapSel> {
        self.tap_sel.iter()
    }
}
#[doc = "CTL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control Register"]
pub mod ctl;
#[doc = "STAT (rw) register accessor: Status Fields\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status Fields"]
pub mod stat;
#[doc = "DIRECT (rw) register accessor: Direct control of target voltage\n\nYou can [`read`](crate::Reg::read) this register and get [`direct::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`direct::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@direct`]
module"]
#[doc(alias = "DIRECT")]
pub type Direct = crate::Reg<direct::DirectSpec>;
#[doc = "Direct control of target voltage"]
pub mod direct;
#[doc = "MON (rw) register accessor: Monitor Delay\n\nYou can [`read`](crate::Reg::read) this register and get [`mon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mon`]
module"]
#[doc(alias = "MON")]
pub type Mon = crate::Reg<mon::MonSpec>;
#[doc = "Monitor Delay"]
pub mod mon;
#[doc = "ADJ_UP (rw) register accessor: Up Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adj_up::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adj_up::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adj_up`]
module"]
#[doc(alias = "ADJ_UP")]
pub type AdjUp = crate::Reg<adj_up::AdjUpSpec>;
#[doc = "Up Delay Register"]
pub mod adj_up;
#[doc = "ADJ_DWN (rw) register accessor: Down Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adj_dwn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adj_dwn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adj_dwn`]
module"]
#[doc(alias = "ADJ_DWN")]
pub type AdjDwn = crate::Reg<adj_dwn::AdjDwnSpec>;
#[doc = "Down Delay Register"]
pub mod adj_dwn;
#[doc = "THRES_CMP (rw) register accessor: Up Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`thres_cmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres_cmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thres_cmp`]
module"]
#[doc(alias = "THRES_CMP")]
pub type ThresCmp = crate::Reg<thres_cmp::ThresCmpSpec>;
#[doc = "Up Delay Register"]
pub mod thres_cmp;
#[doc = "TAP_SEL (rw) register accessor: DVS Tap Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tap_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tap_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tap_sel`]
module"]
#[doc(alias = "TAP_SEL")]
pub type TapSel = crate::Reg<tap_sel::TapSelSpec>;
#[doc = "DVS Tap Select Register"]
pub mod tap_sel;
