#[doc = "Register `AUTOCAL2` reader"]
pub type R = crate::R<Autocal2Spec>;
#[doc = "Register `AUTOCAL2` writer"]
pub type W = crate::W<Autocal2Spec>;
#[doc = "Field `DONECNT` reader - Auto-callibration Done Counter Setting."]
pub type DonecntR = crate::FieldReader;
#[doc = "Field `DONECNT` writer - Auto-callibration Done Counter Setting."]
pub type DonecntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ACDIV` reader - Auto-callibration Div Setting."]
pub type AcdivR = crate::FieldReader<u16>;
#[doc = "Field `ACDIV` writer - Auto-callibration Div Setting."]
pub type AcdivW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:7 - Auto-callibration Done Counter Setting."]
    #[inline(always)]
    pub fn donecnt(&self) -> DonecntR {
        DonecntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:20 - Auto-callibration Div Setting."]
    #[inline(always)]
    pub fn acdiv(&self) -> AcdivR {
        AcdivR::new(((self.bits >> 8) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Auto-callibration Done Counter Setting."]
    #[inline(always)]
    pub fn donecnt(&mut self) -> DonecntW<Autocal2Spec> {
        DonecntW::new(self, 0)
    }
    #[doc = "Bits 8:20 - Auto-callibration Div Setting."]
    #[inline(always)]
    pub fn acdiv(&mut self) -> AcdivW<Autocal2Spec> {
        AcdivW::new(self, 8)
    }
}
#[doc = "Automatic Calibration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`autocal2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocal2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Autocal2Spec;
impl crate::RegisterSpec for Autocal2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autocal2::R`](R) reader structure"]
impl crate::Readable for Autocal2Spec {}
#[doc = "`write(|w| ..)` method takes [`autocal2::W`](W) writer structure"]
impl crate::Writable for Autocal2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTOCAL2 to value 0"]
impl crate::Resettable for Autocal2Spec {
    const RESET_VALUE: u32 = 0;
}
