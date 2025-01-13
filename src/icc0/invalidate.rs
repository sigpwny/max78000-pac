#[doc = "Register `INVALIDATE` reader"]
pub type R = crate::R<InvalidateSpec>;
#[doc = "Register `INVALIDATE` writer"]
pub type W = crate::W<InvalidateSpec>;
#[doc = "Field `INVALID` reader - Invalidate."]
pub type InvalidR = crate::FieldReader<u32>;
#[doc = "Field `INVALID` writer - Invalidate."]
pub type InvalidW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Invalidate."]
    #[inline(always)]
    pub fn invalid(&self) -> InvalidR {
        InvalidR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Invalidate."]
    #[inline(always)]
    pub fn invalid(&mut self) -> InvalidW<InvalidateSpec> {
        InvalidW::new(self, 0)
    }
}
#[doc = "Invalidate All Registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`invalidate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`invalidate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InvalidateSpec;
impl crate::RegisterSpec for InvalidateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`invalidate::R`](R) reader structure"]
impl crate::Readable for InvalidateSpec {}
#[doc = "`write(|w| ..)` method takes [`invalidate::W`](W) writer structure"]
impl crate::Writable for InvalidateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INVALIDATE to value 0"]
impl crate::Resettable for InvalidateSpec {
    const RESET_VALUE: u32 = 0;
}
