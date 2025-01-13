#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    vrego_a: VregoA,
    vrego_b: VregoB,
    vrego_c: VregoC,
    vrego_d: VregoD,
    ipka: Ipka,
    ipkb: Ipkb,
    maxton: Maxton,
    iload_a: IloadA,
    iload_b: IloadB,
    iload_c: IloadC,
    iload_d: IloadD,
    buck_alert_thr_a: BuckAlertThrA,
    buck_alert_thr_b: BuckAlertThrB,
    buck_alert_thr_c: BuckAlertThrC,
    buck_alert_thr_d: BuckAlertThrD,
    buck_out_ready: BuckOutReady,
    zero_cross_cal_a: ZeroCrossCalA,
    zero_cross_cal_b: ZeroCrossCalB,
    zero_cross_cal_c: ZeroCrossCalC,
    zero_cross_cal_d: ZeroCrossCalD,
}
impl RegisterBlock {
    #[doc = "0x04 - Buck Voltage Regulator A Control Register"]
    #[inline(always)]
    pub const fn vrego_a(&self) -> &VregoA {
        &self.vrego_a
    }
    #[doc = "0x08 - Buck Voltage Regulator B Control Register"]
    #[inline(always)]
    pub const fn vrego_b(&self) -> &VregoB {
        &self.vrego_b
    }
    #[doc = "0x0c - Buck Voltage Regulator C Control Register"]
    #[inline(always)]
    pub const fn vrego_c(&self) -> &VregoC {
        &self.vrego_c
    }
    #[doc = "0x10 - Buck Voltage Regulator D Control Register"]
    #[inline(always)]
    pub const fn vrego_d(&self) -> &VregoD {
        &self.vrego_d
    }
    #[doc = "0x14 - High Side FET Peak Current VREGO_A/VREGO_B Register"]
    #[inline(always)]
    pub const fn ipka(&self) -> &Ipka {
        &self.ipka
    }
    #[doc = "0x18 - High Side FET Peak Current VREGO_C/VREGO_D Register"]
    #[inline(always)]
    pub const fn ipkb(&self) -> &Ipkb {
        &self.ipkb
    }
    #[doc = "0x1c - Maximum High Side FET Time On Register"]
    #[inline(always)]
    pub const fn maxton(&self) -> &Maxton {
        &self.maxton
    }
    #[doc = "0x20 - Buck Cycle Count VREGO_A Register"]
    #[inline(always)]
    pub const fn iload_a(&self) -> &IloadA {
        &self.iload_a
    }
    #[doc = "0x24 - Buck Cycle Count VREGO_B Register"]
    #[inline(always)]
    pub const fn iload_b(&self) -> &IloadB {
        &self.iload_b
    }
    #[doc = "0x28 - Buck Cycle Count VREGO_C Register"]
    #[inline(always)]
    pub const fn iload_c(&self) -> &IloadC {
        &self.iload_c
    }
    #[doc = "0x2c - Buck Cycle Count VREGO_D Register"]
    #[inline(always)]
    pub const fn iload_d(&self) -> &IloadD {
        &self.iload_d
    }
    #[doc = "0x30 - Buck Cycle Count Alert VERGO_A Register"]
    #[inline(always)]
    pub const fn buck_alert_thr_a(&self) -> &BuckAlertThrA {
        &self.buck_alert_thr_a
    }
    #[doc = "0x34 - Buck Cycle Count Alert VERGO_B Register"]
    #[inline(always)]
    pub const fn buck_alert_thr_b(&self) -> &BuckAlertThrB {
        &self.buck_alert_thr_b
    }
    #[doc = "0x38 - Buck Cycle Count Alert VERGO_C Register"]
    #[inline(always)]
    pub const fn buck_alert_thr_c(&self) -> &BuckAlertThrC {
        &self.buck_alert_thr_c
    }
    #[doc = "0x3c - Buck Cycle Count Alert VERGO_D Register"]
    #[inline(always)]
    pub const fn buck_alert_thr_d(&self) -> &BuckAlertThrD {
        &self.buck_alert_thr_d
    }
    #[doc = "0x40 - Buck Regulator Output Ready Register"]
    #[inline(always)]
    pub const fn buck_out_ready(&self) -> &BuckOutReady {
        &self.buck_out_ready
    }
    #[doc = "0x44 - Zero Cross Calibration VERGO_A Register"]
    #[inline(always)]
    pub const fn zero_cross_cal_a(&self) -> &ZeroCrossCalA {
        &self.zero_cross_cal_a
    }
    #[doc = "0x48 - Zero Cross Calibration VERGO_B Register"]
    #[inline(always)]
    pub const fn zero_cross_cal_b(&self) -> &ZeroCrossCalB {
        &self.zero_cross_cal_b
    }
    #[doc = "0x4c - Zero Cross Calibration VERGO_C Register"]
    #[inline(always)]
    pub const fn zero_cross_cal_c(&self) -> &ZeroCrossCalC {
        &self.zero_cross_cal_c
    }
    #[doc = "0x50 - Zero Cross Calibration VERGO_D Register"]
    #[inline(always)]
    pub const fn zero_cross_cal_d(&self) -> &ZeroCrossCalD {
        &self.zero_cross_cal_d
    }
}
#[doc = "VREGO_A (rw) register accessor: Buck Voltage Regulator A Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vrego_a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrego_a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrego_a`]
module"]
#[doc(alias = "VREGO_A")]
pub type VregoA = crate::Reg<vrego_a::VregoASpec>;
#[doc = "Buck Voltage Regulator A Control Register"]
pub mod vrego_a;
#[doc = "VREGO_B (rw) register accessor: Buck Voltage Regulator B Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vrego_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrego_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrego_b`]
module"]
#[doc(alias = "VREGO_B")]
pub type VregoB = crate::Reg<vrego_b::VregoBSpec>;
#[doc = "Buck Voltage Regulator B Control Register"]
pub mod vrego_b;
#[doc = "VREGO_C (rw) register accessor: Buck Voltage Regulator C Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vrego_c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrego_c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrego_c`]
module"]
#[doc(alias = "VREGO_C")]
pub type VregoC = crate::Reg<vrego_c::VregoCSpec>;
#[doc = "Buck Voltage Regulator C Control Register"]
pub mod vrego_c;
#[doc = "VREGO_D (rw) register accessor: Buck Voltage Regulator D Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vrego_d::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrego_d::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vrego_d`]
module"]
#[doc(alias = "VREGO_D")]
pub type VregoD = crate::Reg<vrego_d::VregoDSpec>;
#[doc = "Buck Voltage Regulator D Control Register"]
pub mod vrego_d;
#[doc = "IPKA (rw) register accessor: High Side FET Peak Current VREGO_A/VREGO_B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipka::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipka::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipka`]
module"]
#[doc(alias = "IPKA")]
pub type Ipka = crate::Reg<ipka::IpkaSpec>;
#[doc = "High Side FET Peak Current VREGO_A/VREGO_B Register"]
pub mod ipka;
#[doc = "IPKB (rw) register accessor: High Side FET Peak Current VREGO_C/VREGO_D Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipkb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipkb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipkb`]
module"]
#[doc(alias = "IPKB")]
pub type Ipkb = crate::Reg<ipkb::IpkbSpec>;
#[doc = "High Side FET Peak Current VREGO_C/VREGO_D Register"]
pub mod ipkb;
#[doc = "MAXTON (rw) register accessor: Maximum High Side FET Time On Register\n\nYou can [`read`](crate::Reg::read) this register and get [`maxton::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxton::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxton`]
module"]
#[doc(alias = "MAXTON")]
pub type Maxton = crate::Reg<maxton::MaxtonSpec>;
#[doc = "Maximum High Side FET Time On Register"]
pub mod maxton;
#[doc = "ILOAD_A (r) register accessor: Buck Cycle Count VREGO_A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iload_a::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iload_a`]
module"]
#[doc(alias = "ILOAD_A")]
pub type IloadA = crate::Reg<iload_a::IloadASpec>;
#[doc = "Buck Cycle Count VREGO_A Register"]
pub mod iload_a;
#[doc = "ILOAD_B (r) register accessor: Buck Cycle Count VREGO_B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iload_b::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iload_b`]
module"]
#[doc(alias = "ILOAD_B")]
pub type IloadB = crate::Reg<iload_b::IloadBSpec>;
#[doc = "Buck Cycle Count VREGO_B Register"]
pub mod iload_b;
#[doc = "ILOAD_C (r) register accessor: Buck Cycle Count VREGO_C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iload_c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iload_c`]
module"]
#[doc(alias = "ILOAD_C")]
pub type IloadC = crate::Reg<iload_c::IloadCSpec>;
#[doc = "Buck Cycle Count VREGO_C Register"]
pub mod iload_c;
#[doc = "ILOAD_D (r) register accessor: Buck Cycle Count VREGO_D Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iload_d::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iload_d`]
module"]
#[doc(alias = "ILOAD_D")]
pub type IloadD = crate::Reg<iload_d::IloadDSpec>;
#[doc = "Buck Cycle Count VREGO_D Register"]
pub mod iload_d;
#[doc = "BUCK_ALERT_THR_A (rw) register accessor: Buck Cycle Count Alert VERGO_A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`buck_alert_thr_a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buck_alert_thr_a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buck_alert_thr_a`]
module"]
#[doc(alias = "BUCK_ALERT_THR_A")]
pub type BuckAlertThrA = crate::Reg<buck_alert_thr_a::BuckAlertThrASpec>;
#[doc = "Buck Cycle Count Alert VERGO_A Register"]
pub mod buck_alert_thr_a;
#[doc = "BUCK_ALERT_THR_B (rw) register accessor: Buck Cycle Count Alert VERGO_B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`buck_alert_thr_b::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buck_alert_thr_b::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buck_alert_thr_b`]
module"]
#[doc(alias = "BUCK_ALERT_THR_B")]
pub type BuckAlertThrB = crate::Reg<buck_alert_thr_b::BuckAlertThrBSpec>;
#[doc = "Buck Cycle Count Alert VERGO_B Register"]
pub mod buck_alert_thr_b;
#[doc = "BUCK_ALERT_THR_C (rw) register accessor: Buck Cycle Count Alert VERGO_C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`buck_alert_thr_c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buck_alert_thr_c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buck_alert_thr_c`]
module"]
#[doc(alias = "BUCK_ALERT_THR_C")]
pub type BuckAlertThrC = crate::Reg<buck_alert_thr_c::BuckAlertThrCSpec>;
#[doc = "Buck Cycle Count Alert VERGO_C Register"]
pub mod buck_alert_thr_c;
#[doc = "BUCK_ALERT_THR_D (rw) register accessor: Buck Cycle Count Alert VERGO_D Register\n\nYou can [`read`](crate::Reg::read) this register and get [`buck_alert_thr_d::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buck_alert_thr_d::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buck_alert_thr_d`]
module"]
#[doc(alias = "BUCK_ALERT_THR_D")]
pub type BuckAlertThrD = crate::Reg<buck_alert_thr_d::BuckAlertThrDSpec>;
#[doc = "Buck Cycle Count Alert VERGO_D Register"]
pub mod buck_alert_thr_d;
#[doc = "BUCK_OUT_READY (r) register accessor: Buck Regulator Output Ready Register\n\nYou can [`read`](crate::Reg::read) this register and get [`buck_out_ready::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buck_out_ready`]
module"]
#[doc(alias = "BUCK_OUT_READY")]
pub type BuckOutReady = crate::Reg<buck_out_ready::BuckOutReadySpec>;
#[doc = "Buck Regulator Output Ready Register"]
pub mod buck_out_ready;
#[doc = "ZERO_CROSS_CAL_A (r) register accessor: Zero Cross Calibration VERGO_A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`zero_cross_cal_a::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zero_cross_cal_a`]
module"]
#[doc(alias = "ZERO_CROSS_CAL_A")]
pub type ZeroCrossCalA = crate::Reg<zero_cross_cal_a::ZeroCrossCalASpec>;
#[doc = "Zero Cross Calibration VERGO_A Register"]
pub mod zero_cross_cal_a;
#[doc = "ZERO_CROSS_CAL_B (r) register accessor: Zero Cross Calibration VERGO_B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`zero_cross_cal_b::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zero_cross_cal_b`]
module"]
#[doc(alias = "ZERO_CROSS_CAL_B")]
pub type ZeroCrossCalB = crate::Reg<zero_cross_cal_b::ZeroCrossCalBSpec>;
#[doc = "Zero Cross Calibration VERGO_B Register"]
pub mod zero_cross_cal_b;
#[doc = "ZERO_CROSS_CAL_C (r) register accessor: Zero Cross Calibration VERGO_C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`zero_cross_cal_c::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zero_cross_cal_c`]
module"]
#[doc(alias = "ZERO_CROSS_CAL_C")]
pub type ZeroCrossCalC = crate::Reg<zero_cross_cal_c::ZeroCrossCalCSpec>;
#[doc = "Zero Cross Calibration VERGO_C Register"]
pub mod zero_cross_cal_c;
#[doc = "ZERO_CROSS_CAL_D (r) register accessor: Zero Cross Calibration VERGO_D Register\n\nYou can [`read`](crate::Reg::read) this register and get [`zero_cross_cal_d::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zero_cross_cal_d`]
module"]
#[doc(alias = "ZERO_CROSS_CAL_D")]
pub type ZeroCrossCalD = crate::Reg<zero_cross_cal_d::ZeroCrossCalDSpec>;
#[doc = "Zero Cross Calibration VERGO_D Register"]
pub mod zero_cross_cal_d;
