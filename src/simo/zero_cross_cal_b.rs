#[doc = "Register `ZERO_CROSS_CAL_B` reader"]
pub type R = crate::R<ZeroCrossCalBSpec>;
#[doc = "Field `ZXCALB` reader - Zero Cross Calibrartion Value VREGO_B"]
pub type ZxcalbR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Zero Cross Calibrartion Value VREGO_B"]
    #[inline(always)]
    pub fn zxcalb(&self) -> ZxcalbR {
        ZxcalbR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Zero Cross Calibration VERGO_B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`zero_cross_cal_b::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZeroCrossCalBSpec;
impl crate::RegisterSpec for ZeroCrossCalBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`zero_cross_cal_b::R`](R) reader structure"]
impl crate::Readable for ZeroCrossCalBSpec {}
#[doc = "`reset()` method sets ZERO_CROSS_CAL_B to value 0"]
impl crate::Resettable for ZeroCrossCalBSpec {
    const RESET_VALUE: u32 = 0;
}
