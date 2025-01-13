#[doc = "Register `FIFO_SIZE` reader"]
pub type R = crate::R<FifoSizeSpec>;
#[doc = "Register `FIFO_SIZE` writer"]
pub type W = crate::W<FifoSizeSpec>;
#[doc = "Field `fifo_size` reader - FIFO size."]
pub type FifoSizeR = crate::FieldReader;
#[doc = "Field `fifo_size` writer - FIFO size."]
pub type FifoSizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - FIFO size."]
    #[inline(always)]
    pub fn fifo_size(&self) -> FifoSizeR {
        FifoSizeR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FIFO size."]
    #[inline(always)]
    pub fn fifo_size(&mut self) -> FifoSizeW<FifoSizeSpec> {
        FifoSizeW::new(self, 0)
    }
}
#[doc = "FIFO Depth.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoSizeSpec;
impl crate::RegisterSpec for FifoSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_size::R`](R) reader structure"]
impl crate::Readable for FifoSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo_size::W`](W) writer structure"]
impl crate::Writable for FifoSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO_SIZE to value 0"]
impl crate::Resettable for FifoSizeSpec {
    const RESET_VALUE: u32 = 0;
}
