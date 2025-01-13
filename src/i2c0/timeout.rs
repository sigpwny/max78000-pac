#[doc = "Register `TIMEOUT` reader"]
pub type R = crate::R<TimeoutSpec>;
#[doc = "Register `TIMEOUT` writer"]
pub type W = crate::W<TimeoutSpec>;
#[doc = "Field `SCL_TO_VAL` reader - Timeout"]
pub type SclToValR = crate::FieldReader<u16>;
#[doc = "Field `SCL_TO_VAL` writer - Timeout"]
pub type SclToValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timeout"]
    #[inline(always)]
    pub fn scl_to_val(&self) -> SclToValR {
        SclToValR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout"]
    #[inline(always)]
    pub fn scl_to_val(&mut self) -> SclToValW<TimeoutSpec> {
        SclToValW::new(self, 0)
    }
}
#[doc = "Timeout Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeoutSpec;
impl crate::RegisterSpec for TimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout::R`](R) reader structure"]
impl crate::Readable for TimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`timeout::W`](W) writer structure"]
impl crate::Writable for TimeoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMEOUT to value 0"]
impl crate::Resettable for TimeoutSpec {
    const RESET_VALUE: u32 = 0;
}
