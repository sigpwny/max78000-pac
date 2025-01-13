#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `active` reader - ADC Conversion In Progress"]
pub type ActiveR = crate::BitReader;
#[doc = "Field `afe_pwr_up_active` reader - AFE Power Up Delay Active"]
pub type AfePwrUpActiveR = crate::BitReader;
#[doc = "Field `overflow` reader - ADC Overflow"]
pub type OverflowR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ADC Conversion In Progress"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - AFE Power Up Delay Active"]
    #[inline(always)]
    pub fn afe_pwr_up_active(&self) -> AfePwrUpActiveR {
        AfePwrUpActiveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Overflow"]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {}
#[doc = "ADC Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
