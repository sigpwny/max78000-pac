#[doc = "Register `SRC` reader"]
pub type R = crate::R<SrcSpec>;
#[doc = "Register `SRC` writer"]
pub type W = crate::W<SrcSpec>;
#[doc = "Field `ADDR` reader - "]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - "]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<SrcSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Source Device Address. If SRCINC=1, the counter bits are incremented by 1,2, or 4, depending on the data width of each AHB cycle. For peripheral transfers, some or all of the actual address bits are fixed. If SRCINC=0, this register remains constant. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with the contents of DMA_SRC_RLD.\n\nYou can [`read`](crate::Reg::read) this register and get [`src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcSpec;
impl crate::RegisterSpec for SrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src::R`](R) reader structure"]
impl crate::Readable for SrcSpec {}
#[doc = "`write(|w| ..)` method takes [`src::W`](W) writer structure"]
impl crate::Writable for SrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC to value 0"]
impl crate::Resettable for SrcSpec {
    const RESET_VALUE: u32 = 0;
}
