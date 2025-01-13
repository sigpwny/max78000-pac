#[doc = "Register `SRCRLD` reader"]
pub type R = crate::R<SrcrldSpec>;
#[doc = "Register `SRCRLD` writer"]
pub type W = crate::W<SrcrldSpec>;
#[doc = "Field `ADDR` reader - Source Address Reload Value."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Source Address Reload Value."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - Source Address Reload Value."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - Source Address Reload Value."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<SrcrldSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Source Address Reload Value. The value of this register is loaded into DMA0_SRC upon a count-to-zero condition.\n\nYou can [`read`](crate::Reg::read) this register and get [`srcrld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcrld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcrldSpec;
impl crate::RegisterSpec for SrcrldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcrld::R`](R) reader structure"]
impl crate::Readable for SrcrldSpec {}
#[doc = "`write(|w| ..)` method takes [`srcrld::W`](W) writer structure"]
impl crate::Writable for SrcrldSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCRLD to value 0"]
impl crate::Resettable for SrcrldSpec {
    const RESET_VALUE: u32 = 0;
}
