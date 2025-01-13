#[doc = "Register `BUCK_ALERT_THR_C` reader"]
pub type R = crate::R<BuckAlertThrCSpec>;
#[doc = "Register `BUCK_ALERT_THR_C` writer"]
pub type W = crate::W<BuckAlertThrCSpec>;
#[doc = "Field `BUCKTHRC` reader - Threshold for ILOADC to generate the BUCK_ALERT"]
pub type BuckthrcR = crate::FieldReader;
#[doc = "Field `BUCKTHRC` writer - Threshold for ILOADC to generate the BUCK_ALERT"]
pub type BuckthrcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Threshold for ILOADC to generate the BUCK_ALERT"]
    #[inline(always)]
    pub fn buckthrc(&self) -> BuckthrcR {
        BuckthrcR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Threshold for ILOADC to generate the BUCK_ALERT"]
    #[inline(always)]
    pub fn buckthrc(&mut self) -> BuckthrcW<BuckAlertThrCSpec> {
        BuckthrcW::new(self, 0)
    }
}
#[doc = "Buck Cycle Count Alert VERGO_C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`buck_alert_thr_c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buck_alert_thr_c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BuckAlertThrCSpec;
impl crate::RegisterSpec for BuckAlertThrCSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buck_alert_thr_c::R`](R) reader structure"]
impl crate::Readable for BuckAlertThrCSpec {}
#[doc = "`write(|w| ..)` method takes [`buck_alert_thr_c::W`](W) writer structure"]
impl crate::Writable for BuckAlertThrCSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUCK_ALERT_THR_C to value 0"]
impl crate::Resettable for BuckAlertThrCSpec {
    const RESET_VALUE: u32 = 0;
}
