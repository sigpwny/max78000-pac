#[doc = "Register `IPOLO` reader"]
pub type R = crate::R<IpoloSpec>;
#[doc = "Field `IPO_LIMITLO` reader - IPO Low Limit Trim."]
pub type IpoLimitloR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - IPO Low Limit Trim."]
    #[inline(always)]
    pub fn ipo_limitlo(&self) -> IpoLimitloR {
        IpoLimitloR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "IPO Low Trim System Initialization Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipolo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpoloSpec;
impl crate::RegisterSpec for IpoloSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipolo::R`](R) reader structure"]
impl crate::Readable for IpoloSpec {}
#[doc = "`reset()` method sets IPOLO to value 0"]
impl crate::Resettable for IpoloSpec {
    const RESET_VALUE: u32 = 0;
}
