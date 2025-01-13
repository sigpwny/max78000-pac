#[doc = "Register `BUCK_ALERT_THR_B` reader"]
pub type R = crate::R<BuckAlertThrBSpec>;
#[doc = "Register `BUCK_ALERT_THR_B` writer"]
pub type W = crate::W<BuckAlertThrBSpec>;
#[doc = "Field `BUCKTHRB` reader - Threshold for ILOADB to generate the BUCK_ALERT"]
pub type BuckthrbR = crate::FieldReader;
#[doc = "Field `BUCKTHRB` writer - Threshold for ILOADB to generate the BUCK_ALERT"]
pub type BuckthrbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Threshold for ILOADB to generate the BUCK_ALERT"]
    #[inline(always)]
    pub fn buckthrb(&self) -> BuckthrbR {
        BuckthrbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Threshold for ILOADB to generate the BUCK_ALERT"]
    #[inline(always)]
    pub fn buckthrb(&mut self) -> BuckthrbW<BuckAlertThrBSpec> {
        BuckthrbW::new(self, 0)
    }
}
#[doc = "Buck Cycle Count Alert VERGO_B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`buck_alert_thr_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buck_alert_thr_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BuckAlertThrBSpec;
impl crate::RegisterSpec for BuckAlertThrBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buck_alert_thr_b::R`](R) reader structure"]
impl crate::Readable for BuckAlertThrBSpec {}
#[doc = "`write(|w| ..)` method takes [`buck_alert_thr_b::W`](W) writer structure"]
impl crate::Writable for BuckAlertThrBSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUCK_ALERT_THR_B to value 0"]
impl crate::Resettable for BuckAlertThrBSpec {
    const RESET_VALUE: u32 = 0;
}
