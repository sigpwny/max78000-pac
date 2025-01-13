#[doc = "Register `AUTOCAL1` reader"]
pub type R = crate::R<Autocal1Spec>;
#[doc = "Register `AUTOCAL1` writer"]
pub type W = crate::W<Autocal1Spec>;
#[doc = "Field `INITTRM` reader - Initial Trim Setting."]
pub type InittrmR = crate::FieldReader<u16>;
#[doc = "Field `INITTRM` writer - Initial Trim Setting."]
pub type InittrmW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Initial Trim Setting."]
    #[inline(always)]
    pub fn inittrm(&self) -> InittrmR {
        InittrmR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Initial Trim Setting."]
    #[inline(always)]
    pub fn inittrm(&mut self) -> InittrmW<Autocal1Spec> {
        InittrmW::new(self, 0)
    }
}
#[doc = "Automatic Calibration 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`autocal1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocal1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Autocal1Spec;
impl crate::RegisterSpec for Autocal1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autocal1::R`](R) reader structure"]
impl crate::Readable for Autocal1Spec {}
#[doc = "`write(|w| ..)` method takes [`autocal1::W`](W) writer structure"]
impl crate::Writable for Autocal1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTOCAL1 to value 0"]
impl crate::Resettable for Autocal1Spec {
    const RESET_VALUE: u32 = 0;
}
