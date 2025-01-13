#[doc = "Register `INTMODE` reader"]
pub type R = crate::R<IntmodeSpec>;
#[doc = "Register `INTMODE` writer"]
pub type W = crate::W<IntmodeSpec>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioIntmode {
    #[doc = "0: Interrupts for this pin are level triggered."]
    Level = 0,
    #[doc = "1: Interrupts for this pin are edge triggered."]
    Edge = 1,
}
impl From<GpioIntmode> for u32 {
    #[inline(always)]
    fn from(variant: GpioIntmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioIntmode {
    type Ux = u32;
}
impl crate::IsEnum for GpioIntmode {}
#[doc = "Field `GPIO_INTMODE` reader - Mask of all of the pins on the port."]
pub type GpioIntmodeR = crate::FieldReader<GpioIntmode>;
impl GpioIntmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioIntmode> {
        match self.bits {
            0 => Some(GpioIntmode::Level),
            1 => Some(GpioIntmode::Edge),
            _ => None,
        }
    }
    #[doc = "Interrupts for this pin are level triggered."]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == GpioIntmode::Level
    }
    #[doc = "Interrupts for this pin are edge triggered."]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == GpioIntmode::Edge
    }
}
#[doc = "Field `GPIO_INTMODE` writer - Mask of all of the pins on the port."]
pub type GpioIntmodeW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioIntmode>;
impl<'a, REG> GpioIntmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Interrupts for this pin are level triggered."]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIntmode::Level)
    }
    #[doc = "Interrupts for this pin are edge triggered."]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIntmode::Edge)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_intmode(&self) -> GpioIntmodeR {
        GpioIntmodeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_intmode(&mut self) -> GpioIntmodeW<IntmodeSpec> {
        GpioIntmodeW::new(self, 0)
    }
}
#[doc = "GPIO Interrupt Mode Register. Each bit in this register controls the interrupt mode setting for the associated GPIO pin on this port.\n\nYou can [`read`](crate::Reg::read) this register and get [`intmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntmodeSpec;
impl crate::RegisterSpec for IntmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmode::R`](R) reader structure"]
impl crate::Readable for IntmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`intmode::W`](W) writer structure"]
impl crate::Writable for IntmodeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTMODE to value 0"]
impl crate::Resettable for IntmodeSpec {
    const RESET_VALUE: u32 = 0;
}
