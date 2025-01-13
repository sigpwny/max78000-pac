#[doc = "Register `VREGO_B` reader"]
pub type R = crate::R<VregoBSpec>;
#[doc = "Register `VREGO_B` writer"]
pub type W = crate::W<VregoBSpec>;
#[doc = "Field `VSETB` reader - Regulator Output Voltage Setting"]
pub type VsetbR = crate::FieldReader;
#[doc = "Field `VSETB` writer - Regulator Output Voltage Setting"]
pub type VsetbW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Regulator Output Range Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rangeb {
    #[doc = "0: Low output voltage range"]
    Low = 0,
    #[doc = "1: High output voltage range"]
    High = 1,
}
impl From<Rangeb> for bool {
    #[inline(always)]
    fn from(variant: Rangeb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGEB` reader - Regulator Output Range Set"]
pub type RangebR = crate::BitReader<Rangeb>;
impl RangebR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rangeb {
        match self.bits {
            false => Rangeb::Low,
            true => Rangeb::High,
        }
    }
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Rangeb::Low
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Rangeb::High
    }
}
#[doc = "Field `RANGEB` writer - Regulator Output Range Set"]
pub type RangebW<'a, REG> = crate::BitWriter<'a, REG, Rangeb>;
impl<'a, REG> RangebW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Rangeb::Low)
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Rangeb::High)
    }
}
impl R {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    pub fn vsetb(&self) -> VsetbR {
        VsetbR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    pub fn rangeb(&self) -> RangebR {
        RangebR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    pub fn vsetb(&mut self) -> VsetbW<VregoBSpec> {
        VsetbW::new(self, 0)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    pub fn rangeb(&mut self) -> RangebW<VregoBSpec> {
        RangebW::new(self, 7)
    }
}
#[doc = "Buck Voltage Regulator B Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vrego_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrego_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VregoBSpec;
impl crate::RegisterSpec for VregoBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrego_b::R`](R) reader structure"]
impl crate::Readable for VregoBSpec {}
#[doc = "`write(|w| ..)` method takes [`vrego_b::W`](W) writer structure"]
impl crate::Writable for VregoBSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREGO_B to value 0"]
impl crate::Resettable for VregoBSpec {
    const RESET_VALUE: u32 = 0;
}
