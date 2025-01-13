#[doc = "Register `MAXTON` reader"]
pub type R = crate::R<MaxtonSpec>;
#[doc = "Register `MAXTON` writer"]
pub type W = crate::W<MaxtonSpec>;
#[doc = "Field `TONSET` reader - Sets the maximum on time for the high side FET, each increment represents 500ns"]
pub type TonsetR = crate::FieldReader;
#[doc = "Field `TONSET` writer - Sets the maximum on time for the high side FET, each increment represents 500ns"]
pub type TonsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Sets the maximum on time for the high side FET, each increment represents 500ns"]
    #[inline(always)]
    pub fn tonset(&self) -> TonsetR {
        TonsetR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sets the maximum on time for the high side FET, each increment represents 500ns"]
    #[inline(always)]
    pub fn tonset(&mut self) -> TonsetW<MaxtonSpec> {
        TonsetW::new(self, 0)
    }
}
#[doc = "Maximum High Side FET Time On Register\n\nYou can [`read`](crate::Reg::read) this register and get [`maxton::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxton::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxtonSpec;
impl crate::RegisterSpec for MaxtonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxton::R`](R) reader structure"]
impl crate::Readable for MaxtonSpec {}
#[doc = "`write(|w| ..)` method takes [`maxton::W`](W) writer structure"]
impl crate::Writable for MaxtonSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAXTON to value 0"]
impl crate::Resettable for MaxtonSpec {
    const RESET_VALUE: u32 = 0;
}
