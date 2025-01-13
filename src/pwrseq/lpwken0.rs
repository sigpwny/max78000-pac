#[doc = "Register `LPWKEN0` reader"]
pub type R = crate::R<Lpwken0Spec>;
#[doc = "Register `LPWKEN0` writer"]
pub type W = crate::W<Lpwken0Spec>;
#[doc = "Field `WAKEEN` reader - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
pub type WakeenR = crate::FieldReader<u32>;
#[doc = "Field `WAKEEN` writer - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
pub type WakeenW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
    #[inline(always)]
    pub fn wakeen(&self) -> WakeenR {
        WakeenR::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - Enable wakeup. These bits allow wakeup from the corresponding GPIO pin (s) on transition (s) from low to high or high to low when PM.GPIOWKEN is set. Wakeup status is indicated in PPWKST register."]
    #[inline(always)]
    pub fn wakeen(&mut self) -> WakeenW<Lpwken0Spec> {
        WakeenW::new(self, 0)
    }
}
#[doc = "Low Power I/O Wakeup Enable Register 0. This register enables low power wakeup functionality for GPIO0.\n\nYou can [`read`](crate::Reg::read) this register and get [`lpwken0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpwken0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lpwken0Spec;
impl crate::RegisterSpec for Lpwken0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpwken0::R`](R) reader structure"]
impl crate::Readable for Lpwken0Spec {}
#[doc = "`write(|w| ..)` method takes [`lpwken0::W`](W) writer structure"]
impl crate::Writable for Lpwken0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPWKEN0 to value 0"]
impl crate::Resettable for Lpwken0Spec {
    const RESET_VALUE: u32 = 0;
}
