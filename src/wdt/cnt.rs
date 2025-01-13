#[doc = "Register `CNT` reader"]
pub type R = crate::R<CntSpec>;
#[doc = "Field `COUNT` reader - Current Value of the Windowed Watchdog Timer Counter."]
pub type CountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current Value of the Windowed Watchdog Timer Counter."]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new(self.bits)
    }
}
#[doc = "Windowed Watchdog Timer Count Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntSpec;
impl crate::RegisterSpec for CntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CntSpec {}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CntSpec {
    const RESET_VALUE: u32 = 0;
}
