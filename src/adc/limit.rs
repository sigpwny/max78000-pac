#[doc = "Register `LIMIT[%s]` reader"]
pub type R = crate::R<LimitSpec>;
#[doc = "Register `LIMIT[%s]` writer"]
pub type W = crate::W<LimitSpec>;
#[doc = "Field `ch_lo_limit` reader - Low Limit Threshold"]
pub type ChLoLimitR = crate::FieldReader<u16>;
#[doc = "Field `ch_lo_limit` writer - Low Limit Threshold"]
pub type ChLoLimitW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ch_hi_limit` reader - High Limit Threshold"]
pub type ChHiLimitR = crate::FieldReader<u16>;
#[doc = "Field `ch_hi_limit` writer - High Limit Threshold"]
pub type ChHiLimitW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ch_sel` reader - ADC Channel Select"]
pub type ChSelR = crate::FieldReader;
#[doc = "Field `ch_sel` writer - ADC Channel Select"]
pub type ChSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ch_lo_limit_en` reader - Low Limit Monitoring Enable"]
pub type ChLoLimitEnR = crate::BitReader;
#[doc = "Field `ch_lo_limit_en` writer - Low Limit Monitoring Enable"]
pub type ChLoLimitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ch_hi_limit_en` reader - High Limit Monitoring Enable"]
pub type ChHiLimitEnR = crate::BitReader;
#[doc = "Field `ch_hi_limit_en` writer - High Limit Monitoring Enable"]
pub type ChHiLimitEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Low Limit Threshold"]
    #[inline(always)]
    pub fn ch_lo_limit(&self) -> ChLoLimitR {
        ChLoLimitR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21 - High Limit Threshold"]
    #[inline(always)]
    pub fn ch_hi_limit(&self) -> ChHiLimitR {
        ChHiLimitR::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:28 - ADC Channel Select"]
    #[inline(always)]
    pub fn ch_sel(&self) -> ChSelR {
        ChSelR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - Low Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_lo_limit_en(&self) -> ChLoLimitEnR {
        ChLoLimitEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - High Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_hi_limit_en(&self) -> ChHiLimitEnR {
        ChHiLimitEnR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Low Limit Threshold"]
    #[inline(always)]
    pub fn ch_lo_limit(&mut self) -> ChLoLimitW<LimitSpec> {
        ChLoLimitW::new(self, 0)
    }
    #[doc = "Bits 12:21 - High Limit Threshold"]
    #[inline(always)]
    pub fn ch_hi_limit(&mut self) -> ChHiLimitW<LimitSpec> {
        ChHiLimitW::new(self, 12)
    }
    #[doc = "Bits 24:28 - ADC Channel Select"]
    #[inline(always)]
    pub fn ch_sel(&mut self) -> ChSelW<LimitSpec> {
        ChSelW::new(self, 24)
    }
    #[doc = "Bit 29 - Low Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_lo_limit_en(&mut self) -> ChLoLimitEnW<LimitSpec> {
        ChLoLimitEnW::new(self, 29)
    }
    #[doc = "Bit 30 - High Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_hi_limit_en(&mut self) -> ChHiLimitEnW<LimitSpec> {
        ChHiLimitEnW::new(self, 30)
    }
}
#[doc = "ADC Limit\n\nYou can [`read`](crate::Reg::read) this register and get [`limit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LimitSpec;
impl crate::RegisterSpec for LimitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`limit::R`](R) reader structure"]
impl crate::Readable for LimitSpec {}
#[doc = "`write(|w| ..)` method takes [`limit::W`](W) writer structure"]
impl crate::Writable for LimitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LIMIT[%s]
to value 0"]
impl crate::Resettable for LimitSpec {
    const RESET_VALUE: u32 = 0;
}
