#[doc = "Register `CMP` reader"]
pub type R = crate::R<CmpSpec>;
#[doc = "Register `CMP` writer"]
pub type W = crate::W<CmpSpec>;
#[doc = "Field `COMPARE` reader - Timer Compare Value."]
pub type CompareR = crate::FieldReader<u32>;
#[doc = "Field `COMPARE` writer - Timer Compare Value."]
pub type CompareW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer Compare Value."]
    #[inline(always)]
    pub fn compare(&self) -> CompareR {
        CompareR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer Compare Value."]
    #[inline(always)]
    pub fn compare(&mut self) -> CompareW<CmpSpec> {
        CompareW::new(self, 0)
    }
}
#[doc = "Compare. This register stores the compare value, which is used to set the maximum count value to initiate a reload of the timer to 0x0001.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpSpec;
impl crate::RegisterSpec for CmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp::R`](R) reader structure"]
impl crate::Readable for CmpSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp::W`](W) writer structure"]
impl crate::Writable for CmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP to value 0xffff"]
impl crate::Resettable for CmpSpec {
    const RESET_VALUE: u32 = 0xffff;
}
