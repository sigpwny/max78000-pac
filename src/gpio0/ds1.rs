#[doc = "Register `DS1` reader"]
pub type R = crate::R<Ds1Spec>;
#[doc = "Register `DS1` writer"]
pub type W = crate::W<Ds1Spec>;
#[doc = "Field `GPIO_DS1` reader - Mask of all of the pins on the port."]
pub type GpioDs1R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_DS1` writer - Mask of all of the pins on the port."]
pub type GpioDs1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_ds1(&self) -> GpioDs1R {
        GpioDs1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_ds1(&mut self) -> GpioDs1W<Ds1Spec> {
        GpioDs1W::new(self, 0)
    }
}
#[doc = "GPIO Drive Strength 1 Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`ds1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ds1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ds1Spec;
impl crate::RegisterSpec for Ds1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ds1::R`](R) reader structure"]
impl crate::Readable for Ds1Spec {}
#[doc = "`write(|w| ..)` method takes [`ds1::W`](W) writer structure"]
impl crate::Writable for Ds1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DS1 to value 0"]
impl crate::Resettable for Ds1Spec {
    const RESET_VALUE: u32 = 0;
}
