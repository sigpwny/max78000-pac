#[doc = "Register `BUCK_ALERT_THR_A` reader"]
pub type R = crate::R<BuckAlertThrASpec>;
#[doc = "Register `BUCK_ALERT_THR_A` writer"]
pub type W = crate::W<BuckAlertThrASpec>;
#[doc = "Field `BUCKTHRA` reader - Threshold for ILOADA to generate the BUCK_ALERT"]
pub type BuckthraR = crate::FieldReader;
#[doc = "Field `BUCKTHRA` writer - Threshold for ILOADA to generate the BUCK_ALERT"]
pub type BuckthraW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Threshold for ILOADA to generate the BUCK_ALERT"]
    #[inline(always)]
    pub fn buckthra(&self) -> BuckthraR {
        BuckthraR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Threshold for ILOADA to generate the BUCK_ALERT"]
    #[inline(always)]
    pub fn buckthra(&mut self) -> BuckthraW<BuckAlertThrASpec> {
        BuckthraW::new(self, 0)
    }
}
#[doc = "Buck Cycle Count Alert VERGO_A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`buck_alert_thr_a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buck_alert_thr_a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BuckAlertThrASpec;
impl crate::RegisterSpec for BuckAlertThrASpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buck_alert_thr_a::R`](R) reader structure"]
impl crate::Readable for BuckAlertThrASpec {}
#[doc = "`write(|w| ..)` method takes [`buck_alert_thr_a::W`](W) writer structure"]
impl crate::Writable for BuckAlertThrASpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUCK_ALERT_THR_A to value 0"]
impl crate::Resettable for BuckAlertThrASpec {
    const RESET_VALUE: u32 = 0;
}
