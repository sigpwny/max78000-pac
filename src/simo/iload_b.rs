#[doc = "Register `ILOAD_B` reader"]
pub type R = crate::R<IloadBSpec>;
#[doc = "Field `ILOADB` reader - Number of buck cycles that occur within the cycle clock"]
pub type IloadbR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of buck cycles that occur within the cycle clock"]
    #[inline(always)]
    pub fn iloadb(&self) -> IloadbR {
        IloadbR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Buck Cycle Count VREGO_B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iload_b::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IloadBSpec;
impl crate::RegisterSpec for IloadBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iload_b::R`](R) reader structure"]
impl crate::Readable for IloadBSpec {}
#[doc = "`reset()` method sets ILOAD_B to value 0"]
impl crate::Resettable for IloadBSpec {
    const RESET_VALUE: u32 = 0;
}
