#[doc = "Register `RST` reader"]
pub type R = crate::R<RstSpec>;
#[doc = "Register `RST` writer"]
pub type W = crate::W<RstSpec>;
#[doc = "Low Power GPIO 2 Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reset {
    #[doc = "0: Reset complete."]
    ResetDone = 0,
    #[doc = "1: Starts Reset or indicates reset in progress."]
    Busy = 1,
}
impl From<Reset> for bool {
    #[inline(always)]
    fn from(variant: Reset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO2` reader - Low Power GPIO 2 Reset."]
pub type Gpio2R = crate::BitReader<Reset>;
impl Gpio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reset {
        match self.bits {
            false => Reset::ResetDone,
            true => Reset::Busy,
        }
    }
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn is_reset_done(&self) -> bool {
        *self == Reset::ResetDone
    }
    #[doc = "Starts Reset or indicates reset in progress."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Reset::Busy
    }
}
#[doc = "Field `GPIO2` writer - Low Power GPIO 2 Reset."]
pub type Gpio2W<'a, REG> = crate::BitWriter<'a, REG, Reset>;
impl<'a, REG> Gpio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset complete."]
    #[inline(always)]
    pub fn reset_done(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::ResetDone)
    }
    #[doc = "Starts Reset or indicates reset in progress."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Busy)
    }
}
#[doc = "Field `WDT1` reader - Low Power Watchdog Timer 1 Reset."]
pub use Gpio2R as Wdt1R;
#[doc = "Field `TMR4` reader - Low Power Timer 4 Reset."]
pub use Gpio2R as Tmr4R;
#[doc = "Field `TMR5` reader - Low Power Timer 5 Reset."]
pub use Gpio2R as Tmr5R;
#[doc = "Field `UART3` reader - Low Power UART 3 Reset."]
pub use Gpio2R as Uart3R;
#[doc = "Field `LPCOMP` reader - Low Power Comparator Reset."]
pub use Gpio2R as LpcompR;
#[doc = "Field `WDT1` writer - Low Power Watchdog Timer 1 Reset."]
pub use Gpio2W as Wdt1W;
#[doc = "Field `TMR4` writer - Low Power Timer 4 Reset."]
pub use Gpio2W as Tmr4W;
#[doc = "Field `TMR5` writer - Low Power Timer 5 Reset."]
pub use Gpio2W as Tmr5W;
#[doc = "Field `UART3` writer - Low Power UART 3 Reset."]
pub use Gpio2W as Uart3W;
#[doc = "Field `LPCOMP` writer - Low Power Comparator Reset."]
pub use Gpio2W as LpcompW;
impl R {
    #[doc = "Bit 0 - Low Power GPIO 2 Reset."]
    #[inline(always)]
    pub fn gpio2(&self) -> Gpio2R {
        Gpio2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Power Watchdog Timer 1 Reset."]
    #[inline(always)]
    pub fn wdt1(&self) -> Wdt1R {
        Wdt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Power Timer 4 Reset."]
    #[inline(always)]
    pub fn tmr4(&self) -> Tmr4R {
        Tmr4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low Power Timer 5 Reset."]
    #[inline(always)]
    pub fn tmr5(&self) -> Tmr5R {
        Tmr5R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Power UART 3 Reset."]
    #[inline(always)]
    pub fn uart3(&self) -> Uart3R {
        Uart3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Low Power Comparator Reset."]
    #[inline(always)]
    pub fn lpcomp(&self) -> LpcompR {
        LpcompR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power GPIO 2 Reset."]
    #[inline(always)]
    pub fn gpio2(&mut self) -> Gpio2W<RstSpec> {
        Gpio2W::new(self, 0)
    }
    #[doc = "Bit 1 - Low Power Watchdog Timer 1 Reset."]
    #[inline(always)]
    pub fn wdt1(&mut self) -> Wdt1W<RstSpec> {
        Wdt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Low Power Timer 4 Reset."]
    #[inline(always)]
    pub fn tmr4(&mut self) -> Tmr4W<RstSpec> {
        Tmr4W::new(self, 2)
    }
    #[doc = "Bit 3 - Low Power Timer 5 Reset."]
    #[inline(always)]
    pub fn tmr5(&mut self) -> Tmr5W<RstSpec> {
        Tmr5W::new(self, 3)
    }
    #[doc = "Bit 4 - Low Power UART 3 Reset."]
    #[inline(always)]
    pub fn uart3(&mut self) -> Uart3W<RstSpec> {
        Uart3W::new(self, 4)
    }
    #[doc = "Bit 6 - Low Power Comparator Reset."]
    #[inline(always)]
    pub fn lpcomp(&mut self) -> LpcompW<RstSpec> {
        LpcompW::new(self, 6)
    }
}
#[doc = "Low Power Reset Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstSpec;
impl crate::RegisterSpec for RstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst::R`](R) reader structure"]
impl crate::Readable for RstSpec {}
#[doc = "`write(|w| ..)` method takes [`rst::W`](W) writer structure"]
impl crate::Writable for RstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST to value 0"]
impl crate::Resettable for RstSpec {
    const RESET_VALUE: u32 = 0;
}
