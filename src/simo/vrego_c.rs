#[doc = "Register `VREGO_C` reader"]
pub type R = crate::R<VregoCSpec>;
#[doc = "Register `VREGO_C` writer"]
pub type W = crate::W<VregoCSpec>;
#[doc = "Field `VSETC` reader - Regulator Output Voltage Setting"]
pub type VsetcR = crate::FieldReader;
#[doc = "Field `VSETC` writer - Regulator Output Voltage Setting"]
pub type VsetcW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Regulator Output Range Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rangec {
    #[doc = "0: Low output voltage range"]
    Low = 0,
    #[doc = "1: High output voltage range"]
    High = 1,
}
impl From<Rangec> for bool {
    #[inline(always)]
    fn from(variant: Rangec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGEC` reader - Regulator Output Range Set"]
pub type RangecR = crate::BitReader<Rangec>;
impl RangecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rangec {
        match self.bits {
            false => Rangec::Low,
            true => Rangec::High,
        }
    }
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Rangec::Low
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Rangec::High
    }
}
#[doc = "Field `RANGEC` writer - Regulator Output Range Set"]
pub type RangecW<'a, REG> = crate::BitWriter<'a, REG, Rangec>;
impl<'a, REG> RangecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Rangec::Low)
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Rangec::High)
    }
}
impl R {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    pub fn vsetc(&self) -> VsetcR {
        VsetcR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    pub fn rangec(&self) -> RangecR {
        RangecR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    pub fn vsetc(&mut self) -> VsetcW<VregoCSpec> {
        VsetcW::new(self, 0)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    pub fn rangec(&mut self) -> RangecW<VregoCSpec> {
        RangecW::new(self, 7)
    }
}
#[doc = "Buck Voltage Regulator C Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vrego_c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrego_c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VregoCSpec;
impl crate::RegisterSpec for VregoCSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrego_c::R`](R) reader structure"]
impl crate::Readable for VregoCSpec {}
#[doc = "`write(|w| ..)` method takes [`vrego_c::W`](W) writer structure"]
impl crate::Writable for VregoCSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREGO_C to value 0"]
impl crate::Resettable for VregoCSpec {
    const RESET_VALUE: u32 = 0;
}
