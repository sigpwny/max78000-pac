#[doc = "Register `EN0` reader"]
pub type R = crate::R<En0Spec>;
#[doc = "Register `EN0` writer"]
pub type W = crate::W<En0Spec>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioEn {
    #[doc = "0: Alternate function enabled."]
    Alternate = 0,
    #[doc = "1: GPIO function is enabled."]
    Gpio = 1,
}
impl From<GpioEn> for u32 {
    #[inline(always)]
    fn from(variant: GpioEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioEn {
    type Ux = u32;
}
impl crate::IsEnum for GpioEn {}
#[doc = "Field `GPIO_EN` reader - Mask of all of the pins on the port."]
pub type GpioEnR = crate::FieldReader<GpioEn>;
impl GpioEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioEn> {
        match self.bits {
            0 => Some(GpioEn::Alternate),
            1 => Some(GpioEn::Gpio),
            _ => None,
        }
    }
    #[doc = "Alternate function enabled."]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == GpioEn::Alternate
    }
    #[doc = "GPIO function is enabled."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == GpioEn::Gpio
    }
}
#[doc = "Field `GPIO_EN` writer - Mask of all of the pins on the port."]
pub type GpioEnW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioEn>;
impl<'a, REG> GpioEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Alternate function enabled."]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(GpioEn::Alternate)
    }
    #[doc = "GPIO function is enabled."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(GpioEn::Gpio)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_en(&self) -> GpioEnR {
        GpioEnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_en(&mut self) -> GpioEnW<En0Spec> {
        GpioEnW::new(self, 0)
    }
}
#[doc = "GPIO Function Enable Register. Each bit controls the GPIO_EN setting for one GPIO pin on the associated port.\n\nYou can [`read`](crate::Reg::read) this register and get [`en0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct En0Spec;
impl crate::RegisterSpec for En0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en0::R`](R) reader structure"]
impl crate::Readable for En0Spec {}
#[doc = "`write(|w| ..)` method takes [`en0::W`](W) writer structure"]
impl crate::Writable for En0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EN0 to value 0"]
impl crate::Resettable for En0Spec {
    const RESET_VALUE: u32 = 0;
}
