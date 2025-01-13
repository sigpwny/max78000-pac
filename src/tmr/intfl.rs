#[doc = "Register `INTFL` reader"]
pub type R = crate::R<IntflSpec>;
#[doc = "Register `INTFL` writer"]
pub type W = crate::W<IntflSpec>;
#[doc = "Field `IRQ_A` reader - Interrupt Flag for Timer A."]
pub type IrqAR = crate::BitReader;
#[doc = "Field `IRQ_A` writer - Interrupt Flag for Timer A."]
pub type IrqAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRDONE_A` reader - Write Done Flag for Timer A indicating the write is complete from APB to CLK_TMR domain."]
pub type WrdoneAR = crate::BitReader;
#[doc = "Field `WRDONE_A` writer - Write Done Flag for Timer A indicating the write is complete from APB to CLK_TMR domain."]
pub type WrdoneAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_DIS_A` reader - Write Disable to CNT/PWM for Timer A in the non-cascaded dual timer configuration."]
pub type WrDisAR = crate::BitReader;
#[doc = "Field `WR_DIS_A` writer - Write Disable to CNT/PWM for Timer A in the non-cascaded dual timer configuration."]
pub type WrDisAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQ_B` reader - Interrupt Flag for Timer B."]
pub type IrqBR = crate::BitReader;
#[doc = "Field `IRQ_B` writer - Interrupt Flag for Timer B."]
pub type IrqBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRDONE_B` reader - Write Done Flag for Timer B indicating the write is complete from APB to CLK_TMR domain."]
pub type WrdoneBR = crate::BitReader;
#[doc = "Field `WRDONE_B` writer - Write Done Flag for Timer B indicating the write is complete from APB to CLK_TMR domain."]
pub type WrdoneBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_DIS_B` reader - Write Disable to CNT/PWM for Timer B in the non-cascaded dual timer configuration."]
pub type WrDisBR = crate::BitReader;
#[doc = "Field `WR_DIS_B` writer - Write Disable to CNT/PWM for Timer B in the non-cascaded dual timer configuration."]
pub type WrDisBW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Flag for Timer A."]
    #[inline(always)]
    pub fn irq_a(&self) -> IrqAR {
        IrqAR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Write Done Flag for Timer A indicating the write is complete from APB to CLK_TMR domain."]
    #[inline(always)]
    pub fn wrdone_a(&self) -> WrdoneAR {
        WrdoneAR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write Disable to CNT/PWM for Timer A in the non-cascaded dual timer configuration."]
    #[inline(always)]
    pub fn wr_dis_a(&self) -> WrDisAR {
        WrDisAR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Flag for Timer B."]
    #[inline(always)]
    pub fn irq_b(&self) -> IrqBR {
        IrqBR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Write Done Flag for Timer B indicating the write is complete from APB to CLK_TMR domain."]
    #[inline(always)]
    pub fn wrdone_b(&self) -> WrdoneBR {
        WrdoneBR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write Disable to CNT/PWM for Timer B in the non-cascaded dual timer configuration."]
    #[inline(always)]
    pub fn wr_dis_b(&self) -> WrDisBR {
        WrDisBR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Flag for Timer A."]
    #[inline(always)]
    pub fn irq_a(&mut self) -> IrqAW<IntflSpec> {
        IrqAW::new(self, 0)
    }
    #[doc = "Bit 8 - Write Done Flag for Timer A indicating the write is complete from APB to CLK_TMR domain."]
    #[inline(always)]
    pub fn wrdone_a(&mut self) -> WrdoneAW<IntflSpec> {
        WrdoneAW::new(self, 8)
    }
    #[doc = "Bit 9 - Write Disable to CNT/PWM for Timer A in the non-cascaded dual timer configuration."]
    #[inline(always)]
    pub fn wr_dis_a(&mut self) -> WrDisAW<IntflSpec> {
        WrDisAW::new(self, 9)
    }
    #[doc = "Bit 16 - Interrupt Flag for Timer B."]
    #[inline(always)]
    pub fn irq_b(&mut self) -> IrqBW<IntflSpec> {
        IrqBW::new(self, 16)
    }
    #[doc = "Bit 24 - Write Done Flag for Timer B indicating the write is complete from APB to CLK_TMR domain."]
    #[inline(always)]
    pub fn wrdone_b(&mut self) -> WrdoneBW<IntflSpec> {
        WrdoneBW::new(self, 24)
    }
    #[doc = "Bit 25 - Write Disable to CNT/PWM for Timer B in the non-cascaded dual timer configuration."]
    #[inline(always)]
    pub fn wr_dis_b(&mut self) -> WrDisBW<IntflSpec> {
        WrDisBW::new(self, 25)
    }
}
#[doc = "Timer Interrupt Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflSpec;
impl crate::RegisterSpec for IntflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl::R`](R) reader structure"]
impl crate::Readable for IntflSpec {}
#[doc = "`write(|w| ..)` method takes [`intfl::W`](W) writer structure"]
impl crate::Writable for IntflSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for IntflSpec {
    const RESET_VALUE: u32 = 0;
}
