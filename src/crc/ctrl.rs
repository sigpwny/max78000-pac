#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EN` reader - CRC Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - CRC Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_EN` reader - DMA Request Enable"]
pub type DmaEnR = crate::BitReader;
#[doc = "Field `DMA_EN` writer - DMA Request Enable"]
pub type DmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSB` reader - MSB Select"]
pub type MsbR = crate::BitReader;
#[doc = "Field `MSB` writer - MSB Select"]
pub type MsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTE_SWAP_IN` reader - Byte Swap CRC Data Input"]
pub type ByteSwapInR = crate::BitReader;
#[doc = "Field `BYTE_SWAP_IN` writer - Byte Swap CRC Data Input"]
pub type ByteSwapInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTE_SWAP_OUT` reader - Byte Swap CRC Value Output"]
pub type ByteSwapOutR = crate::BitReader;
#[doc = "Field `BYTE_SWAP_OUT` writer - Byte Swap CRC Value Output"]
pub type ByteSwapOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - CRC Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - CRC Busy"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CRC Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Request Enable"]
    #[inline(always)]
    pub fn dma_en(&self) -> DmaEnR {
        DmaEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSB Select"]
    #[inline(always)]
    pub fn msb(&self) -> MsbR {
        MsbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Byte Swap CRC Data Input"]
    #[inline(always)]
    pub fn byte_swap_in(&self) -> ByteSwapInR {
        ByteSwapInR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Byte Swap CRC Value Output"]
    #[inline(always)]
    pub fn byte_swap_out(&self) -> ByteSwapOutR {
        ByteSwapOutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - CRC Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Request Enable"]
    #[inline(always)]
    pub fn dma_en(&mut self) -> DmaEnW<CtrlSpec> {
        DmaEnW::new(self, 1)
    }
    #[doc = "Bit 2 - MSB Select"]
    #[inline(always)]
    pub fn msb(&mut self) -> MsbW<CtrlSpec> {
        MsbW::new(self, 2)
    }
    #[doc = "Bit 3 - Byte Swap CRC Data Input"]
    #[inline(always)]
    pub fn byte_swap_in(&mut self) -> ByteSwapInW<CtrlSpec> {
        ByteSwapInW::new(self, 3)
    }
    #[doc = "Bit 4 - Byte Swap CRC Value Output"]
    #[inline(always)]
    pub fn byte_swap_out(&mut self) -> ByteSwapOutW<CtrlSpec> {
        ByteSwapOutW::new(self, 4)
    }
    #[doc = "Bit 16 - CRC Busy"]
    #[inline(always)]
    pub fn busy(&mut self) -> BusyW<CtrlSpec> {
        BusyW::new(self, 16)
    }
}
#[doc = "CRC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
