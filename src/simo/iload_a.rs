#[doc = "Register `ILOAD_A` reader"]
pub type R = crate::R<IloadASpec>;
#[doc = "Field `ILOADA` reader - Number of buck cycles that occur within the cycle clock"]
pub type IloadaR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of buck cycles that occur within the cycle clock"]
    #[inline(always)]
    pub fn iloada(&self) -> IloadaR {
        IloadaR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Buck Cycle Count VREGO_A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iload_a::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IloadASpec;
impl crate::RegisterSpec for IloadASpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iload_a::R`](R) reader structure"]
impl crate::Readable for IloadASpec {}
#[doc = "`reset()` method sets ILOAD_A to value 0"]
impl crate::Resettable for IloadASpec {
    const RESET_VALUE: u32 = 0;
}
