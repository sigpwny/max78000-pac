#[doc = "Register `TAP_SEL[%s]` reader"]
pub type R = crate::R<TapSelSpec>;
#[doc = "Register `TAP_SEL[%s]` writer"]
pub type W = crate::W<TapSelSpec>;
#[doc = "Field `LO` reader - Select delay line tap for lower bound of auto adjustment"]
pub type LoR = crate::FieldReader;
#[doc = "Field `LO` writer - Select delay line tap for lower bound of auto adjustment"]
pub type LoW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LO_TAP_STAT` reader - Returns last delay line tap value"]
pub type LoTapStatR = crate::BitReader;
#[doc = "Field `LO_TAP_STAT` writer - Returns last delay line tap value"]
pub type LoTapStatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR_TAP_STAT` reader - Returns last delay line tap value"]
pub type CtrTapStatR = crate::BitReader;
#[doc = "Field `CTR_TAP_STAT` writer - Returns last delay line tap value"]
pub type CtrTapStatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HI_TAP_STAT` reader - Returns last delay line tap value"]
pub type HiTapStatR = crate::BitReader;
#[doc = "Field `HI_TAP_STAT` writer - Returns last delay line tap value"]
pub type HiTapStatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HI` reader - Selects delay line tap for high point of auto adjustment"]
pub type HiR = crate::FieldReader;
#[doc = "Field `HI` writer - Selects delay line tap for high point of auto adjustment"]
pub type HiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CTR` reader - Selects delay line tap for center point of auto adjustment"]
pub type CtrR = crate::FieldReader;
#[doc = "Field `CTR` writer - Selects delay line tap for center point of auto adjustment"]
pub type CtrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `COARSE` reader - Selects delay line tap for coarse or fixed delay portion of the line"]
pub type CoarseR = crate::FieldReader;
#[doc = "Field `COARSE` writer - Selects delay line tap for coarse or fixed delay portion of the line"]
pub type CoarseW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DET_DLY` reader - Number of HCLK between delay line launch and sampling"]
pub type DetDlyR = crate::FieldReader;
#[doc = "Field `DET_DLY` writer - Number of HCLK between delay line launch and sampling"]
pub type DetDlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DELAY_ACT` reader - Set if the delay is active"]
pub type DelayActR = crate::BitReader;
#[doc = "Field `DELAY_ACT` writer - Set if the delay is active"]
pub type DelayActW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Select delay line tap for lower bound of auto adjustment"]
    #[inline(always)]
    pub fn lo(&self) -> LoR {
        LoR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Returns last delay line tap value"]
    #[inline(always)]
    pub fn lo_tap_stat(&self) -> LoTapStatR {
        LoTapStatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Returns last delay line tap value"]
    #[inline(always)]
    pub fn ctr_tap_stat(&self) -> CtrTapStatR {
        CtrTapStatR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Returns last delay line tap value"]
    #[inline(always)]
    pub fn hi_tap_stat(&self) -> HiTapStatR {
        HiTapStatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Selects delay line tap for high point of auto adjustment"]
    #[inline(always)]
    pub fn hi(&self) -> HiR {
        HiR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Selects delay line tap for center point of auto adjustment"]
    #[inline(always)]
    pub fn ctr(&self) -> CtrR {
        CtrR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Selects delay line tap for coarse or fixed delay portion of the line"]
    #[inline(always)]
    pub fn coarse(&self) -> CoarseR {
        CoarseR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 29:30 - Number of HCLK between delay line launch and sampling"]
    #[inline(always)]
    pub fn det_dly(&self) -> DetDlyR {
        DetDlyR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Set if the delay is active"]
    #[inline(always)]
    pub fn delay_act(&self) -> DelayActR {
        DelayActR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select delay line tap for lower bound of auto adjustment"]
    #[inline(always)]
    pub fn lo(&mut self) -> LoW<TapSelSpec> {
        LoW::new(self, 0)
    }
    #[doc = "Bit 5 - Returns last delay line tap value"]
    #[inline(always)]
    pub fn lo_tap_stat(&mut self) -> LoTapStatW<TapSelSpec> {
        LoTapStatW::new(self, 5)
    }
    #[doc = "Bit 6 - Returns last delay line tap value"]
    #[inline(always)]
    pub fn ctr_tap_stat(&mut self) -> CtrTapStatW<TapSelSpec> {
        CtrTapStatW::new(self, 6)
    }
    #[doc = "Bit 7 - Returns last delay line tap value"]
    #[inline(always)]
    pub fn hi_tap_stat(&mut self) -> HiTapStatW<TapSelSpec> {
        HiTapStatW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Selects delay line tap for high point of auto adjustment"]
    #[inline(always)]
    pub fn hi(&mut self) -> HiW<TapSelSpec> {
        HiW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Selects delay line tap for center point of auto adjustment"]
    #[inline(always)]
    pub fn ctr(&mut self) -> CtrW<TapSelSpec> {
        CtrW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Selects delay line tap for coarse or fixed delay portion of the line"]
    #[inline(always)]
    pub fn coarse(&mut self) -> CoarseW<TapSelSpec> {
        CoarseW::new(self, 24)
    }
    #[doc = "Bits 29:30 - Number of HCLK between delay line launch and sampling"]
    #[inline(always)]
    pub fn det_dly(&mut self) -> DetDlyW<TapSelSpec> {
        DetDlyW::new(self, 29)
    }
    #[doc = "Bit 31 - Set if the delay is active"]
    #[inline(always)]
    pub fn delay_act(&mut self) -> DelayActW<TapSelSpec> {
        DelayActW::new(self, 31)
    }
}
#[doc = "DVS Tap Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tap_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tap_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TapSelSpec;
impl crate::RegisterSpec for TapSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tap_sel::R`](R) reader structure"]
impl crate::Readable for TapSelSpec {}
#[doc = "`write(|w| ..)` method takes [`tap_sel::W`](W) writer structure"]
impl crate::Writable for TapSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAP_SEL[%s]
to value 0"]
impl crate::Resettable for TapSelSpec {
    const RESET_VALUE: u32 = 0;
}
