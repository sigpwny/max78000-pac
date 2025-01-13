#[doc = "Register `SZ` reader"]
pub type R = crate::R<SzSpec>;
#[doc = "Field `CCH` reader - Cache Size. Indicates total size in Kbytes of cache."]
pub type CchR = crate::FieldReader<u16>;
#[doc = "Field `MEM` reader - Main Memory Size. Indicates the total size, in units of 128 Kbytes, of code memory accessible to the cache controller."]
pub type MemR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Cache Size. Indicates total size in Kbytes of cache."]
    #[inline(always)]
    pub fn cch(&self) -> CchR {
        CchR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Main Memory Size. Indicates the total size, in units of 128 Kbytes, of code memory accessible to the cache controller."]
    #[inline(always)]
    pub fn mem(&self) -> MemR {
        MemR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Memory Configuration Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sz::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SzSpec;
impl crate::RegisterSpec for SzSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sz::R`](R) reader structure"]
impl crate::Readable for SzSpec {}
#[doc = "`reset()` method sets SZ to value 0x0008_0008"]
impl crate::Resettable for SzSpec {
    const RESET_VALUE: u32 = 0x0008_0008;
}
