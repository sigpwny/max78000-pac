#[doc = "Register `DS_TIMING_CODES` reader"]
pub type R = crate::R<DsTimingCodesSpec>;
#[doc = "Register `DS_TIMING_CODES` writer"]
pub type W = crate::W<DsTimingCodesSpec>;
#[doc = "Field `SAV` reader - Start Active Video Code."]
pub type SavR = crate::FieldReader;
#[doc = "Field `SAV` writer - Start Active Video Code."]
pub type SavW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EAV` reader - End Active Video Code."]
pub type EavR = crate::FieldReader;
#[doc = "Field `EAV` writer - End Active Video Code."]
pub type EavW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Start Active Video Code."]
    #[inline(always)]
    pub fn sav(&self) -> SavR {
        SavR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - End Active Video Code."]
    #[inline(always)]
    pub fn eav(&self) -> EavR {
        EavR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start Active Video Code."]
    #[inline(always)]
    pub fn sav(&mut self) -> SavW<DsTimingCodesSpec> {
        SavW::new(self, 0)
    }
    #[doc = "Bits 8:15 - End Active Video Code."]
    #[inline(always)]
    pub fn eav(&mut self) -> EavW<DsTimingCodesSpec> {
        EavW::new(self, 8)
    }
}
#[doc = "DS Timing Code Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ds_timing_codes::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ds_timing_codes::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsTimingCodesSpec;
impl crate::RegisterSpec for DsTimingCodesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ds_timing_codes::R`](R) reader structure"]
impl crate::Readable for DsTimingCodesSpec {}
#[doc = "`write(|w| ..)` method takes [`ds_timing_codes::W`](W) writer structure"]
impl crate::Writable for DsTimingCodesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DS_TIMING_CODES to value 0"]
impl crate::Resettable for DsTimingCodesSpec {
    const RESET_VALUE: u32 = 0;
}
