#[doc = "Register `ILOAD_D` reader"]
pub type R = crate::R<IloadDSpec>;
#[doc = "Field `ILOADD` reader - Number of buck cycles that occur within the cycle clock"]
pub type IloaddR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of buck cycles that occur within the cycle clock"]
    #[inline(always)]
    pub fn iloadd(&self) -> IloaddR {
        IloaddR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Buck Cycle Count VREGO_D Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iload_d::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IloadDSpec;
impl crate::RegisterSpec for IloadDSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iload_d::R`](R) reader structure"]
impl crate::Readable for IloadDSpec {}
#[doc = "`reset()` method sets ILOAD_D to value 0"]
impl crate::Resettable for IloadDSpec {
    const RESET_VALUE: u32 = 0;
}
