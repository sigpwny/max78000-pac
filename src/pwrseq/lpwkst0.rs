#[doc = "Register `LPWKST0` reader"]
pub type R = crate::R<Lpwkst0Spec>;
#[doc = "Register `LPWKST0` writer"]
pub type W = crate::W<Lpwkst0Spec>;
#[doc = "Field `WAKEST` reader - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
pub type WakestR = crate::BitReader;
#[doc = "Field `WAKEST` writer - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
pub type WakestW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
    #[inline(always)]
    pub fn wakest(&self) -> WakestR {
        WakestR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup IRQ flags (write ones to clear). One or more of these bits will be set when the corresponding dedicated GPIO pin (s) transition (s) from low to high or high to low. If GPIO wakeup source is selected, using PM.GPIOWKEN register, and the corresponding bit is also selected in LPWKEN register, an interrupt will be gnerated to wake up the CPU from a low power mode."]
    #[inline(always)]
    pub fn wakest(&mut self) -> WakestW<Lpwkst0Spec> {
        WakestW::new(self, 0)
    }
}
#[doc = "Low Power I/O Wakeup Status Register 0. This register indicates the low power wakeup status for GPIO0.\n\nYou can [`read`](crate::Reg::read) this register and get [`lpwkst0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpwkst0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lpwkst0Spec;
impl crate::RegisterSpec for Lpwkst0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpwkst0::R`](R) reader structure"]
impl crate::Readable for Lpwkst0Spec {}
#[doc = "`write(|w| ..)` method takes [`lpwkst0::W`](W) writer structure"]
impl crate::Writable for Lpwkst0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPWKST0 to value 0"]
impl crate::Resettable for Lpwkst0Spec {
    const RESET_VALUE: u32 = 0;
}
