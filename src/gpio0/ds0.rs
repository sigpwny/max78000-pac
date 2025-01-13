#[doc = "Register `DS0` reader"]
pub type R = crate::R<Ds0Spec>;
#[doc = "Register `DS0` writer"]
pub type W = crate::W<Ds0Spec>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioDs0 {
    #[doc = "0: GPIO port pin is in low-drive mode."]
    Ld = 0,
    #[doc = "1: GPIO port pin is in high-drive mode."]
    Hd = 1,
}
impl From<GpioDs0> for u32 {
    #[inline(always)]
    fn from(variant: GpioDs0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioDs0 {
    type Ux = u32;
}
impl crate::IsEnum for GpioDs0 {}
#[doc = "Field `GPIO_DS0` reader - Mask of all of the pins on the port."]
pub type GpioDs0R = crate::FieldReader<GpioDs0>;
impl GpioDs0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioDs0> {
        match self.bits {
            0 => Some(GpioDs0::Ld),
            1 => Some(GpioDs0::Hd),
            _ => None,
        }
    }
    #[doc = "GPIO port pin is in low-drive mode."]
    #[inline(always)]
    pub fn is_ld(&self) -> bool {
        *self == GpioDs0::Ld
    }
    #[doc = "GPIO port pin is in high-drive mode."]
    #[inline(always)]
    pub fn is_hd(&self) -> bool {
        *self == GpioDs0::Hd
    }
}
#[doc = "Field `GPIO_DS0` writer - Mask of all of the pins on the port."]
pub type GpioDs0W<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioDs0>;
impl<'a, REG> GpioDs0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "GPIO port pin is in low-drive mode."]
    #[inline(always)]
    pub fn ld(self) -> &'a mut crate::W<REG> {
        self.variant(GpioDs0::Ld)
    }
    #[doc = "GPIO port pin is in high-drive mode."]
    #[inline(always)]
    pub fn hd(self) -> &'a mut crate::W<REG> {
        self.variant(GpioDs0::Hd)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_ds0(&self) -> GpioDs0R {
        GpioDs0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_ds0(&mut self) -> GpioDs0W<Ds0Spec> {
        GpioDs0W::new(self, 0)
    }
}
#[doc = "GPIO Drive Strength Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`ds0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ds0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ds0Spec;
impl crate::RegisterSpec for Ds0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ds0::R`](R) reader structure"]
impl crate::Readable for Ds0Spec {}
#[doc = "`write(|w| ..)` method takes [`ds0::W`](W) writer structure"]
impl crate::Writable for Ds0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DS0 to value 0"]
impl crate::Resettable for Ds0Spec {
    const RESET_VALUE: u32 = 0;
}
