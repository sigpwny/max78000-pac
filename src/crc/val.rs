#[doc = "Register `VAL` reader"]
pub type R = crate::R<ValSpec>;
#[doc = "Register `VAL` writer"]
pub type W = crate::W<ValSpec>;
#[doc = "Field `VALUE` reader - Current CRC Value"]
pub type ValueR = crate::FieldReader<u32>;
#[doc = "Field `VALUE` writer - Current CRC Value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current CRC Value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current CRC Value"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<ValSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Current CRC Value\n\nYou can [`read`](crate::Reg::read) this register and get [`val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValSpec;
impl crate::RegisterSpec for ValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`val::R`](R) reader structure"]
impl crate::Readable for ValSpec {}
#[doc = "`write(|w| ..)` method takes [`val::W`](W) writer structure"]
impl crate::Writable for ValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VAL to value 0"]
impl crate::Resettable for ValSpec {
    const RESET_VALUE: u32 = 0;
}
