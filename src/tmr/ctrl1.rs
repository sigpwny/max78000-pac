#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `CLKSEL_A` reader - Timer Clock Select for Timer A"]
pub type ClkselAR = crate::FieldReader;
#[doc = "Field `CLKSEL_A` writer - Timer Clock Select for Timer A"]
pub type ClkselAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLKEN_A` reader - Timer A Enable Status"]
pub type ClkenAR = crate::BitReader;
#[doc = "Field `CLKEN_A` writer - Timer A Enable Status"]
pub type ClkenAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKRDY_A` reader - CLK_TMR Ready Flag for Timer A"]
pub type ClkrdyAR = crate::BitReader;
#[doc = "Field `CLKRDY_A` writer - CLK_TMR Ready Flag for Timer A"]
pub type ClkrdyAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENT_SEL_A` reader - Event Select for Timer A"]
pub type EventSelAR = crate::FieldReader;
#[doc = "Field `EVENT_SEL_A` writer - Event Select for Timer A"]
pub type EventSelAW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NEGTRIG_A` reader - Negative Edge Trigger for Event for Timer A"]
pub type NegtrigAR = crate::BitReader;
#[doc = "Field `NEGTRIG_A` writer - Negative Edge Trigger for Event for Timer A"]
pub type NegtrigAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE_A` reader - Interrupt Enable for Timer A"]
pub type IeAR = crate::BitReader;
#[doc = "Field `IE_A` writer - Interrupt Enable for Timer A"]
pub type IeAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPEVENT_SEL_A` reader - Capture Event Select for Timer A"]
pub type CapeventSelAR = crate::FieldReader;
#[doc = "Field `CAPEVENT_SEL_A` writer - Capture Event Select for Timer A"]
pub type CapeventSelAW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_CAPEVENT_A` reader - Software Capture Event for Timer A"]
pub type SwCapeventAR = crate::BitReader;
#[doc = "Field `SW_CAPEVENT_A` writer - Software Capture Event for Timer A"]
pub type SwCapeventAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WE_A` reader - Wake-Up Enable for Timer A"]
pub type WeAR = crate::BitReader;
#[doc = "Field `WE_A` writer - Wake-Up Enable for Timer A"]
pub type WeAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEN_A` reader - OUT_OE_O Enable for Modes 0, 1,and 5 for Timer A"]
pub type OutenAR = crate::BitReader;
#[doc = "Field `OUTEN_A` writer - OUT_OE_O Enable for Modes 0, 1,and 5 for Timer A"]
pub type OutenAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTBEN_A` reader - PWM_CKB_EN_O Enable for Modes other than Mode 3 for Timer A"]
pub type OutbenAR = crate::BitReader;
#[doc = "Field `OUTBEN_A` writer - PWM_CKB_EN_O Enable for Modes other than Mode 3 for Timer A"]
pub type OutbenAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNC` reader - Allows asynchronous reads of the PWM and CNT registers."]
pub type AsyncR = crate::BitReader;
#[doc = "Field `ASYNC` writer - Allows asynchronous reads of the PWM and CNT registers."]
pub type AsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSEL_B` reader - Timer Clock Select for Timer B"]
pub type ClkselBR = crate::FieldReader;
#[doc = "Field `CLKSEL_B` writer - Timer Clock Select for Timer B"]
pub type ClkselBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLKEN_B` reader - Timer B Enable Status"]
pub type ClkenBR = crate::BitReader;
#[doc = "Field `CLKEN_B` writer - Timer B Enable Status"]
pub type ClkenBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKRDY_B` reader - CLK_TMR Ready Flag for Timer B"]
pub type ClkrdyBR = crate::BitReader;
#[doc = "Field `CLKRDY_B` writer - CLK_TMR Ready Flag for Timer B"]
pub type ClkrdyBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVENT_SEL_B` reader - Event Select for Timer B"]
pub type EventSelBR = crate::FieldReader;
#[doc = "Field `EVENT_SEL_B` writer - Event Select for Timer B"]
pub type EventSelBW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NEGTRIG_B` reader - Negative Edge Trigger for Event for Timer B"]
pub type NegtrigBR = crate::BitReader;
#[doc = "Field `NEGTRIG_B` writer - Negative Edge Trigger for Event for Timer B"]
pub type NegtrigBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE_B` reader - Interrupt Enable for Timer B"]
pub type IeBR = crate::BitReader;
#[doc = "Field `IE_B` writer - Interrupt Enable for Timer B"]
pub type IeBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPEVENT_SEL_B` reader - Capture Event Select for Timer B"]
pub type CapeventSelBR = crate::FieldReader;
#[doc = "Field `CAPEVENT_SEL_B` writer - Capture Event Select for Timer B"]
pub type CapeventSelBW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_CAPEVENT_B` reader - Software Capture Event for Timer B"]
pub type SwCapeventBR = crate::BitReader;
#[doc = "Field `SW_CAPEVENT_B` writer - Software Capture Event for Timer B"]
pub type SwCapeventBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WE_B` reader - Wake-Up Enable for Timer B"]
pub type WeBR = crate::BitReader;
#[doc = "Field `WE_B` writer - Wake-Up Enable for Timer B"]
pub type WeBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CASCADE` reader - Cascade two 16-bit timers into one 32-bit timer. Only available when C_TMR16=0 adn C_DUALTMR16=1."]
pub type CascadeR = crate::BitReader;
#[doc = "Field `CASCADE` writer - Cascade two 16-bit timers into one 32-bit timer. Only available when C_TMR16=0 adn C_DUALTMR16=1."]
pub type CascadeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Timer Clock Select for Timer A"]
    #[inline(always)]
    pub fn clksel_a(&self) -> ClkselAR {
        ClkselAR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Timer A Enable Status"]
    #[inline(always)]
    pub fn clken_a(&self) -> ClkenAR {
        ClkenAR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CLK_TMR Ready Flag for Timer A"]
    #[inline(always)]
    pub fn clkrdy_a(&self) -> ClkrdyAR {
        ClkrdyAR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Event Select for Timer A"]
    #[inline(always)]
    pub fn event_sel_a(&self) -> EventSelAR {
        EventSelAR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Negative Edge Trigger for Event for Timer A"]
    #[inline(always)]
    pub fn negtrig_a(&self) -> NegtrigAR {
        NegtrigAR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Enable for Timer A"]
    #[inline(always)]
    pub fn ie_a(&self) -> IeAR {
        IeAR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Capture Event Select for Timer A"]
    #[inline(always)]
    pub fn capevent_sel_a(&self) -> CapeventSelAR {
        CapeventSelAR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Software Capture Event for Timer A"]
    #[inline(always)]
    pub fn sw_capevent_a(&self) -> SwCapeventAR {
        SwCapeventAR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wake-Up Enable for Timer A"]
    #[inline(always)]
    pub fn we_a(&self) -> WeAR {
        WeAR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OUT_OE_O Enable for Modes 0, 1,and 5 for Timer A"]
    #[inline(always)]
    pub fn outen_a(&self) -> OutenAR {
        OutenAR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PWM_CKB_EN_O Enable for Modes other than Mode 3 for Timer A"]
    #[inline(always)]
    pub fn outben_a(&self) -> OutbenAR {
        OutbenAR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Allows asynchronous reads of the PWM and CNT registers."]
    #[inline(always)]
    pub fn async_(&self) -> AsyncR {
        AsyncR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Timer Clock Select for Timer B"]
    #[inline(always)]
    pub fn clksel_b(&self) -> ClkselBR {
        ClkselBR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Timer B Enable Status"]
    #[inline(always)]
    pub fn clken_b(&self) -> ClkenBR {
        ClkenBR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CLK_TMR Ready Flag for Timer B"]
    #[inline(always)]
    pub fn clkrdy_b(&self) -> ClkrdyBR {
        ClkrdyBR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Event Select for Timer B"]
    #[inline(always)]
    pub fn event_sel_b(&self) -> EventSelBR {
        EventSelBR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Negative Edge Trigger for Event for Timer B"]
    #[inline(always)]
    pub fn negtrig_b(&self) -> NegtrigBR {
        NegtrigBR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Enable for Timer B"]
    #[inline(always)]
    pub fn ie_b(&self) -> IeBR {
        IeBR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Capture Event Select for Timer B"]
    #[inline(always)]
    pub fn capevent_sel_b(&self) -> CapeventSelBR {
        CapeventSelBR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Software Capture Event for Timer B"]
    #[inline(always)]
    pub fn sw_capevent_b(&self) -> SwCapeventBR {
        SwCapeventBR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wake-Up Enable for Timer B"]
    #[inline(always)]
    pub fn we_b(&self) -> WeBR {
        WeBR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Cascade two 16-bit timers into one 32-bit timer. Only available when C_TMR16=0 adn C_DUALTMR16=1."]
    #[inline(always)]
    pub fn cascade(&self) -> CascadeR {
        CascadeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Clock Select for Timer A"]
    #[inline(always)]
    pub fn clksel_a(&mut self) -> ClkselAW<Ctrl1Spec> {
        ClkselAW::new(self, 0)
    }
    #[doc = "Bit 2 - Timer A Enable Status"]
    #[inline(always)]
    pub fn clken_a(&mut self) -> ClkenAW<Ctrl1Spec> {
        ClkenAW::new(self, 2)
    }
    #[doc = "Bit 3 - CLK_TMR Ready Flag for Timer A"]
    #[inline(always)]
    pub fn clkrdy_a(&mut self) -> ClkrdyAW<Ctrl1Spec> {
        ClkrdyAW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Event Select for Timer A"]
    #[inline(always)]
    pub fn event_sel_a(&mut self) -> EventSelAW<Ctrl1Spec> {
        EventSelAW::new(self, 4)
    }
    #[doc = "Bit 7 - Negative Edge Trigger for Event for Timer A"]
    #[inline(always)]
    pub fn negtrig_a(&mut self) -> NegtrigAW<Ctrl1Spec> {
        NegtrigAW::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt Enable for Timer A"]
    #[inline(always)]
    pub fn ie_a(&mut self) -> IeAW<Ctrl1Spec> {
        IeAW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Capture Event Select for Timer A"]
    #[inline(always)]
    pub fn capevent_sel_a(&mut self) -> CapeventSelAW<Ctrl1Spec> {
        CapeventSelAW::new(self, 9)
    }
    #[doc = "Bit 11 - Software Capture Event for Timer A"]
    #[inline(always)]
    pub fn sw_capevent_a(&mut self) -> SwCapeventAW<Ctrl1Spec> {
        SwCapeventAW::new(self, 11)
    }
    #[doc = "Bit 12 - Wake-Up Enable for Timer A"]
    #[inline(always)]
    pub fn we_a(&mut self) -> WeAW<Ctrl1Spec> {
        WeAW::new(self, 12)
    }
    #[doc = "Bit 13 - OUT_OE_O Enable for Modes 0, 1,and 5 for Timer A"]
    #[inline(always)]
    pub fn outen_a(&mut self) -> OutenAW<Ctrl1Spec> {
        OutenAW::new(self, 13)
    }
    #[doc = "Bit 14 - PWM_CKB_EN_O Enable for Modes other than Mode 3 for Timer A"]
    #[inline(always)]
    pub fn outben_a(&mut self) -> OutbenAW<Ctrl1Spec> {
        OutbenAW::new(self, 14)
    }
    #[doc = "Bit 15 - Allows asynchronous reads of the PWM and CNT registers."]
    #[inline(always)]
    pub fn async_(&mut self) -> AsyncW<Ctrl1Spec> {
        AsyncW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Timer Clock Select for Timer B"]
    #[inline(always)]
    pub fn clksel_b(&mut self) -> ClkselBW<Ctrl1Spec> {
        ClkselBW::new(self, 16)
    }
    #[doc = "Bit 18 - Timer B Enable Status"]
    #[inline(always)]
    pub fn clken_b(&mut self) -> ClkenBW<Ctrl1Spec> {
        ClkenBW::new(self, 18)
    }
    #[doc = "Bit 19 - CLK_TMR Ready Flag for Timer B"]
    #[inline(always)]
    pub fn clkrdy_b(&mut self) -> ClkrdyBW<Ctrl1Spec> {
        ClkrdyBW::new(self, 19)
    }
    #[doc = "Bits 20:22 - Event Select for Timer B"]
    #[inline(always)]
    pub fn event_sel_b(&mut self) -> EventSelBW<Ctrl1Spec> {
        EventSelBW::new(self, 20)
    }
    #[doc = "Bit 23 - Negative Edge Trigger for Event for Timer B"]
    #[inline(always)]
    pub fn negtrig_b(&mut self) -> NegtrigBW<Ctrl1Spec> {
        NegtrigBW::new(self, 23)
    }
    #[doc = "Bit 24 - Interrupt Enable for Timer B"]
    #[inline(always)]
    pub fn ie_b(&mut self) -> IeBW<Ctrl1Spec> {
        IeBW::new(self, 24)
    }
    #[doc = "Bits 25:26 - Capture Event Select for Timer B"]
    #[inline(always)]
    pub fn capevent_sel_b(&mut self) -> CapeventSelBW<Ctrl1Spec> {
        CapeventSelBW::new(self, 25)
    }
    #[doc = "Bit 27 - Software Capture Event for Timer B"]
    #[inline(always)]
    pub fn sw_capevent_b(&mut self) -> SwCapeventBW<Ctrl1Spec> {
        SwCapeventBW::new(self, 27)
    }
    #[doc = "Bit 28 - Wake-Up Enable for Timer B"]
    #[inline(always)]
    pub fn we_b(&mut self) -> WeBW<Ctrl1Spec> {
        WeBW::new(self, 28)
    }
    #[doc = "Bit 31 - Cascade two 16-bit timers into one 32-bit timer. Only available when C_TMR16=0 adn C_DUALTMR16=1."]
    #[inline(always)]
    pub fn cascade(&mut self) -> CascadeW<Ctrl1Spec> {
        CascadeW::new(self, 31)
    }
}
#[doc = "Timer Configuration Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
