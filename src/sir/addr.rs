#[doc = "Register `ADDR` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Field `ERRADDR` reader - "]
pub type ErraddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn erraddr(&self) -> ErraddrR {
        ErraddrR::new(self.bits)
    }
}
#[doc = "Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory. Contains the failing address in OTP memory (when CRCERR equals 1).\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for AddrSpec {}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for AddrSpec {
    const RESET_VALUE: u32 = 0;
}
