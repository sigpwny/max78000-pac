#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `IRQ_CLR` reader - Clear Interrupt."]
pub type IrqClrR = crate::BitReader;
#[doc = "Field `IRQ_CLR` writer - Clear Interrupt."]
pub type IrqClrW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear Interrupt."]
    #[inline(always)]
    pub fn irq_clr(&self) -> IrqClrR {
        IrqClrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Interrupt."]
    #[inline(always)]
    pub fn irq_clr(&mut self) -> IrqClrW<IntrSpec> {
        IrqClrW::new(self, 0)
    }
}
#[doc = "Clear Interrupt. Writing a value (0 or 1) to a bit in this register clears the associated interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSpec;
impl crate::RegisterSpec for IntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for IntrSpec {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for IntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for IntrSpec {
    const RESET_VALUE: u32 = 0;
}
