#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Field `DATA` reader - Data. The content of this register is valid only when RNG_IS =1. When TNRG is disabled, read returns 0x0000 0000."]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data. The content of this register is valid only when RNG_IS =1. When TNRG is disabled, read returns 0x0000 0000."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000.\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DataSpec {
    const RESET_VALUE: u32 = 0;
}
