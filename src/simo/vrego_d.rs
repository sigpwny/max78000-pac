#[doc = "Register `VREGO_D` reader"]
pub type R = crate::R<VregoDSpec>;
#[doc = "Register `VREGO_D` writer"]
pub type W = crate::W<VregoDSpec>;
#[doc = "Field `VSETD` reader - Regulator Output Voltage Setting"]
pub type VsetdR = crate::FieldReader;
#[doc = "Field `VSETD` writer - Regulator Output Voltage Setting"]
pub type VsetdW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Regulator Output Range Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ranged {
    #[doc = "0: Low output voltage range"]
    Low = 0,
    #[doc = "1: High output voltage range"]
    High = 1,
}
impl From<Ranged> for bool {
    #[inline(always)]
    fn from(variant: Ranged) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGED` reader - Regulator Output Range Set"]
pub type RangedR = crate::BitReader<Ranged>;
impl RangedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ranged {
        match self.bits {
            false => Ranged::Low,
            true => Ranged::High,
        }
    }
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ranged::Low
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ranged::High
    }
}
#[doc = "Field `RANGED` writer - Regulator Output Range Set"]
pub type RangedW<'a, REG> = crate::BitWriter<'a, REG, Ranged>;
impl<'a, REG> RangedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low output voltage range"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ranged::Low)
    }
    #[doc = "High output voltage range"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ranged::High)
    }
}
impl R {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    pub fn vsetd(&self) -> VsetdR {
        VsetdR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    pub fn ranged(&self) -> RangedR {
        RangedR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Regulator Output Voltage Setting"]
    #[inline(always)]
    pub fn vsetd(&mut self) -> VsetdW<VregoDSpec> {
        VsetdW::new(self, 0)
    }
    #[doc = "Bit 7 - Regulator Output Range Set"]
    #[inline(always)]
    pub fn ranged(&mut self) -> RangedW<VregoDSpec> {
        RangedW::new(self, 7)
    }
}
#[doc = "Buck Voltage Regulator D Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vrego_d::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrego_d::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VregoDSpec;
impl crate::RegisterSpec for VregoDSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrego_d::R`](R) reader structure"]
impl crate::Readable for VregoDSpec {}
#[doc = "`write(|w| ..)` method takes [`vrego_d::W`](W) writer structure"]
impl crate::Writable for VregoDSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREGO_D to value 0"]
impl crate::Resettable for VregoDSpec {
    const RESET_VALUE: u32 = 0;
}
