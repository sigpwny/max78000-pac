#[doc = "Register `DST` reader"]
pub type R = crate::R<DstSpec>;
#[doc = "Register `DST` writer"]
pub type W = crate::W<DstSpec>;
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
    pub fn addr(&mut self) -> AddrW<DstSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Destination Device Address. For peripheral transfers, some or all of the actual address bits are fixed. If DSTINC=1, this register is incremented on every AHB write out of the DMA FIFO. They are incremented by 1, 2, or 4, depending on the data width of each AHB cycle. In the case where a count-to-zero condition occurs while RLDEN=1, the register is reloaded with DMA_DST_RLD.\n\nYou can [`read`](crate::Reg::read) this register and get [`dst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstSpec;
impl crate::RegisterSpec for DstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst::R`](R) reader structure"]
impl crate::Readable for DstSpec {}
#[doc = "`write(|w| ..)` method takes [`dst::W`](W) writer structure"]
impl crate::Writable for DstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST to value 0"]
impl crate::Resettable for DstSpec {
    const RESET_VALUE: u32 = 0;
}
