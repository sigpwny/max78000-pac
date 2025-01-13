#[doc = "Register `DMA` reader"]
pub type R = crate::R<DmaSpec>;
#[doc = "Register `DMA` writer"]
pub type W = crate::W<DmaSpec>;
#[doc = "TX channel enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxEn {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<TxEn> for bool {
    #[inline(always)]
    fn from(variant: TxEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_EN` reader - TX channel enable."]
pub type TxEnR = crate::BitReader<TxEn>;
impl TxEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxEn {
        match self.bits {
            false => TxEn::Dis,
            true => TxEn::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TxEn::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TxEn::En
    }
}
#[doc = "Field `TX_EN` writer - TX channel enable."]
pub type TxEnW<'a, REG> = crate::BitWriter<'a, REG, TxEn>;
impl<'a, REG> TxEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TxEn::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TxEn::En)
    }
}
#[doc = "RX channel enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxEn {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<RxEn> for bool {
    #[inline(always)]
    fn from(variant: RxEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_EN` reader - RX channel enable."]
pub type RxEnR = crate::BitReader<RxEn>;
impl RxEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxEn {
        match self.bits {
            false => RxEn::Dis,
            true => RxEn::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RxEn::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RxEn::En
    }
}
#[doc = "Field `RX_EN` writer - RX channel enable."]
pub type RxEnW<'a, REG> = crate::BitWriter<'a, REG, RxEn>;
impl<'a, REG> RxEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RxEn::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RxEn::En)
    }
}
impl R {
    #[doc = "Bit 0 - TX channel enable."]
    #[inline(always)]
    pub fn tx_en(&self) -> TxEnR {
        TxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX channel enable."]
    #[inline(always)]
    pub fn rx_en(&self) -> RxEnR {
        RxEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX channel enable."]
    #[inline(always)]
    pub fn tx_en(&mut self) -> TxEnW<DmaSpec> {
        TxEnW::new(self, 0)
    }
    #[doc = "Bit 1 - RX channel enable."]
    #[inline(always)]
    pub fn rx_en(&mut self) -> RxEnW<DmaSpec> {
        RxEnW::new(self, 1)
    }
}
#[doc = "DMA Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSpec;
impl crate::RegisterSpec for DmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma::R`](R) reader structure"]
impl crate::Readable for DmaSpec {}
#[doc = "`write(|w| ..)` method takes [`dma::W`](W) writer structure"]
impl crate::Writable for DmaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DmaSpec {
    const RESET_VALUE: u32 = 0;
}
