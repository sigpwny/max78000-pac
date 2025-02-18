#[doc = "Register `PADCTRL0` reader"]
pub type R = crate::R<Padctrl0Spec>;
#[doc = "Register `PADCTRL0` writer"]
pub type W = crate::W<Padctrl0Spec>;
#[doc = "The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioPadctrl0 {
    #[doc = "0: High Impedance."]
    Impedance = 0,
    #[doc = "1: Weak pull-up mode."]
    Pu = 1,
    #[doc = "2: weak pull-down mode."]
    Pd = 2,
}
impl From<GpioPadctrl0> for u32 {
    #[inline(always)]
    fn from(variant: GpioPadctrl0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioPadctrl0 {
    type Ux = u32;
}
impl crate::IsEnum for GpioPadctrl0 {}
#[doc = "Field `GPIO_PADCTRL0` reader - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
pub type GpioPadctrl0R = crate::FieldReader<GpioPadctrl0>;
impl GpioPadctrl0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioPadctrl0> {
        match self.bits {
            0 => Some(GpioPadctrl0::Impedance),
            1 => Some(GpioPadctrl0::Pu),
            2 => Some(GpioPadctrl0::Pd),
            _ => None,
        }
    }
    #[doc = "High Impedance."]
    #[inline(always)]
    pub fn is_impedance(&self) -> bool {
        *self == GpioPadctrl0::Impedance
    }
    #[doc = "Weak pull-up mode."]
    #[inline(always)]
    pub fn is_pu(&self) -> bool {
        *self == GpioPadctrl0::Pu
    }
    #[doc = "weak pull-down mode."]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == GpioPadctrl0::Pd
    }
}
#[doc = "Field `GPIO_PADCTRL0` writer - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
pub type GpioPadctrl0W<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioPadctrl0>;
impl<'a, REG> GpioPadctrl0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "High Impedance."]
    #[inline(always)]
    pub fn impedance(self) -> &'a mut crate::W<REG> {
        self.variant(GpioPadctrl0::Impedance)
    }
    #[doc = "Weak pull-up mode."]
    #[inline(always)]
    pub fn pu(self) -> &'a mut crate::W<REG> {
        self.variant(GpioPadctrl0::Pu)
    }
    #[doc = "weak pull-down mode."]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(GpioPadctrl0::Pd)
    }
}
impl R {
    #[doc = "Bits 0:31 - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
    #[inline(always)]
    pub fn gpio_padctrl0(&self) -> GpioPadctrl0R {
        GpioPadctrl0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The two bits in GPIO_PAD_CFG1 and GPIO_PAD_CFG2 for each GPIO pin work together to determine the pad mode when the GPIO is set to input mode."]
    #[inline(always)]
    pub fn gpio_padctrl0(&mut self) -> GpioPadctrl0W<Padctrl0Spec> {
        GpioPadctrl0W::new(self, 0)
    }
}
#[doc = "GPIO Input Mode Config 1. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port.\n\nYou can [`read`](crate::Reg::read) this register and get [`padctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Padctrl0Spec;
impl crate::RegisterSpec for Padctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padctrl0::R`](R) reader structure"]
impl crate::Readable for Padctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`padctrl0::W`](W) writer structure"]
impl crate::Writable for Padctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PADCTRL0 to value 0"]
impl crate::Resettable for Padctrl0Spec {
    const RESET_VALUE: u32 = 0;
}
