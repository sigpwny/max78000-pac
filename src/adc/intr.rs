#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `done_ie` reader - ADC Done Interrupt Enable"]
pub type DoneIeR = crate::BitReader;
#[doc = "Field `done_ie` writer - ADC Done Interrupt Enable"]
pub type DoneIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ref_ready_ie` reader - ADC Reference Ready Interrupt Enable"]
pub type RefReadyIeR = crate::BitReader;
#[doc = "Field `ref_ready_ie` writer - ADC Reference Ready Interrupt Enable"]
pub type RefReadyIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hi_limit_ie` reader - ADC Hi Limit Monitor Interrupt Enable"]
pub type HiLimitIeR = crate::BitReader;
#[doc = "Field `hi_limit_ie` writer - ADC Hi Limit Monitor Interrupt Enable"]
pub type HiLimitIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lo_limit_ie` reader - ADC Lo Limit Monitor Interrupt Enable"]
pub type LoLimitIeR = crate::BitReader;
#[doc = "Field `lo_limit_ie` writer - ADC Lo Limit Monitor Interrupt Enable"]
pub type LoLimitIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `overflow_ie` reader - ADC Overflow Interrupt Enable"]
pub type OverflowIeR = crate::BitReader;
#[doc = "Field `overflow_ie` writer - ADC Overflow Interrupt Enable"]
pub type OverflowIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `done_if` reader - ADC Done Interrupt Flag"]
pub type DoneIfR = crate::BitReader;
#[doc = "Field `done_if` writer - ADC Done Interrupt Flag"]
pub type DoneIfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ref_ready_if` reader - ADC Reference Ready Interrupt Flag"]
pub type RefReadyIfR = crate::BitReader;
#[doc = "Field `ref_ready_if` writer - ADC Reference Ready Interrupt Flag"]
pub type RefReadyIfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `hi_limit_if` reader - ADC Hi Limit Monitor Interrupt Flag"]
pub type HiLimitIfR = crate::BitReader;
#[doc = "Field `hi_limit_if` writer - ADC Hi Limit Monitor Interrupt Flag"]
pub type HiLimitIfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `lo_limit_if` reader - ADC Lo Limit Monitor Interrupt Flag"]
pub type LoLimitIfR = crate::BitReader;
#[doc = "Field `lo_limit_if` writer - ADC Lo Limit Monitor Interrupt Flag"]
pub type LoLimitIfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `overflow_if` reader - ADC Overflow Interrupt Flag"]
pub type OverflowIfR = crate::BitReader;
#[doc = "Field `overflow_if` writer - ADC Overflow Interrupt Flag"]
pub type OverflowIfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `pending` reader - ADC Interrupt Pending Status"]
pub type PendingR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ADC Done Interrupt Enable"]
    #[inline(always)]
    pub fn done_ie(&self) -> DoneIeR {
        DoneIeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Reference Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ref_ready_ie(&self) -> RefReadyIeR {
        RefReadyIeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Hi Limit Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn hi_limit_ie(&self) -> HiLimitIeR {
        HiLimitIeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Lo Limit Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn lo_limit_ie(&self) -> LoLimitIeR {
        LoLimitIeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn overflow_ie(&self) -> OverflowIeR {
        OverflowIeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC Done Interrupt Flag"]
    #[inline(always)]
    pub fn done_if(&self) -> DoneIfR {
        DoneIfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC Reference Ready Interrupt Flag"]
    #[inline(always)]
    pub fn ref_ready_if(&self) -> RefReadyIfR {
        RefReadyIfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC Hi Limit Monitor Interrupt Flag"]
    #[inline(always)]
    pub fn hi_limit_if(&self) -> HiLimitIfR {
        HiLimitIfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC Lo Limit Monitor Interrupt Flag"]
    #[inline(always)]
    pub fn lo_limit_if(&self) -> LoLimitIfR {
        LoLimitIfR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn overflow_if(&self) -> OverflowIfR {
        OverflowIfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - ADC Interrupt Pending Status"]
    #[inline(always)]
    pub fn pending(&self) -> PendingR {
        PendingR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Done Interrupt Enable"]
    #[inline(always)]
    pub fn done_ie(&mut self) -> DoneIeW<IntrSpec> {
        DoneIeW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC Reference Ready Interrupt Enable"]
    #[inline(always)]
    pub fn ref_ready_ie(&mut self) -> RefReadyIeW<IntrSpec> {
        RefReadyIeW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC Hi Limit Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn hi_limit_ie(&mut self) -> HiLimitIeW<IntrSpec> {
        HiLimitIeW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC Lo Limit Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn lo_limit_ie(&mut self) -> LoLimitIeW<IntrSpec> {
        LoLimitIeW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn overflow_ie(&mut self) -> OverflowIeW<IntrSpec> {
        OverflowIeW::new(self, 4)
    }
    #[doc = "Bit 16 - ADC Done Interrupt Flag"]
    #[inline(always)]
    pub fn done_if(&mut self) -> DoneIfW<IntrSpec> {
        DoneIfW::new(self, 16)
    }
    #[doc = "Bit 17 - ADC Reference Ready Interrupt Flag"]
    #[inline(always)]
    pub fn ref_ready_if(&mut self) -> RefReadyIfW<IntrSpec> {
        RefReadyIfW::new(self, 17)
    }
    #[doc = "Bit 18 - ADC Hi Limit Monitor Interrupt Flag"]
    #[inline(always)]
    pub fn hi_limit_if(&mut self) -> HiLimitIfW<IntrSpec> {
        HiLimitIfW::new(self, 18)
    }
    #[doc = "Bit 19 - ADC Lo Limit Monitor Interrupt Flag"]
    #[inline(always)]
    pub fn lo_limit_if(&mut self) -> LoLimitIfW<IntrSpec> {
        LoLimitIfW::new(self, 19)
    }
    #[doc = "Bit 20 - ADC Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn overflow_if(&mut self) -> OverflowIfW<IntrSpec> {
        OverflowIfW::new(self, 20)
    }
}
#[doc = "ADC Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSpec;
impl crate::RegisterSpec for IntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for IntrSpec {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for IntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x001f_0000;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for IntrSpec {
    const RESET_VALUE: u32 = 0;
}
