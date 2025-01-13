#[doc = "Register `ILOAD_C` reader"]
pub type R = crate::R<IloadCSpec>;
#[doc = "Field `ILOADC` reader - Number of buck cycles that occur within the cycle clock"]
pub type IloadcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of buck cycles that occur within the cycle clock"]
    #[inline(always)]
    pub fn iloadc(&self) -> IloadcR {
        IloadcR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Buck Cycle Count VREGO_C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iload_c::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IloadCSpec;
impl crate::RegisterSpec for IloadCSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iload_c::R`](R) reader structure"]
impl crate::Readable for IloadCSpec {}
#[doc = "`reset()` method sets ILOAD_C to value 0"]
impl crate::Resettable for IloadCSpec {
    const RESET_VALUE: u32 = 0;
}
