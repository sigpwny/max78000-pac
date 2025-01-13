#[doc = "Register `HYSEN` reader"]
pub type R = crate::R<HysenSpec>;
#[doc = "Register `HYSEN` writer"]
pub type W = crate::W<HysenSpec>;
#[doc = "Field `GPIO_HYSEN` reader - Mask of all of the pins on the port."]
pub type GpioHysenR = crate::FieldReader<u32>;
#[doc = "Field `GPIO_HYSEN` writer - Mask of all of the pins on the port."]
pub type GpioHysenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_hysen(&self) -> GpioHysenR {
        GpioHysenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_hysen(&mut self) -> GpioHysenW<HysenSpec> {
        GpioHysenW::new(self, 0)
    }
}
#[doc = "GPIO Input Hysteresis Enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`hysen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hysen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HysenSpec;
impl crate::RegisterSpec for HysenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hysen::R`](R) reader structure"]
impl crate::Readable for HysenSpec {}
#[doc = "`write(|w| ..)` method takes [`hysen::W`](W) writer structure"]
impl crate::Writable for HysenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HYSEN to value 0"]
impl crate::Resettable for HysenSpec {
    const RESET_VALUE: u32 = 0;
}
