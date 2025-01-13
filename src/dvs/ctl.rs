#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `MON_ENA` reader - Enable the DVS monitoring circuit"]
pub type MonEnaR = crate::BitReader;
#[doc = "Field `MON_ENA` writer - Enable the DVS monitoring circuit"]
pub type MonEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ_ENA` reader - Enable the power supply adjustment based on measurements"]
pub type AdjEnaR = crate::BitReader;
#[doc = "Field `ADJ_ENA` writer - Enable the power supply adjustment based on measurements"]
pub type AdjEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS_FB_DIS` reader - Power Supply Feedback Disable"]
pub type PsFbDisR = crate::BitReader;
#[doc = "Field `PS_FB_DIS` writer - Power Supply Feedback Disable"]
pub type PsFbDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_TAP_ENA` reader - Use the TAP Select for automatic adjustment or monitoring"]
pub type CtrlTapEnaR = crate::BitReader;
#[doc = "Field `CTRL_TAP_ENA` writer - Use the TAP Select for automatic adjustment or monitoring"]
pub type CtrlTapEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROP_DLY` reader - Additional delay to monitor lines"]
pub type PropDlyR = crate::FieldReader;
#[doc = "Field `PROP_DLY` writer - Additional delay to monitor lines"]
pub type PropDlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MON_ONESHOT` reader - Measure delay once"]
pub type MonOneshotR = crate::BitReader;
#[doc = "Field `MON_ONESHOT` writer - Measure delay once"]
pub type MonOneshotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GO_DIRECT` reader - Operate in automatic mode or move directly"]
pub type GoDirectR = crate::BitReader;
#[doc = "Field `GO_DIRECT` writer - Operate in automatic mode or move directly"]
pub type GoDirectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRECT_REG` reader - Step incrementally to target voltage"]
pub type DirectRegR = crate::BitReader;
#[doc = "Field `DIRECT_REG` writer - Step incrementally to target voltage"]
pub type DirectRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIME_ENA` reader - Include a delay line priming signal before monitoring"]
pub type PrimeEnaR = crate::BitReader;
#[doc = "Field `PRIME_ENA` writer - Include a delay line priming signal before monitoring"]
pub type PrimeEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LIMIT_IE` reader - Enable Limit Error Interrupt"]
pub type LimitIeR = crate::BitReader;
#[doc = "Field `LIMIT_IE` writer - Enable Limit Error Interrupt"]
pub type LimitIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RANGE_IE` reader - Enable Range Error Interrupt"]
pub type RangeIeR = crate::BitReader;
#[doc = "Field `RANGE_IE` writer - Enable Range Error Interrupt"]
pub type RangeIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ_IE` reader - Enable Adjustment Error Interrupt"]
pub type AdjIeR = crate::BitReader;
#[doc = "Field `ADJ_IE` writer - Enable Adjustment Error Interrupt"]
pub type AdjIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REF_SEL` reader - Select TAP used for voltage adjustment"]
pub type RefSelR = crate::FieldReader;
#[doc = "Field `REF_SEL` writer - Select TAP used for voltage adjustment"]
pub type RefSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INC_VAL` reader - Step size to increment voltage when in automatic mode"]
pub type IncValR = crate::FieldReader;
#[doc = "Field `INC_VAL` writer - Step size to increment voltage when in automatic mode"]
pub type IncValW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DVS_PS_APB_DIS` reader - Prevent the application code from adjusting Vcore"]
pub type DvsPsApbDisR = crate::BitReader;
#[doc = "Field `DVS_PS_APB_DIS` writer - Prevent the application code from adjusting Vcore"]
pub type DvsPsApbDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DVS_HI_RANGE_ANY` reader - Any high range signal from a delay line will cause a voltage adjustment"]
pub type DvsHiRangeAnyR = crate::BitReader;
#[doc = "Field `DVS_HI_RANGE_ANY` writer - Any high range signal from a delay line will cause a voltage adjustment"]
pub type DvsHiRangeAnyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FB_TO_IE` reader - Enable Voltage Adjustment Timeout Interrupt"]
pub type FbToIeR = crate::BitReader;
#[doc = "Field `FB_TO_IE` writer - Enable Voltage Adjustment Timeout Interrupt"]
pub type FbToIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FC_LV_IE` reader - Enable Low Voltage Interrupt"]
pub type FcLvIeR = crate::BitReader;
#[doc = "Field `FC_LV_IE` writer - Enable Low Voltage Interrupt"]
pub type FcLvIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD_ACK_ENA` reader - Prevent DVS from ack'ing a request to enter a low power mode until in the idle state"]
pub type PdAckEnaR = crate::BitReader;
#[doc = "Field `PD_ACK_ENA` writer - Prevent DVS from ack'ing a request to enter a low power mode until in the idle state"]
pub type PdAckEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ_ABORT` reader - Causes the DVS to enter the idle state immediately on a request to enter a low power mode"]
pub type AdjAbortR = crate::BitReader;
#[doc = "Field `ADJ_ABORT` writer - Causes the DVS to enter the idle state immediately on a request to enter a low power mode"]
pub type AdjAbortW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the DVS monitoring circuit"]
    #[inline(always)]
    pub fn mon_ena(&self) -> MonEnaR {
        MonEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the power supply adjustment based on measurements"]
    #[inline(always)]
    pub fn adj_ena(&self) -> AdjEnaR {
        AdjEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Supply Feedback Disable"]
    #[inline(always)]
    pub fn ps_fb_dis(&self) -> PsFbDisR {
        PsFbDisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Use the TAP Select for automatic adjustment or monitoring"]
    #[inline(always)]
    pub fn ctrl_tap_ena(&self) -> CtrlTapEnaR {
        CtrlTapEnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Additional delay to monitor lines"]
    #[inline(always)]
    pub fn prop_dly(&self) -> PropDlyR {
        PropDlyR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Measure delay once"]
    #[inline(always)]
    pub fn mon_oneshot(&self) -> MonOneshotR {
        MonOneshotR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Operate in automatic mode or move directly"]
    #[inline(always)]
    pub fn go_direct(&self) -> GoDirectR {
        GoDirectR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Step incrementally to target voltage"]
    #[inline(always)]
    pub fn direct_reg(&self) -> DirectRegR {
        DirectRegR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Include a delay line priming signal before monitoring"]
    #[inline(always)]
    pub fn prime_ena(&self) -> PrimeEnaR {
        PrimeEnaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Limit Error Interrupt"]
    #[inline(always)]
    pub fn limit_ie(&self) -> LimitIeR {
        LimitIeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Range Error Interrupt"]
    #[inline(always)]
    pub fn range_ie(&self) -> RangeIeR {
        RangeIeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Adjustment Error Interrupt"]
    #[inline(always)]
    pub fn adj_ie(&self) -> AdjIeR {
        AdjIeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - Select TAP used for voltage adjustment"]
    #[inline(always)]
    pub fn ref_sel(&self) -> RefSelR {
        RefSelR::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:19 - Step size to increment voltage when in automatic mode"]
    #[inline(always)]
    pub fn inc_val(&self) -> IncValR {
        IncValR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Prevent the application code from adjusting Vcore"]
    #[inline(always)]
    pub fn dvs_ps_apb_dis(&self) -> DvsPsApbDisR {
        DvsPsApbDisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Any high range signal from a delay line will cause a voltage adjustment"]
    #[inline(always)]
    pub fn dvs_hi_range_any(&self) -> DvsHiRangeAnyR {
        DvsHiRangeAnyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Voltage Adjustment Timeout Interrupt"]
    #[inline(always)]
    pub fn fb_to_ie(&self) -> FbToIeR {
        FbToIeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Low Voltage Interrupt"]
    #[inline(always)]
    pub fn fc_lv_ie(&self) -> FcLvIeR {
        FcLvIeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Prevent DVS from ack'ing a request to enter a low power mode until in the idle state"]
    #[inline(always)]
    pub fn pd_ack_ena(&self) -> PdAckEnaR {
        PdAckEnaR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Causes the DVS to enter the idle state immediately on a request to enter a low power mode"]
    #[inline(always)]
    pub fn adj_abort(&self) -> AdjAbortR {
        AdjAbortR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the DVS monitoring circuit"]
    #[inline(always)]
    pub fn mon_ena(&mut self) -> MonEnaW<CtlSpec> {
        MonEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the power supply adjustment based on measurements"]
    #[inline(always)]
    pub fn adj_ena(&mut self) -> AdjEnaW<CtlSpec> {
        AdjEnaW::new(self, 1)
    }
    #[doc = "Bit 2 - Power Supply Feedback Disable"]
    #[inline(always)]
    pub fn ps_fb_dis(&mut self) -> PsFbDisW<CtlSpec> {
        PsFbDisW::new(self, 2)
    }
    #[doc = "Bit 3 - Use the TAP Select for automatic adjustment or monitoring"]
    #[inline(always)]
    pub fn ctrl_tap_ena(&mut self) -> CtrlTapEnaW<CtlSpec> {
        CtrlTapEnaW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Additional delay to monitor lines"]
    #[inline(always)]
    pub fn prop_dly(&mut self) -> PropDlyW<CtlSpec> {
        PropDlyW::new(self, 4)
    }
    #[doc = "Bit 6 - Measure delay once"]
    #[inline(always)]
    pub fn mon_oneshot(&mut self) -> MonOneshotW<CtlSpec> {
        MonOneshotW::new(self, 6)
    }
    #[doc = "Bit 7 - Operate in automatic mode or move directly"]
    #[inline(always)]
    pub fn go_direct(&mut self) -> GoDirectW<CtlSpec> {
        GoDirectW::new(self, 7)
    }
    #[doc = "Bit 8 - Step incrementally to target voltage"]
    #[inline(always)]
    pub fn direct_reg(&mut self) -> DirectRegW<CtlSpec> {
        DirectRegW::new(self, 8)
    }
    #[doc = "Bit 9 - Include a delay line priming signal before monitoring"]
    #[inline(always)]
    pub fn prime_ena(&mut self) -> PrimeEnaW<CtlSpec> {
        PrimeEnaW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Limit Error Interrupt"]
    #[inline(always)]
    pub fn limit_ie(&mut self) -> LimitIeW<CtlSpec> {
        LimitIeW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Range Error Interrupt"]
    #[inline(always)]
    pub fn range_ie(&mut self) -> RangeIeW<CtlSpec> {
        RangeIeW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Adjustment Error Interrupt"]
    #[inline(always)]
    pub fn adj_ie(&mut self) -> AdjIeW<CtlSpec> {
        AdjIeW::new(self, 12)
    }
    #[doc = "Bits 13:16 - Select TAP used for voltage adjustment"]
    #[inline(always)]
    pub fn ref_sel(&mut self) -> RefSelW<CtlSpec> {
        RefSelW::new(self, 13)
    }
    #[doc = "Bits 17:19 - Step size to increment voltage when in automatic mode"]
    #[inline(always)]
    pub fn inc_val(&mut self) -> IncValW<CtlSpec> {
        IncValW::new(self, 17)
    }
    #[doc = "Bit 20 - Prevent the application code from adjusting Vcore"]
    #[inline(always)]
    pub fn dvs_ps_apb_dis(&mut self) -> DvsPsApbDisW<CtlSpec> {
        DvsPsApbDisW::new(self, 20)
    }
    #[doc = "Bit 21 - Any high range signal from a delay line will cause a voltage adjustment"]
    #[inline(always)]
    pub fn dvs_hi_range_any(&mut self) -> DvsHiRangeAnyW<CtlSpec> {
        DvsHiRangeAnyW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Voltage Adjustment Timeout Interrupt"]
    #[inline(always)]
    pub fn fb_to_ie(&mut self) -> FbToIeW<CtlSpec> {
        FbToIeW::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Low Voltage Interrupt"]
    #[inline(always)]
    pub fn fc_lv_ie(&mut self) -> FcLvIeW<CtlSpec> {
        FcLvIeW::new(self, 23)
    }
    #[doc = "Bit 24 - Prevent DVS from ack'ing a request to enter a low power mode until in the idle state"]
    #[inline(always)]
    pub fn pd_ack_ena(&mut self) -> PdAckEnaW<CtlSpec> {
        PdAckEnaW::new(self, 24)
    }
    #[doc = "Bit 25 - Causes the DVS to enter the idle state immediately on a request to enter a low power mode"]
    #[inline(always)]
    pub fn adj_abort(&mut self) -> AdjAbortW<CtlSpec> {
        AdjAbortW::new(self, 25)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
