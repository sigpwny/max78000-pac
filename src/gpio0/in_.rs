#[doc = "Register `IN` reader"]
pub type R = crate::R<InSpec>;
#[doc = "Field `GPIO_IN` reader - Mask of all of the pins on the port."]
pub type GpioInR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_in(&self) -> GpioInR {
        GpioInR::new(self.bits)
    }
}
#[doc = "GPIO Input Register. Read-only register to read from the logic states of the GPIO pins on this port.\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InSpec;
impl crate::RegisterSpec for InSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_::R`](R) reader structure"]
impl crate::Readable for InSpec {}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for InSpec {
    const RESET_VALUE: u32 = 0;
}
