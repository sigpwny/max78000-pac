#[doc = "Register `ECCEN` reader"]
pub type R = crate::R<EccenSpec>;
#[doc = "Register `ECCEN` writer"]
pub type W = crate::W<EccenSpec>;
#[doc = "ECC System RAM0 Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ram0 {
    #[doc = "0: disabled."]
    Dis = 0,
    #[doc = "1: enabled."]
    En = 1,
}
impl From<Ram0> for bool {
    #[inline(always)]
    fn from(variant: Ram0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM0` reader - ECC System RAM0 Enable."]
pub type Ram0R = crate::BitReader<Ram0>;
impl Ram0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ram0 {
        match self.bits {
            false => Ram0::Dis,
            true => Ram0::En,
        }
    }
    #[doc = "disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ram0::Dis
    }
    #[doc = "enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ram0::En
    }
}
#[doc = "Field `RAM0` writer - ECC System RAM0 Enable."]
pub type Ram0W<'a, REG> = crate::BitWriter<'a, REG, Ram0>;
impl<'a, REG> Ram0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ram0::Dis)
    }
    #[doc = "enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ram0::En)
    }
}
impl R {
    #[doc = "Bit 0 - ECC System RAM0 Enable."]
    #[inline(always)]
    pub fn ram0(&self) -> Ram0R {
        Ram0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC System RAM0 Enable."]
    #[inline(always)]
    pub fn ram0(&mut self) -> Ram0W<EccenSpec> {
        Ram0W::new(self, 0)
    }
}
#[doc = "ECC Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccenSpec;
impl crate::RegisterSpec for EccenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccen::R`](R) reader structure"]
impl crate::Readable for EccenSpec {}
#[doc = "`write(|w| ..)` method takes [`eccen::W`](W) writer structure"]
impl crate::Writable for EccenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECCEN to value 0"]
impl crate::Resettable for EccenSpec {
    const RESET_VALUE: u32 = 0;
}
