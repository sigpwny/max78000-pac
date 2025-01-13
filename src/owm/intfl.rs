#[doc = "Register `INTFL` reader"]
pub type R = crate::R<IntflSpec>;
#[doc = "Register `INTFL` writer"]
pub type W = crate::W<IntflSpec>;
#[doc = "Field `ow_reset_done` reader - OW Reset Sequence Completed."]
pub type OwResetDoneR = crate::BitReader;
#[doc = "Field `ow_reset_done` writer - OW Reset Sequence Completed."]
pub type OwResetDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_data_empty` reader - TX Data Empty Interrupt Flag."]
pub type TxDataEmptyR = crate::BitReader;
#[doc = "Field `tx_data_empty` writer - TX Data Empty Interrupt Flag."]
pub type TxDataEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_data_ready` reader - RX Data Ready Interrupt Flag"]
pub type RxDataReadyR = crate::BitReader;
#[doc = "Field `rx_data_ready` writer - RX Data Ready Interrupt Flag"]
pub type RxDataReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `line_short` reader - OW Line Short Detected Interrupt Flag."]
pub type LineShortR = crate::BitReader;
#[doc = "Field `line_short` writer - OW Line Short Detected Interrupt Flag."]
pub type LineShortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `line_low` reader - OW Line Low Detected Interrupt Flag."]
pub type LineLowR = crate::BitReader;
#[doc = "Field `line_low` writer - OW Line Low Detected Interrupt Flag."]
pub type LineLowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OW Reset Sequence Completed."]
    #[inline(always)]
    pub fn ow_reset_done(&self) -> OwResetDoneR {
        OwResetDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Data Empty Interrupt Flag."]
    #[inline(always)]
    pub fn tx_data_empty(&self) -> TxDataEmptyR {
        TxDataEmptyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Data Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rx_data_ready(&self) -> RxDataReadyR {
        RxDataReadyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OW Line Short Detected Interrupt Flag."]
    #[inline(always)]
    pub fn line_short(&self) -> LineShortR {
        LineShortR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OW Line Low Detected Interrupt Flag."]
    #[inline(always)]
    pub fn line_low(&self) -> LineLowR {
        LineLowR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OW Reset Sequence Completed."]
    #[inline(always)]
    pub fn ow_reset_done(&mut self) -> OwResetDoneW<IntflSpec> {
        OwResetDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - TX Data Empty Interrupt Flag."]
    #[inline(always)]
    pub fn tx_data_empty(&mut self) -> TxDataEmptyW<IntflSpec> {
        TxDataEmptyW::new(self, 1)
    }
    #[doc = "Bit 2 - RX Data Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rx_data_ready(&mut self) -> RxDataReadyW<IntflSpec> {
        RxDataReadyW::new(self, 2)
    }
    #[doc = "Bit 3 - OW Line Short Detected Interrupt Flag."]
    #[inline(always)]
    pub fn line_short(&mut self) -> LineShortW<IntflSpec> {
        LineShortW::new(self, 3)
    }
    #[doc = "Bit 4 - OW Line Low Detected Interrupt Flag."]
    #[inline(always)]
    pub fn line_low(&mut self) -> LineLowW<IntflSpec> {
        LineLowW::new(self, 4)
    }
}
#[doc = "1-Wire Master Interrupt Flags.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflSpec;
impl crate::RegisterSpec for IntflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl::R`](R) reader structure"]
impl crate::Readable for IntflSpec {}
#[doc = "`write(|w| ..)` method takes [`intfl::W`](W) writer structure"]
impl crate::Writable for IntflSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for IntflSpec {
    const RESET_VALUE: u32 = 0;
}
