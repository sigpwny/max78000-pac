#[doc = "Register `INTPOL` reader"]
pub type R = crate::R<IntpolSpec>;
#[doc = "Register `INTPOL` writer"]
pub type W = crate::W<IntpolSpec>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioIntpol {
    #[doc = "0: Interrupts are latched on a falling edge or low level condition for this pin."]
    Falling = 0,
    #[doc = "1: Interrupts are latched on a rising edge or high condition for this pin."]
    Rising = 1,
}
impl From<GpioIntpol> for u32 {
    #[inline(always)]
    fn from(variant: GpioIntpol) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioIntpol {
    type Ux = u32;
}
impl crate::IsEnum for GpioIntpol {}
#[doc = "Field `GPIO_INTPOL` reader - Mask of all of the pins on the port."]
pub type GpioIntpolR = crate::FieldReader<GpioIntpol>;
impl GpioIntpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioIntpol> {
        match self.bits {
            0 => Some(GpioIntpol::Falling),
            1 => Some(GpioIntpol::Rising),
            _ => None,
        }
    }
    #[doc = "Interrupts are latched on a falling edge or low level condition for this pin."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == GpioIntpol::Falling
    }
    #[doc = "Interrupts are latched on a rising edge or high condition for this pin."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == GpioIntpol::Rising
    }
}
#[doc = "Field `GPIO_INTPOL` writer - Mask of all of the pins on the port."]
pub type GpioIntpolW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioIntpol>;
impl<'a, REG> GpioIntpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Interrupts are latched on a falling edge or low level condition for this pin."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIntpol::Falling)
    }
    #[doc = "Interrupts are latched on a rising edge or high condition for this pin."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIntpol::Rising)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_intpol(&self) -> GpioIntpolR {
        GpioIntpolR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_intpol(&mut self) -> GpioIntpolW<IntpolSpec> {
        GpioIntpolW::new(self, 0)
    }
}
#[doc = "GPIO Interrupt Polarity Register. Each bit in this register controls the interrupt polarity setting for one GPIO pin in the associated port.\n\nYou can [`read`](crate::Reg::read) this register and get [`intpol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntpolSpec;
impl crate::RegisterSpec for IntpolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intpol::R`](R) reader structure"]
impl crate::Readable for IntpolSpec {}
#[doc = "`write(|w| ..)` method takes [`intpol::W`](W) writer structure"]
impl crate::Writable for IntpolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTPOL to value 0"]
impl crate::Resettable for IntpolSpec {
    const RESET_VALUE: u32 = 0;
}
