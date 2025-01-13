#[doc = "Register `PCLKDIS` reader"]
pub type R = crate::R<PclkdisSpec>;
#[doc = "Register `PCLKDIS` writer"]
pub type W = crate::W<PclkdisSpec>;
#[doc = "Low Power GPIO 2 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio2 {
    #[doc = "0: enable it."]
    En = 0,
    #[doc = "1: disable it."]
    Dis = 1,
}
impl From<Gpio2> for bool {
    #[inline(always)]
    fn from(variant: Gpio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO2` reader - Low Power GPIO 2 Clock Disable."]
pub type Gpio2R = crate::BitReader<Gpio2>;
impl Gpio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2 {
        match self.bits {
            false => Gpio2::En,
            true => Gpio2::Dis,
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Gpio2::En
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Gpio2::Dis
    }
}
#[doc = "Field `GPIO2` writer - Low Power GPIO 2 Clock Disable."]
pub type Gpio2W<'a, REG> = crate::BitWriter<'a, REG, Gpio2>;
impl<'a, REG> Gpio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2::En)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2::Dis)
    }
}
#[doc = "Low Power Watchdog 1 Clock Disable."]
pub use Gpio2 as Wdt1;
#[doc = "Low Power Timer 4 Clock Disable."]
pub use Gpio2 as Tmr4;
#[doc = "Low Power Timer 5 Clock Disable."]
pub use Gpio2 as Tmr5;
#[doc = "Low Power UART 3 Clock Disable."]
pub use Gpio2 as Uart3;
#[doc = "Low Power Comparator Clock Disable."]
pub use Gpio2 as Lpcomp;
#[doc = "Field `WDT1` reader - Low Power Watchdog 1 Clock Disable."]
pub use Gpio2R as Wdt1R;
#[doc = "Field `TMR4` reader - Low Power Timer 4 Clock Disable."]
pub use Gpio2R as Tmr4R;
#[doc = "Field `TMR5` reader - Low Power Timer 5 Clock Disable."]
pub use Gpio2R as Tmr5R;
#[doc = "Field `UART3` reader - Low Power UART 3 Clock Disable."]
pub use Gpio2R as Uart3R;
#[doc = "Field `LPCOMP` reader - Low Power Comparator Clock Disable."]
pub use Gpio2R as LpcompR;
#[doc = "Field `WDT1` writer - Low Power Watchdog 1 Clock Disable."]
pub use Gpio2W as Wdt1W;
#[doc = "Field `TMR4` writer - Low Power Timer 4 Clock Disable."]
pub use Gpio2W as Tmr4W;
#[doc = "Field `TMR5` writer - Low Power Timer 5 Clock Disable."]
pub use Gpio2W as Tmr5W;
#[doc = "Field `UART3` writer - Low Power UART 3 Clock Disable."]
pub use Gpio2W as Uart3W;
#[doc = "Field `LPCOMP` writer - Low Power Comparator Clock Disable."]
pub use Gpio2W as LpcompW;
impl R {
    #[doc = "Bit 0 - Low Power GPIO 2 Clock Disable."]
    #[inline(always)]
    pub fn gpio2(&self) -> Gpio2R {
        Gpio2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Power Watchdog 1 Clock Disable."]
    #[inline(always)]
    pub fn wdt1(&self) -> Wdt1R {
        Wdt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low Power Timer 4 Clock Disable."]
    #[inline(always)]
    pub fn tmr4(&self) -> Tmr4R {
        Tmr4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low Power Timer 5 Clock Disable."]
    #[inline(always)]
    pub fn tmr5(&self) -> Tmr5R {
        Tmr5R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Power UART 3 Clock Disable."]
    #[inline(always)]
    pub fn uart3(&self) -> Uart3R {
        Uart3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Low Power Comparator Clock Disable."]
    #[inline(always)]
    pub fn lpcomp(&self) -> LpcompR {
        LpcompR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power GPIO 2 Clock Disable."]
    #[inline(always)]
    pub fn gpio2(&mut self) -> Gpio2W<PclkdisSpec> {
        Gpio2W::new(self, 0)
    }
    #[doc = "Bit 1 - Low Power Watchdog 1 Clock Disable."]
    #[inline(always)]
    pub fn wdt1(&mut self) -> Wdt1W<PclkdisSpec> {
        Wdt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Low Power Timer 4 Clock Disable."]
    #[inline(always)]
    pub fn tmr4(&mut self) -> Tmr4W<PclkdisSpec> {
        Tmr4W::new(self, 2)
    }
    #[doc = "Bit 3 - Low Power Timer 5 Clock Disable."]
    #[inline(always)]
    pub fn tmr5(&mut self) -> Tmr5W<PclkdisSpec> {
        Tmr5W::new(self, 3)
    }
    #[doc = "Bit 4 - Low Power UART 3 Clock Disable."]
    #[inline(always)]
    pub fn uart3(&mut self) -> Uart3W<PclkdisSpec> {
        Uart3W::new(self, 4)
    }
    #[doc = "Bit 6 - Low Power Comparator Clock Disable."]
    #[inline(always)]
    pub fn lpcomp(&mut self) -> LpcompW<PclkdisSpec> {
        LpcompW::new(self, 6)
    }
}
#[doc = "Low Power Peripheral Clock Disable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pclkdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pclkdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PclkdisSpec;
impl crate::RegisterSpec for PclkdisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclkdis::R`](R) reader structure"]
impl crate::Readable for PclkdisSpec {}
#[doc = "`write(|w| ..)` method takes [`pclkdis::W`](W) writer structure"]
impl crate::Writable for PclkdisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCLKDIS to value 0"]
impl crate::Resettable for PclkdisSpec {
    const RESET_VALUE: u32 = 0;
}
