#[doc = "Register `ZERO_CROSS_CAL_D` reader"]
pub type R = crate::R<ZeroCrossCalDSpec>;
#[doc = "Field `ZXCALD` reader - Zero Cross Calibrartion Value VREGO_D"]
pub type ZxcaldR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Zero Cross Calibrartion Value VREGO_D"]
    #[inline(always)]
    pub fn zxcald(&self) -> ZxcaldR {
        ZxcaldR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Zero Cross Calibration VERGO_D Register\n\nYou can [`read`](crate::Reg::read) this register and get [`zero_cross_cal_d::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZeroCrossCalDSpec;
impl crate::RegisterSpec for ZeroCrossCalDSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`zero_cross_cal_d::R`](R) reader structure"]
impl crate::Readable for ZeroCrossCalDSpec {}
#[doc = "`reset()` method sets ZERO_CROSS_CAL_D to value 0"]
impl crate::Resettable for ZeroCrossCalDSpec {
    const RESET_VALUE: u32 = 0;
}
