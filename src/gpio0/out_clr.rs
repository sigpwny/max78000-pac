#[doc = "Register `OUT_CLR` writer"]
pub type W = crate::W<OutClrSpec>;
#[doc = "Field `GPIO_OUT_CLR` writer - Mask of all of the pins on the port."]
pub type GpioOutClrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_out_clr(&mut self) -> GpioOutClrW<OutClrSpec> {
        GpioOutClrW::new(self, 0)
    }
}
#[doc = "GPIO Output Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT to 0, without affecting other bits in that register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutClrSpec;
impl crate::RegisterSpec for OutClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_clr::W`](W) writer structure"]
impl crate::Writable for OutClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_CLR to value 0"]
impl crate::Resettable for OutClrSpec {
    const RESET_VALUE: u32 = 0;
}
