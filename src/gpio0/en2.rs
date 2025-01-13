#[doc = "Register `EN2` reader"]
pub type R = crate::R<En2Spec>;
#[doc = "Register `EN2` writer"]
pub type W = crate::W<En2Spec>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioEn2 {
    #[doc = "0: Primary function selected."]
    Primary = 0,
    #[doc = "1: Secondary function selected."]
    Secondary = 1,
}
impl From<GpioEn2> for u32 {
    #[inline(always)]
    fn from(variant: GpioEn2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioEn2 {
    type Ux = u32;
}
impl crate::IsEnum for GpioEn2 {}
#[doc = "Field `GPIO_EN2` reader - Mask of all of the pins on the port."]
pub type GpioEn2R = crate::FieldReader<GpioEn2>;
impl GpioEn2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioEn2> {
        match self.bits {
            0 => Some(GpioEn2::Primary),
            1 => Some(GpioEn2::Secondary),
            _ => None,
        }
    }
    #[doc = "Primary function selected."]
    #[inline(always)]
    pub fn is_primary(&self) -> bool {
        *self == GpioEn2::Primary
    }
    #[doc = "Secondary function selected."]
    #[inline(always)]
    pub fn is_secondary(&self) -> bool {
        *self == GpioEn2::Secondary
    }
}
#[doc = "Field `GPIO_EN2` writer - Mask of all of the pins on the port."]
pub type GpioEn2W<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioEn2>;
impl<'a, REG> GpioEn2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Primary function selected."]
    #[inline(always)]
    pub fn primary(self) -> &'a mut crate::W<REG> {
        self.variant(GpioEn2::Primary)
    }
    #[doc = "Secondary function selected."]
    #[inline(always)]
    pub fn secondary(self) -> &'a mut crate::W<REG> {
        self.variant(GpioEn2::Secondary)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_en2(&self) -> GpioEn2R {
        GpioEn2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_en2(&mut self) -> GpioEn2W<En2Spec> {
        GpioEn2W::new(self, 0)
    }
}
#[doc = "GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port.\n\nYou can [`read`](crate::Reg::read) this register and get [`en2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct En2Spec;
impl crate::RegisterSpec for En2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en2::R`](R) reader structure"]
impl crate::Readable for En2Spec {}
#[doc = "`write(|w| ..)` method takes [`en2::W`](W) writer structure"]
impl crate::Writable for En2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EN2 to value 0"]
impl crate::Resettable for En2Spec {
    const RESET_VALUE: u32 = 0;
}
