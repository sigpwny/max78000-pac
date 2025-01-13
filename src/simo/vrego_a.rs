#[doc = "Register `VREGO_A` reader"]
pub type R = crate::R<VregoASpec>;
#[doc = "Register `VREGO_A` writer"]
pub type W = crate::W<VregoASpec>;
#[doc = "Field `VSETA` reader - Regulator Output Voltage Setting"]
pub type VsetaR = crate::FieldReader;
#[doc = "Field `VSETA` writer - Regulator Output Voltage Setting"]
pub type VsetaW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Regulator Output Range Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rangea {
    #[doc = "0: Low output voltage range"]
    Low = 0,
    #[doc = "1: High output voltage range"]
    High = 1,
}
impl From<Rangea> for bool {
    #[inline(always)]
    fn from(variant: Rangea) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGEA` reader - Regulator Output Range Set"]
pub type RangeaR = crate::BitReader<Rangea>;
impl RangeaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rangea {
        match self.bits {
            false => Rangea::Low,
            true => Rangea::High,
        }
    }
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Rangea::Low
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Rangea::High
    }
}
#[doc = "Field `RANGEA` writer - Regulator Output Range Set"]
pub type RangeaW<'a, REG> = crate::BitWriter<'a, REG, Rangea>;
impl<'a, REG> RangeaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Rangea::Low)
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Rangea::High)
    }
}
impl R {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    pub fn vseta(&self) -> VsetaR {
        VsetaR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    pub fn rangea(&self) -> RangeaR {
        RangeaR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    pub fn vseta(&mut self) -> VsetaW<VregoASpec> {
        VsetaW::new(self, 0)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    pub fn rangea(&mut self) -> RangeaW<VregoASpec> {
        RangeaW::new(self, 7)
    }
}
#[doc = "Buck Voltage Regulator A Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vrego_a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrego_a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VregoASpec;
impl crate::RegisterSpec for VregoASpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrego_a::R`](R) reader structure"]
impl crate::Readable for VregoASpec {}
#[doc = "`write(|w| ..)` method takes [`vrego_a::W`](W) writer structure"]
impl crate::Writable for VregoASpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREGO_A to value 0"]
impl crate::Resettable for VregoASpec {
    const RESET_VALUE: u32 = 0;
}
