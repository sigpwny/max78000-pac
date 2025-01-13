#[doc = "Register `CLK_DIV_1US` reader"]
pub type R = crate::R<ClkDiv1usSpec>;
#[doc = "Register `CLK_DIV_1US` writer"]
pub type W = crate::W<ClkDiv1usSpec>;
#[doc = "Field `divisor` reader - Clock Divisor for 1Mhz."]
pub type DivisorR = crate::FieldReader;
#[doc = "Field `divisor` writer - Clock Divisor for 1Mhz."]
pub type DivisorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock Divisor for 1Mhz."]
    #[inline(always)]
    pub fn divisor(&self) -> DivisorR {
        DivisorR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Divisor for 1Mhz."]
    #[inline(always)]
    pub fn divisor(&mut self) -> DivisorW<ClkDiv1usSpec> {
        DivisorW::new(self, 0)
    }
}
#[doc = "1-Wire Master Clock Divisor.\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_div_1us::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_div_1us::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkDiv1usSpec;
impl crate::RegisterSpec for ClkDiv1usSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_div_1us::R`](R) reader structure"]
impl crate::Readable for ClkDiv1usSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_div_1us::W`](W) writer structure"]
impl crate::Writable for ClkDiv1usSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_DIV_1US to value 0"]
impl crate::Resettable for ClkDiv1usSpec {
    const RESET_VALUE: u32 = 0;
}
