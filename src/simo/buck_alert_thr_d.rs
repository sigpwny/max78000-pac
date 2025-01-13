#[doc = "Register `BUCK_ALERT_THR_D` reader"]
pub type R = crate::R<BuckAlertThrDSpec>;
#[doc = "Register `BUCK_ALERT_THR_D` writer"]
pub type W = crate::W<BuckAlertThrDSpec>;
#[doc = "Field `BUCKTHRD` reader - Threshold for ILOADD to generate the BUCK_ALERT"]
pub type BuckthrdR = crate::FieldReader;
#[doc = "Field `BUCKTHRD` writer - Threshold for ILOADD to generate the BUCK_ALERT"]
pub type BuckthrdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Threshold for ILOADD to generate the BUCK_ALERT"]
    #[inline(always)]
    pub fn buckthrd(&self) -> BuckthrdR {
        BuckthrdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Threshold for ILOADD to generate the BUCK_ALERT"]
    #[inline(always)]
    pub fn buckthrd(&mut self) -> BuckthrdW<BuckAlertThrDSpec> {
        BuckthrdW::new(self, 0)
    }
}
#[doc = "Buck Cycle Count Alert VERGO_D Register\n\nYou can [`read`](crate::Reg::read) this register and get [`buck_alert_thr_d::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buck_alert_thr_d::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BuckAlertThrDSpec;
impl crate::RegisterSpec for BuckAlertThrDSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buck_alert_thr_d::R`](R) reader structure"]
impl crate::Readable for BuckAlertThrDSpec {}
#[doc = "`write(|w| ..)` method takes [`buck_alert_thr_d::W`](W) writer structure"]
impl crate::Writable for BuckAlertThrDSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUCK_ALERT_THR_D to value 0"]
impl crate::Resettable for BuckAlertThrDSpec {
    const RESET_VALUE: u32 = 0;
}
