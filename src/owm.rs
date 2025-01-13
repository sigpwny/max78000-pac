#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg: Cfg,
    clk_div_1us: ClkDiv1us,
    ctrl_stat: CtrlStat,
    data: Data,
    intfl: Intfl,
    inten: Inten,
}
impl RegisterBlock {
    #[doc = "0x00 - 1-Wire Master Configuration."]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x04 - 1-Wire Master Clock Divisor."]
    #[inline(always)]
    pub const fn clk_div_1us(&self) -> &ClkDiv1us {
        &self.clk_div_1us
    }
    #[doc = "0x08 - 1-Wire Master Control/Status."]
    #[inline(always)]
    pub const fn ctrl_stat(&self) -> &CtrlStat {
        &self.ctrl_stat
    }
    #[doc = "0x0c - 1-Wire Master Data Buffer."]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x10 - 1-Wire Master Interrupt Flags."]
    #[inline(always)]
    pub const fn intfl(&self) -> &Intfl {
        &self.intfl
    }
    #[doc = "0x14 - 1-Wire Master Interrupt Enables."]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
}
#[doc = "CFG (rw) register accessor: 1-Wire Master Configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "1-Wire Master Configuration."]
pub mod cfg;
#[doc = "CLK_DIV_1US (rw) register accessor: 1-Wire Master Clock Divisor.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_div_1us::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_div_1us::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_div_1us`]
module"]
#[doc(alias = "CLK_DIV_1US")]
pub type ClkDiv1us = crate::Reg<clk_div_1us::ClkDiv1usSpec>;
#[doc = "1-Wire Master Clock Divisor."]
pub mod clk_div_1us;
#[doc = "CTRL_STAT (rw) register accessor: 1-Wire Master Control/Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_stat`]
module"]
#[doc(alias = "CTRL_STAT")]
pub type CtrlStat = crate::Reg<ctrl_stat::CtrlStatSpec>;
#[doc = "1-Wire Master Control/Status."]
pub mod ctrl_stat;
#[doc = "DATA (rw) register accessor: 1-Wire Master Data Buffer.\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "1-Wire Master Data Buffer."]
pub mod data;
#[doc = "INTFL (rw) register accessor: 1-Wire Master Interrupt Flags.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfl`]
module"]
#[doc(alias = "INTFL")]
pub type Intfl = crate::Reg<intfl::IntflSpec>;
#[doc = "1-Wire Master Interrupt Flags."]
pub mod intfl;
#[doc = "INTEN (rw) register accessor: 1-Wire Master Interrupt Enables.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "1-Wire Master Interrupt Enables."]
pub mod inten;
