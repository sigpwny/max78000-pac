#[doc = "Register `EN1` reader"]
pub type R = crate::R<En1Spec>;
#[doc = "Register `EN1` writer"]
pub type W = crate::W<En1Spec>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioEn1 {
    #[doc = "0: Primary function selected."]
    Primary = 0,
    #[doc = "1: Secondary function selected."]
    Secondary = 1,
}
impl From<GpioEn1> for u32 {
    #[inline(always)]
    fn from(variant: GpioEn1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioEn1 {
    type Ux = u32;
}
impl crate::IsEnum for GpioEn1 {}
#[doc = "Field `GPIO_EN1` reader - Mask of all of the pins on the port."]
pub type GpioEn1R = crate::FieldReader<GpioEn1>;
impl GpioEn1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioEn1> {
        match self.bits {
            0 => Some(GpioEn1::Primary),
            1 => Some(GpioEn1::Secondary),
            _ => None,
        }
    }
    #[doc = "Primary function selected."]
    #[inline(always)]
    pub fn is_primary(&self) -> bool {
        *self == GpioEn1::Primary
    }
    #[doc = "Secondary function selected."]
    #[inline(always)]
    pub fn is_secondary(&self) -> bool {
        *self == GpioEn1::Secondary
    }
}
#[doc = "Field `GPIO_EN1` writer - Mask of all of the pins on the port."]
pub type GpioEn1W<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioEn1>;
impl<'a, REG> GpioEn1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Primary function selected."]
    #[inline(always)]
    pub fn primary(self) -> &'a mut crate::W<REG> {
        self.variant(GpioEn1::Primary)
    }
    #[doc = "Secondary function selected."]
    #[inline(always)]
    pub fn secondary(self) -> &'a mut crate::W<REG> {
        self.variant(GpioEn1::Secondary)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_en1(&self) -> GpioEn1R {
        GpioEn1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_en1(&mut self) -> GpioEn1W<En1Spec> {
        GpioEn1W::new(self, 0)
    }
}
#[doc = "GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port.\n\nYou can [`read`](crate::Reg::read) this register and get [`en1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct En1Spec;
impl crate::RegisterSpec for En1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en1::R`](R) reader structure"]
impl crate::Readable for En1Spec {}
#[doc = "`write(|w| ..)` method takes [`en1::W`](W) writer structure"]
impl crate::Writable for En1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EN1 to value 0"]
impl crate::Resettable for En1Spec {
    const RESET_VALUE: u32 = 0;
}
