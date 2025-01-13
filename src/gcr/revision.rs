#[doc = "Register `REVISION` reader"]
pub type R = crate::R<RevisionSpec>;
#[doc = "Field `REVISION` reader - Manufacturer Chip Revision."]
pub type RevisionR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Manufacturer Chip Revision."]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Revision Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`revision::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RevisionSpec;
impl crate::RegisterSpec for RevisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`revision::R`](R) reader structure"]
impl crate::Readable for RevisionSpec {}
#[doc = "`reset()` method sets REVISION to value 0"]
impl crate::Resettable for RevisionSpec {
    const RESET_VALUE: u32 = 0;
}
