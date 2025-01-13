#[doc = "Register `INTFL` reader"]
pub type R = crate::R<IntflSpec>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioIntfl {
    #[doc = "0: No Interrupt is pending on this GPIO pin."]
    No = 0,
    #[doc = "1: An Interrupt is pending on this GPIO pin."]
    Pending = 1,
}
impl From<GpioIntfl> for u32 {
    #[inline(always)]
    fn from(variant: GpioIntfl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioIntfl {
    type Ux = u32;
}
impl crate::IsEnum for GpioIntfl {}
#[doc = "Field `GPIO_INTFL` reader - Mask of all of the pins on the port."]
pub type GpioIntflR = crate::FieldReader<GpioIntfl>;
impl GpioIntflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioIntfl> {
        match self.bits {
            0 => Some(GpioIntfl::No),
            1 => Some(GpioIntfl::Pending),
            _ => None,
        }
    }
    #[doc = "No Interrupt is pending on this GPIO pin."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == GpioIntfl::No
    }
    #[doc = "An Interrupt is pending on this GPIO pin."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == GpioIntfl::Pending
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_intfl(&self) -> GpioIntflR {
        GpioIntflR::new(self.bits)
    }
}
#[doc = "GPIO Interrupt Status Register. Each bit in this register contains the pending interrupt status for the associated GPIO pin in this port.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflSpec;
impl crate::RegisterSpec for IntflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl::R`](R) reader structure"]
impl crate::Readable for IntflSpec {}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for IntflSpec {
    const RESET_VALUE: u32 = 0;
}
