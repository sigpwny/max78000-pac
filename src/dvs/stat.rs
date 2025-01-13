#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `DVS_STATE` reader - State machine state"]
pub type DvsStateR = crate::FieldReader;
#[doc = "Field `DVS_STATE` writer - State machine state"]
pub type DvsStateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADJ_UP_ENA` reader - DVS Raising voltage"]
pub type AdjUpEnaR = crate::BitReader;
#[doc = "Field `ADJ_UP_ENA` writer - DVS Raising voltage"]
pub type AdjUpEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ_DWN_ENA` reader - DVS Lowering voltage"]
pub type AdjDwnEnaR = crate::BitReader;
#[doc = "Field `ADJ_DWN_ENA` writer - DVS Lowering voltage"]
pub type AdjDwnEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ_ACTIVE` reader - Adjustment to a Direct Voltage"]
pub type AdjActiveR = crate::BitReader;
#[doc = "Field `ADJ_ACTIVE` writer - Adjustment to a Direct Voltage"]
pub type AdjActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR_TAP_OK` reader - Tap Enabled and the Tap is withing Hi/Low limits"]
pub type CtrTapOkR = crate::BitReader;
#[doc = "Field `CTR_TAP_OK` writer - Tap Enabled and the Tap is withing Hi/Low limits"]
pub type CtrTapOkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR_TAP_SEL` reader - Status of selected center tap delay line detect output"]
pub type CtrTapSelR = crate::BitReader;
#[doc = "Field `CTR_TAP_SEL` writer - Status of selected center tap delay line detect output"]
pub type CtrTapSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOW_TRIP_DET` reader - Provides the current combined status of all selected Low Range delay lines"]
pub type SlowTripDetR = crate::BitReader;
#[doc = "Field `SLOW_TRIP_DET` writer - Provides the current combined status of all selected Low Range delay lines"]
pub type SlowTripDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAST_TRIP_DET` reader - Provides the current combined status of all selected High Range delay lines"]
pub type FastTripDetR = crate::BitReader;
#[doc = "Field `FAST_TRIP_DET` writer - Provides the current combined status of all selected High Range delay lines"]
pub type FastTripDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS_IN_RANGE` reader - Indicates if the power supply is in range"]
pub type PsInRangeR = crate::BitReader;
#[doc = "Field `PS_IN_RANGE` writer - Indicates if the power supply is in range"]
pub type PsInRangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS_VCNTR` reader - Voltage Count value sent to the power supply"]
pub type PsVcntrR = crate::FieldReader;
#[doc = "Field `PS_VCNTR` writer - Voltage Count value sent to the power supply"]
pub type PsVcntrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MON_DLY_OK` reader - Indicates the monitor delay count is at 0"]
pub type MonDlyOkR = crate::BitReader;
#[doc = "Field `MON_DLY_OK` writer - Indicates the monitor delay count is at 0"]
pub type MonDlyOkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ_DLY_OK` reader - Indicates the adjustment delay count is at 0"]
pub type AdjDlyOkR = crate::BitReader;
#[doc = "Field `ADJ_DLY_OK` writer - Indicates the adjustment delay count is at 0"]
pub type AdjDlyOkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LO_LIMIT_DET` reader - Power supply voltage counter is at low limit"]
pub type LoLimitDetR = crate::BitReader;
#[doc = "Field `LO_LIMIT_DET` writer - Power supply voltage counter is at low limit"]
pub type LoLimitDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HI_LIMIT_DET` reader - Power supply voltage counter is at high limit"]
pub type HiLimitDetR = crate::BitReader;
#[doc = "Field `HI_LIMIT_DET` writer - Power supply voltage counter is at high limit"]
pub type HiLimitDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VALID_TAP` reader - At least one delay line has been enabled"]
pub type ValidTapR = crate::BitReader;
#[doc = "Field `VALID_TAP` writer - At least one delay line has been enabled"]
pub type ValidTapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LIMIT_ERR` reader - Interrupt flag that indicates a voltage count is at/beyond manufacturer limits"]
pub type LimitErrR = crate::BitReader;
#[doc = "Field `LIMIT_ERR` writer - Interrupt flag that indicates a voltage count is at/beyond manufacturer limits"]
pub type LimitErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RANGE_ERR` reader - Interrupt flag that indicates a tap has an invalid value"]
pub type RangeErrR = crate::BitReader;
#[doc = "Field `RANGE_ERR` writer - Interrupt flag that indicates a tap has an invalid value"]
pub type RangeErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ_ERR` reader - Interrupt flag that indicates up and down adjustment requested simultaneously"]
pub type AdjErrR = crate::BitReader;
#[doc = "Field `ADJ_ERR` writer - Interrupt flag that indicates up and down adjustment requested simultaneously"]
pub type AdjErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_SEL_ERR` reader - Indicates the ref select register bit is out of range"]
pub type RefSelErrR = crate::BitReader;
#[doc = "Field `REF_SEL_ERR` writer - Indicates the ref select register bit is out of range"]
pub type RefSelErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FB_TO_ERR` reader - Interrupt flag that indicates a timeout while adjusting the voltage"]
pub type FbToErrR = crate::BitReader;
#[doc = "Field `FB_TO_ERR` writer - Interrupt flag that indicates a timeout while adjusting the voltage"]
pub type FbToErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FB_TO_ERR_S` reader - Interrupt flag that mirror FB_TO_ERR and is write one clear"]
pub type FbToErrSR = crate::BitReader;
#[doc = "Field `FB_TO_ERR_S` writer - Interrupt flag that mirror FB_TO_ERR and is write one clear"]
pub type FbToErrSW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FC_LV_DET_INT` reader - Interrupt flag that indicates the power supply voltage requested is below the low threshold"]
pub type FcLvDetIntR = crate::BitReader;
#[doc = "Field `FC_LV_DET_INT` writer - Interrupt flag that indicates the power supply voltage requested is below the low threshold"]
pub type FcLvDetIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FC_LV_DET_S` reader - Interrupt flag that mirrors FC_LV_DET_INT"]
pub type FcLvDetSR = crate::BitReader;
#[doc = "Field `FC_LV_DET_S` writer - Interrupt flag that mirrors FC_LV_DET_INT"]
pub type FcLvDetSW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - State machine state"]
    #[inline(always)]
    pub fn dvs_state(&self) -> DvsStateR {
        DvsStateR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - DVS Raising voltage"]
    #[inline(always)]
    pub fn adj_up_ena(&self) -> AdjUpEnaR {
        AdjUpEnaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DVS Lowering voltage"]
    #[inline(always)]
    pub fn adj_dwn_ena(&self) -> AdjDwnEnaR {
        AdjDwnEnaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Adjustment to a Direct Voltage"]
    #[inline(always)]
    pub fn adj_active(&self) -> AdjActiveR {
        AdjActiveR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tap Enabled and the Tap is withing Hi/Low limits"]
    #[inline(always)]
    pub fn ctr_tap_ok(&self) -> CtrTapOkR {
        CtrTapOkR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Status of selected center tap delay line detect output"]
    #[inline(always)]
    pub fn ctr_tap_sel(&self) -> CtrTapSelR {
        CtrTapSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Provides the current combined status of all selected Low Range delay lines"]
    #[inline(always)]
    pub fn slow_trip_det(&self) -> SlowTripDetR {
        SlowTripDetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Provides the current combined status of all selected High Range delay lines"]
    #[inline(always)]
    pub fn fast_trip_det(&self) -> FastTripDetR {
        FastTripDetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates if the power supply is in range"]
    #[inline(always)]
    pub fn ps_in_range(&self) -> PsInRangeR {
        PsInRangeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:18 - Voltage Count value sent to the power supply"]
    #[inline(always)]
    pub fn ps_vcntr(&self) -> PsVcntrR {
        PsVcntrR::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bit 19 - Indicates the monitor delay count is at 0"]
    #[inline(always)]
    pub fn mon_dly_ok(&self) -> MonDlyOkR {
        MonDlyOkR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Indicates the adjustment delay count is at 0"]
    #[inline(always)]
    pub fn adj_dly_ok(&self) -> AdjDlyOkR {
        AdjDlyOkR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Power supply voltage counter is at low limit"]
    #[inline(always)]
    pub fn lo_limit_det(&self) -> LoLimitDetR {
        LoLimitDetR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Power supply voltage counter is at high limit"]
    #[inline(always)]
    pub fn hi_limit_det(&self) -> HiLimitDetR {
        HiLimitDetR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - At least one delay line has been enabled"]
    #[inline(always)]
    pub fn valid_tap(&self) -> ValidTapR {
        ValidTapR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt flag that indicates a voltage count is at/beyond manufacturer limits"]
    #[inline(always)]
    pub fn limit_err(&self) -> LimitErrR {
        LimitErrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt flag that indicates a tap has an invalid value"]
    #[inline(always)]
    pub fn range_err(&self) -> RangeErrR {
        RangeErrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt flag that indicates up and down adjustment requested simultaneously"]
    #[inline(always)]
    pub fn adj_err(&self) -> AdjErrR {
        AdjErrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Indicates the ref select register bit is out of range"]
    #[inline(always)]
    pub fn ref_sel_err(&self) -> RefSelErrR {
        RefSelErrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt flag that indicates a timeout while adjusting the voltage"]
    #[inline(always)]
    pub fn fb_to_err(&self) -> FbToErrR {
        FbToErrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt flag that mirror FB_TO_ERR and is write one clear"]
    #[inline(always)]
    pub fn fb_to_err_s(&self) -> FbToErrSR {
        FbToErrSR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt flag that indicates the power supply voltage requested is below the low threshold"]
    #[inline(always)]
    pub fn fc_lv_det_int(&self) -> FcLvDetIntR {
        FcLvDetIntR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt flag that mirrors FC_LV_DET_INT"]
    #[inline(always)]
    pub fn fc_lv_det_s(&self) -> FcLvDetSR {
        FcLvDetSR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - State machine state"]
    #[inline(always)]
    pub fn dvs_state(&mut self) -> DvsStateW<StatSpec> {
        DvsStateW::new(self, 0)
    }
    #[doc = "Bit 4 - DVS Raising voltage"]
    #[inline(always)]
    pub fn adj_up_ena(&mut self) -> AdjUpEnaW<StatSpec> {
        AdjUpEnaW::new(self, 4)
    }
    #[doc = "Bit 5 - DVS Lowering voltage"]
    #[inline(always)]
    pub fn adj_dwn_ena(&mut self) -> AdjDwnEnaW<StatSpec> {
        AdjDwnEnaW::new(self, 5)
    }
    #[doc = "Bit 6 - Adjustment to a Direct Voltage"]
    #[inline(always)]
    pub fn adj_active(&mut self) -> AdjActiveW<StatSpec> {
        AdjActiveW::new(self, 6)
    }
    #[doc = "Bit 7 - Tap Enabled and the Tap is withing Hi/Low limits"]
    #[inline(always)]
    pub fn ctr_tap_ok(&mut self) -> CtrTapOkW<StatSpec> {
        CtrTapOkW::new(self, 7)
    }
    #[doc = "Bit 8 - Status of selected center tap delay line detect output"]
    #[inline(always)]
    pub fn ctr_tap_sel(&mut self) -> CtrTapSelW<StatSpec> {
        CtrTapSelW::new(self, 8)
    }
    #[doc = "Bit 9 - Provides the current combined status of all selected Low Range delay lines"]
    #[inline(always)]
    pub fn slow_trip_det(&mut self) -> SlowTripDetW<StatSpec> {
        SlowTripDetW::new(self, 9)
    }
    #[doc = "Bit 10 - Provides the current combined status of all selected High Range delay lines"]
    #[inline(always)]
    pub fn fast_trip_det(&mut self) -> FastTripDetW<StatSpec> {
        FastTripDetW::new(self, 10)
    }
    #[doc = "Bit 11 - Indicates if the power supply is in range"]
    #[inline(always)]
    pub fn ps_in_range(&mut self) -> PsInRangeW<StatSpec> {
        PsInRangeW::new(self, 11)
    }
    #[doc = "Bits 12:18 - Voltage Count value sent to the power supply"]
    #[inline(always)]
    pub fn ps_vcntr(&mut self) -> PsVcntrW<StatSpec> {
        PsVcntrW::new(self, 12)
    }
    #[doc = "Bit 19 - Indicates the monitor delay count is at 0"]
    #[inline(always)]
    pub fn mon_dly_ok(&mut self) -> MonDlyOkW<StatSpec> {
        MonDlyOkW::new(self, 19)
    }
    #[doc = "Bit 20 - Indicates the adjustment delay count is at 0"]
    #[inline(always)]
    pub fn adj_dly_ok(&mut self) -> AdjDlyOkW<StatSpec> {
        AdjDlyOkW::new(self, 20)
    }
    #[doc = "Bit 21 - Power supply voltage counter is at low limit"]
    #[inline(always)]
    pub fn lo_limit_det(&mut self) -> LoLimitDetW<StatSpec> {
        LoLimitDetW::new(self, 21)
    }
    #[doc = "Bit 22 - Power supply voltage counter is at high limit"]
    #[inline(always)]
    pub fn hi_limit_det(&mut self) -> HiLimitDetW<StatSpec> {
        HiLimitDetW::new(self, 22)
    }
    #[doc = "Bit 23 - At least one delay line has been enabled"]
    #[inline(always)]
    pub fn valid_tap(&mut self) -> ValidTapW<StatSpec> {
        ValidTapW::new(self, 23)
    }
    #[doc = "Bit 24 - Interrupt flag that indicates a voltage count is at/beyond manufacturer limits"]
    #[inline(always)]
    pub fn limit_err(&mut self) -> LimitErrW<StatSpec> {
        LimitErrW::new(self, 24)
    }
    #[doc = "Bit 25 - Interrupt flag that indicates a tap has an invalid value"]
    #[inline(always)]
    pub fn range_err(&mut self) -> RangeErrW<StatSpec> {
        RangeErrW::new(self, 25)
    }
    #[doc = "Bit 26 - Interrupt flag that indicates up and down adjustment requested simultaneously"]
    #[inline(always)]
    pub fn adj_err(&mut self) -> AdjErrW<StatSpec> {
        AdjErrW::new(self, 26)
    }
    #[doc = "Bit 27 - Indicates the ref select register bit is out of range"]
    #[inline(always)]
    pub fn ref_sel_err(&mut self) -> RefSelErrW<StatSpec> {
        RefSelErrW::new(self, 27)
    }
    #[doc = "Bit 28 - Interrupt flag that indicates a timeout while adjusting the voltage"]
    #[inline(always)]
    pub fn fb_to_err(&mut self) -> FbToErrW<StatSpec> {
        FbToErrW::new(self, 28)
    }
    #[doc = "Bit 29 - Interrupt flag that mirror FB_TO_ERR and is write one clear"]
    #[inline(always)]
    pub fn fb_to_err_s(&mut self) -> FbToErrSW<StatSpec> {
        FbToErrSW::new(self, 29)
    }
    #[doc = "Bit 30 - Interrupt flag that indicates the power supply voltage requested is below the low threshold"]
    #[inline(always)]
    pub fn fc_lv_det_int(&mut self) -> FcLvDetIntW<StatSpec> {
        FcLvDetIntW::new(self, 30)
    }
    #[doc = "Bit 31 - Interrupt flag that mirrors FC_LV_DET_INT"]
    #[inline(always)]
    pub fn fc_lv_det_s(&mut self) -> FcLvDetSW<StatSpec> {
        FcLvDetSW::new(self, 31)
    }
}
#[doc = "Status Fields\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
