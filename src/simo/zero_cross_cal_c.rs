#[doc = "Register `ZERO_CROSS_CAL_C` reader"]
pub type R = crate::R<ZeroCrossCalCSpec>;
#[doc = "Field `ZXCALC` reader - Zero Cross Calibrartion Value VREGO_C"]
pub type ZxcalcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Zero Cross Calibrartion Value VREGO_C"]
    #[inline(always)]
    pub fn zxcalc(&self) -> ZxcalcR {
        ZxcalcR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Zero Cross Calibration VERGO_C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`zero_cross_cal_c::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZeroCrossCalCSpec;
impl crate::RegisterSpec for ZeroCrossCalCSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`zero_cross_cal_c::R`](R) reader structure"]
impl crate::Readable for ZeroCrossCalCSpec {}
#[doc = "`reset()` method sets ZERO_CROSS_CAL_C to value 0"]
impl crate::Resettable for ZeroCrossCalCSpec {
    const RESET_VALUE: u32 = 0;
}
