#[doc = "Register `DSTRLD` reader"]
pub type R = crate::R<DstrldSpec>;
#[doc = "Register `DSTRLD` writer"]
pub type W = crate::W<DstrldSpec>;
#[doc = "Field `ADDR` reader - Destination Address Reload Value."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Destination Address Reload Value."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - Destination Address Reload Value."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - Destination Address Reload Value."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<DstrldSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Destination Address Reload Value. The value of this register is loaded into DMA0_DST upon a count-to-zero condition.\n\nYou can [`read`](crate::Reg::read) this register and get [`dstrld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstrld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstrldSpec;
impl crate::RegisterSpec for DstrldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstrld::R`](R) reader structure"]
impl crate::Readable for DstrldSpec {}
#[doc = "`write(|w| ..)` method takes [`dstrld::W`](W) writer structure"]
impl crate::Writable for DstrldSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSTRLD to value 0"]
impl crate::Resettable for DstrldSpec {
    const RESET_VALUE: u32 = 0;
}
