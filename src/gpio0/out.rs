#[doc = "Register `OUT` reader"]
pub type R = crate::R<OutSpec>;
#[doc = "Register `OUT` writer"]
pub type W = crate::W<OutSpec>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioOut {
    #[doc = "0: Drive Logic 0 (low) on GPIO output."]
    Low = 0,
    #[doc = "1: Drive logic 1 (high) on GPIO output."]
    High = 1,
}
impl From<GpioOut> for u32 {
    #[inline(always)]
    fn from(variant: GpioOut) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioOut {
    type Ux = u32;
}
impl crate::IsEnum for GpioOut {}
#[doc = "Field `GPIO_OUT` reader - Mask of all of the pins on the port."]
pub type GpioOutR = crate::FieldReader<GpioOut>;
impl GpioOutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioOut> {
        match self.bits {
            0 => Some(GpioOut::Low),
            1 => Some(GpioOut::High),
            _ => None,
        }
    }
    #[doc = "Drive Logic 0 (low) on GPIO output."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == GpioOut::Low
    }
    #[doc = "Drive logic 1 (high) on GPIO output."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == GpioOut::High
    }
}
#[doc = "Field `GPIO_OUT` writer - Mask of all of the pins on the port."]
pub type GpioOutW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioOut>;
impl<'a, REG> GpioOutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Drive Logic 0 (low) on GPIO output."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(GpioOut::Low)
    }
    #[doc = "Drive logic 1 (high) on GPIO output."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(GpioOut::High)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_out(&self) -> GpioOutR {
        GpioOutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_out(&mut self) -> GpioOutW<OutSpec> {
        GpioOutW::new(self, 0)
    }
}
#[doc = "GPIO Output Register. Each bit controls the GPIO_OUT setting for one pin in the associated port. This register can be written either directly, or by using the GPIO_OUT_SET and GPIO_OUT_CLR registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutSpec;
impl crate::RegisterSpec for OutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out::R`](R) reader structure"]
impl crate::Readable for OutSpec {}
#[doc = "`write(|w| ..)` method takes [`out::W`](W) writer structure"]
impl crate::Writable for OutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT to value 0"]
impl crate::Resettable for OutSpec {
    const RESET_VALUE: u32 = 0;
}
