#[doc = "Register `ZERO_CROSS_CAL_A` reader"]
pub type R = crate::R<ZeroCrossCalASpec>;
#[doc = "Field `ZXCALA` reader - Zero Cross Calibrartion Value VREGO_A"]
pub type ZxcalaR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Zero Cross Calibrartion Value VREGO_A"]
    #[inline(always)]
    pub fn zxcala(&self) -> ZxcalaR {
        ZxcalaR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Zero Cross Calibration VERGO_A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`zero_cross_cal_a::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZeroCrossCalASpec;
impl crate::RegisterSpec for ZeroCrossCalASpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`zero_cross_cal_a::R`](R) reader structure"]
impl crate::Readable for ZeroCrossCalASpec {}
#[doc = "`reset()` method sets ZERO_CROSS_CAL_A to value 0"]
impl crate::Resettable for ZeroCrossCalASpec {
    const RESET_VALUE: u32 = 0;
}
