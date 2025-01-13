#[doc = "Register `irq0` reader"]
pub type R = crate::R<Irq0Spec>;
#[doc = "Register `irq0` writer"]
pub type W = crate::W<Irq0Spec>;
#[doc = "Field `en` reader - "]
pub type EnR = crate::BitReader;
#[doc = "Field `en` writer - "]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cm4_irq` reader - "]
pub type Cm4IrqR = crate::BitReader;
#[doc = "Field `cm4_irq` writer - "]
pub type Cm4IrqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cm4_irq(&self) -> Cm4IrqR {
        Cm4IrqR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<Irq0Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cm4_irq(&mut self) -> Cm4IrqW<Irq0Spec> {
        Cm4IrqW::new(self, 16)
    }
}
#[doc = "Semaphore IRQ0 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`irq0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Irq0Spec;
impl crate::RegisterSpec for Irq0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq0::R`](R) reader structure"]
impl crate::Readable for Irq0Spec {}
#[doc = "`write(|w| ..)` method takes [`irq0::W`](W) writer structure"]
impl crate::Writable for Irq0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets irq0 to value 0"]
impl crate::Resettable for Irq0Spec {
    const RESET_VALUE: u32 = 0;
}
