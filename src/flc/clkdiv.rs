#[doc = "Register `CLKDIV` reader"]
pub type R = crate::R<ClkdivSpec>;
#[doc = "Register `CLKDIV` writer"]
pub type W = crate::W<ClkdivSpec>;
#[doc = "Field `CLKDIV` reader - Flash Clock Divide. The clock is divided by this value to generate a 1MHz clock for flash controller."]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Flash Clock Divide. The clock is divided by this value to generate a 1MHz clock for flash controller."]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Flash Clock Divide. The clock is divided by this value to generate a 1MHz clock for flash controller."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Flash Clock Divide. The clock is divided by this value to generate a 1MHz clock for flash controller."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<ClkdivSpec> {
        ClkdivW::new(self, 0)
    }
}
#[doc = "Flash Clock Divide. The clock (PLL0) is divided by this value to generate a 1 MHz clock for Flash controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkdivSpec;
impl crate::RegisterSpec for ClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv::R`](R) reader structure"]
impl crate::Readable for ClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv::W`](W) writer structure"]
impl crate::Writable for ClkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKDIV to value 0x64"]
impl crate::Resettable for ClkdivSpec {
    const RESET_VALUE: u32 = 0x64;
}
