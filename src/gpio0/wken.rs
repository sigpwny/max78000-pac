#[doc = "Register `WKEN` reader"]
pub type R = crate::R<WkenSpec>;
#[doc = "Register `WKEN` writer"]
pub type W = crate::W<WkenSpec>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioWken {
    #[doc = "0: PMU wakeup for this GPIO is disabled."]
    Dis = 0,
    #[doc = "1: PMU wakeup for this GPIO is enabled."]
    En = 1,
}
impl From<GpioWken> for u32 {
    #[inline(always)]
    fn from(variant: GpioWken) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioWken {
    type Ux = u32;
}
impl crate::IsEnum for GpioWken {}
#[doc = "Field `GPIO_WKEN` reader - Mask of all of the pins on the port."]
pub type GpioWkenR = crate::FieldReader<GpioWken>;
impl GpioWkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioWken> {
        match self.bits {
            0 => Some(GpioWken::Dis),
            1 => Some(GpioWken::En),
            _ => None,
        }
    }
    #[doc = "PMU wakeup for this GPIO is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GpioWken::Dis
    }
    #[doc = "PMU wakeup for this GPIO is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GpioWken::En
    }
}
#[doc = "Field `GPIO_WKEN` writer - Mask of all of the pins on the port."]
pub type GpioWkenW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioWken>;
impl<'a, REG> GpioWkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "PMU wakeup for this GPIO is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(GpioWken::Dis)
    }
    #[doc = "PMU wakeup for this GPIO is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(GpioWken::En)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_wken(&self) -> GpioWkenR {
        GpioWkenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_wken(&mut self) -> GpioWkenW<WkenSpec> {
        GpioWkenW::new(self, 0)
    }
}
#[doc = "GPIO Wake Enable Register. Each bit in this register controls the PMU wakeup enable for the associated GPIO pin in this port.\n\nYou can [`read`](crate::Reg::read) this register and get [`wken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkenSpec;
impl crate::RegisterSpec for WkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wken::R`](R) reader structure"]
impl crate::Readable for WkenSpec {}
#[doc = "`write(|w| ..)` method takes [`wken::W`](W) writer structure"]
impl crate::Writable for WkenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WKEN to value 0"]
impl crate::Resettable for WkenSpec {
    const RESET_VALUE: u32 = 0;
}
