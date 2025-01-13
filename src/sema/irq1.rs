#[doc = "Register `irq1` reader"]
pub type R = crate::R<Irq1Spec>;
#[doc = "Register `irq1` writer"]
pub type W = crate::W<Irq1Spec>;
#[doc = "Field `en` reader - "]
pub type EnR = crate::BitReader;
#[doc = "Field `en` writer - "]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rv32_irq` reader - "]
pub type Rv32IrqR = crate::BitReader;
#[doc = "Field `rv32_irq` writer - "]
pub type Rv32IrqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rv32_irq(&self) -> Rv32IrqR {
        Rv32IrqR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<Irq1Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rv32_irq(&mut self) -> Rv32IrqW<Irq1Spec> {
        Rv32IrqW::new(self, 16)
    }
}
#[doc = "Semaphore IRQ1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`irq1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Irq1Spec;
impl crate::RegisterSpec for Irq1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq1::R`](R) reader structure"]
impl crate::Readable for Irq1Spec {}
#[doc = "`write(|w| ..)` method takes [`irq1::W`](W) writer structure"]
impl crate::Writable for Irq1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets irq1 to value 0"]
impl crate::Resettable for Irq1Spec {
    const RESET_VALUE: u32 = 0;
}
