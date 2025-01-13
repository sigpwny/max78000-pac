#[doc = "Register `CLKLO` reader"]
pub type R = crate::R<ClkloSpec>;
#[doc = "Register `CLKLO` writer"]
pub type W = crate::W<ClkloSpec>;
#[doc = "Field `LO` reader - Clock low. In master mode, these bits define the SCL low period. In slave mode, these bits define the time SCL will be held low after data is outputted."]
pub type LoR = crate::FieldReader<u16>;
#[doc = "Field `LO` writer - Clock low. In master mode, these bits define the SCL low period. In slave mode, these bits define the time SCL will be held low after data is outputted."]
pub type LoW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Clock low. In master mode, these bits define the SCL low period. In slave mode, these bits define the time SCL will be held low after data is outputted."]
    #[inline(always)]
    pub fn lo(&self) -> LoR {
        LoR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Clock low. In master mode, these bits define the SCL low period. In slave mode, these bits define the time SCL will be held low after data is outputted."]
    #[inline(always)]
    pub fn lo(&mut self) -> LoW<ClkloSpec> {
        LoW::new(self, 0)
    }
}
#[doc = "Clock Low Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clklo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clklo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkloSpec;
impl crate::RegisterSpec for ClkloSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clklo::R`](R) reader structure"]
impl crate::Readable for ClkloSpec {}
#[doc = "`write(|w| ..)` method takes [`clklo::W`](W) writer structure"]
impl crate::Writable for ClkloSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKLO to value 0"]
impl crate::Resettable for ClkloSpec {
    const RESET_VALUE: u32 = 0;
}
