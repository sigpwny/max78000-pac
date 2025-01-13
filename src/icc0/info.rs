#[doc = "Register `INFO` reader"]
pub type R = crate::R<InfoSpec>;
#[doc = "Field `RELNUM` reader - Release Number. Identifies the RTL release version."]
pub type RelnumR = crate::FieldReader;
#[doc = "Field `PARTNUM` reader - Part Number. This field reflects the value of C_ID_PART_NUMBER configuration parameter."]
pub type PartnumR = crate::FieldReader;
#[doc = "Field `ID` reader - Cache ID. This field reflects the value of the C_ID_CACHEID configuration parameter."]
pub type IdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Release Number. Identifies the RTL release version."]
    #[inline(always)]
    pub fn relnum(&self) -> RelnumR {
        RelnumR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - Part Number. This field reflects the value of C_ID_PART_NUMBER configuration parameter."]
    #[inline(always)]
    pub fn partnum(&self) -> PartnumR {
        PartnumR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:15 - Cache ID. This field reflects the value of the C_ID_CACHEID configuration parameter."]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
#[doc = "Cache ID Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InfoSpec;
impl crate::RegisterSpec for InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`info::R`](R) reader structure"]
impl crate::Readable for InfoSpec {}
#[doc = "`reset()` method sets INFO to value 0"]
impl crate::Resettable for InfoSpec {
    const RESET_VALUE: u32 = 0;
}
