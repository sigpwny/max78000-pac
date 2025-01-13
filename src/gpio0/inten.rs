#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioInten {
    #[doc = "0: Interrupts are disabled for this GPIO pin."]
    Dis = 0,
    #[doc = "1: Interrupts are enabled for this GPIO pin."]
    En = 1,
}
impl From<GpioInten> for u32 {
    #[inline(always)]
    fn from(variant: GpioInten) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioInten {
    type Ux = u32;
}
impl crate::IsEnum for GpioInten {}
#[doc = "Field `GPIO_INTEN` reader - Mask of all of the pins on the port."]
pub type GpioIntenR = crate::FieldReader<GpioInten>;
impl GpioIntenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioInten> {
        match self.bits {
            0 => Some(GpioInten::Dis),
            1 => Some(GpioInten::En),
            _ => None,
        }
    }
    #[doc = "Interrupts are disabled for this GPIO pin."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GpioInten::Dis
    }
    #[doc = "Interrupts are enabled for this GPIO pin."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GpioInten::En
    }
}
#[doc = "Field `GPIO_INTEN` writer - Mask of all of the pins on the port."]
pub type GpioIntenW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioInten>;
impl<'a, REG> GpioIntenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Interrupts are disabled for this GPIO pin."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInten::Dis)
    }
    #[doc = "Interrupts are enabled for this GPIO pin."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(GpioInten::En)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_inten(&self) -> GpioIntenR {
        GpioIntenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_inten(&mut self) -> GpioIntenW<IntenSpec> {
        GpioIntenW::new(self, 0)
    }
}
#[doc = "GPIO Interrupt Enable Register. Each bit in this register controls the GPIO interrupt enable for the associated pin on the GPIO port.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
