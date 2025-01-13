#[doc = "Register `RST0` reader"]
pub type R = crate::R<Rst0Spec>;
#[doc = "Register `RST0` writer"]
pub type W = crate::W<Rst0Spec>;
#[doc = "DMA Reset.\n\nValue on reset: 0"]
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
#[doc = "Field `DMA` reader - DMA Reset."]
pub type DmaR = crate::BitReader<Reset>;
impl DmaR {
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
#[doc = "Field `DMA` writer - DMA Reset."]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG, Reset>;
impl<'a, REG> DmaW<'a, REG>
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
#[doc = "Field `WDT0` reader - Watchdog Timer 0 Reset."]
pub use DmaR as Wdt0R;
#[doc = "Field `GPIO0` reader - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
pub use DmaR as Gpio0R;
#[doc = "Field `GPIO1` reader - GPIO1 Reset. Setting this bit to 1 resets GPIO1 pins to their default states."]
pub use DmaR as Gpio1R;
#[doc = "Field `TMR0` reader - Timer 0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
pub use DmaR as Tmr0R;
#[doc = "Field `TMR1` reader - Timer 1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
pub use DmaR as Tmr1R;
#[doc = "Field `TMR2` reader - Timer 2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
pub use DmaR as Tmr2R;
#[doc = "Field `TMR3` reader - Timer 3 Reset. Setting this bit to 1 resets Timer 3 blocks."]
pub use DmaR as Tmr3R;
#[doc = "Field `UART0` reader - UART 0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
pub use DmaR as Uart0R;
#[doc = "Field `UART1` reader - UART 1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
pub use DmaR as Uart1R;
#[doc = "Field `SPI1` reader - SPI 1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
pub use DmaR as Spi1R;
#[doc = "Field `I2C0` reader - I2C 0 Reset."]
pub use DmaR as I2c0R;
#[doc = "Field `RTC` reader - Real Time Clock Reset."]
pub use DmaR as RtcR;
#[doc = "Field `SMPHR` reader - Semaphore Reset."]
pub use DmaR as SmphrR;
#[doc = "Field `TRNG` reader - TRNG Reset. This reset is only available during the manufacture testing phase."]
pub use DmaR as TrngR;
#[doc = "Field `CNN` reader - CNN Reset."]
pub use DmaR as CnnR;
#[doc = "Field `ADC` reader - ADC Reset."]
pub use DmaR as AdcR;
#[doc = "Field `UART2` reader - UART2 Reset. Setting this bit to 1 resets all UART 2 blocks."]
pub use DmaR as Uart2R;
#[doc = "Field `SOFT` reader - Soft Reset. Setting this bit to 1 resets everything except the CPU and the watchdog timer."]
pub use DmaR as SoftR;
#[doc = "Field `PERIPH` reader - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
pub use DmaR as PeriphR;
#[doc = "Field `SYS` reader - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
pub use DmaR as SysR;
#[doc = "Field `WDT0` writer - Watchdog Timer 0 Reset."]
pub use DmaW as Wdt0W;
#[doc = "Field `GPIO0` writer - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
pub use DmaW as Gpio0W;
#[doc = "Field `GPIO1` writer - GPIO1 Reset. Setting this bit to 1 resets GPIO1 pins to their default states."]
pub use DmaW as Gpio1W;
#[doc = "Field `TMR0` writer - Timer 0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
pub use DmaW as Tmr0W;
#[doc = "Field `TMR1` writer - Timer 1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
pub use DmaW as Tmr1W;
#[doc = "Field `TMR2` writer - Timer 2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
pub use DmaW as Tmr2W;
#[doc = "Field `TMR3` writer - Timer 3 Reset. Setting this bit to 1 resets Timer 3 blocks."]
pub use DmaW as Tmr3W;
#[doc = "Field `UART0` writer - UART 0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
pub use DmaW as Uart0W;
#[doc = "Field `UART1` writer - UART 1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
pub use DmaW as Uart1W;
#[doc = "Field `SPI1` writer - SPI 1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
pub use DmaW as Spi1W;
#[doc = "Field `I2C0` writer - I2C 0 Reset."]
pub use DmaW as I2c0W;
#[doc = "Field `RTC` writer - Real Time Clock Reset."]
pub use DmaW as RtcW;
#[doc = "Field `SMPHR` writer - Semaphore Reset."]
pub use DmaW as SmphrW;
#[doc = "Field `TRNG` writer - TRNG Reset. This reset is only available during the manufacture testing phase."]
pub use DmaW as TrngW;
#[doc = "Field `CNN` writer - CNN Reset."]
pub use DmaW as CnnW;
#[doc = "Field `ADC` writer - ADC Reset."]
pub use DmaW as AdcW;
#[doc = "Field `UART2` writer - UART2 Reset. Setting this bit to 1 resets all UART 2 blocks."]
pub use DmaW as Uart2W;
#[doc = "Field `SOFT` writer - Soft Reset. Setting this bit to 1 resets everything except the CPU and the watchdog timer."]
pub use DmaW as SoftW;
#[doc = "Field `PERIPH` writer - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
pub use DmaW as PeriphW;
#[doc = "Field `SYS` writer - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
pub use DmaW as SysW;
impl R {
    #[doc = "Bit 0 - DMA Reset."]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Timer 0 Reset."]
    #[inline(always)]
    pub fn wdt0(&self) -> Wdt0R {
        Wdt0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
    #[inline(always)]
    pub fn gpio0(&self) -> Gpio0R {
        Gpio0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO1 Reset. Setting this bit to 1 resets GPIO1 pins to their default states."]
    #[inline(always)]
    pub fn gpio1(&self) -> Gpio1R {
        Gpio1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
    #[inline(always)]
    pub fn tmr0(&self) -> Tmr0R {
        Tmr0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer 1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
    #[inline(always)]
    pub fn tmr1(&self) -> Tmr1R {
        Tmr1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer 2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
    #[inline(always)]
    pub fn tmr2(&self) -> Tmr2R {
        Tmr2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer 3 Reset. Setting this bit to 1 resets Timer 3 blocks."]
    #[inline(always)]
    pub fn tmr3(&self) -> Tmr3R {
        Tmr3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - UART 0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
    #[inline(always)]
    pub fn uart0(&self) -> Uart0R {
        Uart0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UART 1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI 1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C 0 Reset."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Real Time Clock Reset."]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 22 - Semaphore Reset."]
    #[inline(always)]
    pub fn smphr(&self) -> SmphrR {
        SmphrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - TRNG Reset. This reset is only available during the manufacture testing phase."]
    #[inline(always)]
    pub fn trng(&self) -> TrngR {
        TrngR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CNN Reset."]
    #[inline(always)]
    pub fn cnn(&self) -> CnnR {
        CnnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ADC Reset."]
    #[inline(always)]
    pub fn adc(&self) -> AdcR {
        AdcR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - UART2 Reset. Setting this bit to 1 resets all UART 2 blocks."]
    #[inline(always)]
    pub fn uart2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Soft Reset. Setting this bit to 1 resets everything except the CPU and the watchdog timer."]
    #[inline(always)]
    pub fn soft(&self) -> SoftR {
        SoftR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
    #[inline(always)]
    pub fn periph(&self) -> PeriphR {
        PeriphR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
    #[inline(always)]
    pub fn sys(&self) -> SysR {
        SysR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Reset."]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<Rst0Spec> {
        DmaW::new(self, 0)
    }
    #[doc = "Bit 1 - Watchdog Timer 0 Reset."]
    #[inline(always)]
    pub fn wdt0(&mut self) -> Wdt0W<Rst0Spec> {
        Wdt0W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO0 Reset. Setting this bit to 1 resets GPIO0 pins to their default states."]
    #[inline(always)]
    pub fn gpio0(&mut self) -> Gpio0W<Rst0Spec> {
        Gpio0W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO1 Reset. Setting this bit to 1 resets GPIO1 pins to their default states."]
    #[inline(always)]
    pub fn gpio1(&mut self) -> Gpio1W<Rst0Spec> {
        Gpio1W::new(self, 3)
    }
    #[doc = "Bit 5 - Timer 0 Reset. Setting this bit to 1 resets Timer 0 blocks."]
    #[inline(always)]
    pub fn tmr0(&mut self) -> Tmr0W<Rst0Spec> {
        Tmr0W::new(self, 5)
    }
    #[doc = "Bit 6 - Timer 1 Reset. Setting this bit to 1 resets Timer 1 blocks."]
    #[inline(always)]
    pub fn tmr1(&mut self) -> Tmr1W<Rst0Spec> {
        Tmr1W::new(self, 6)
    }
    #[doc = "Bit 7 - Timer 2 Reset. Setting this bit to 1 resets Timer 2 blocks."]
    #[inline(always)]
    pub fn tmr2(&mut self) -> Tmr2W<Rst0Spec> {
        Tmr2W::new(self, 7)
    }
    #[doc = "Bit 8 - Timer 3 Reset. Setting this bit to 1 resets Timer 3 blocks."]
    #[inline(always)]
    pub fn tmr3(&mut self) -> Tmr3W<Rst0Spec> {
        Tmr3W::new(self, 8)
    }
    #[doc = "Bit 11 - UART 0 Reset. Setting this bit to 1 resets all UART 0 blocks."]
    #[inline(always)]
    pub fn uart0(&mut self) -> Uart0W<Rst0Spec> {
        Uart0W::new(self, 11)
    }
    #[doc = "Bit 12 - UART 1 Reset. Setting this bit to 1 resets all UART 1 blocks."]
    #[inline(always)]
    pub fn uart1(&mut self) -> Uart1W<Rst0Spec> {
        Uart1W::new(self, 12)
    }
    #[doc = "Bit 13 - SPI 1 Reset. Setting this bit to 1 resets all SPI 1 blocks."]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<Rst0Spec> {
        Spi1W::new(self, 13)
    }
    #[doc = "Bit 16 - I2C 0 Reset."]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2c0W<Rst0Spec> {
        I2c0W::new(self, 16)
    }
    #[doc = "Bit 17 - Real Time Clock Reset."]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<Rst0Spec> {
        RtcW::new(self, 17)
    }
    #[doc = "Bit 22 - Semaphore Reset."]
    #[inline(always)]
    pub fn smphr(&mut self) -> SmphrW<Rst0Spec> {
        SmphrW::new(self, 22)
    }
    #[doc = "Bit 24 - TRNG Reset. This reset is only available during the manufacture testing phase."]
    #[inline(always)]
    pub fn trng(&mut self) -> TrngW<Rst0Spec> {
        TrngW::new(self, 24)
    }
    #[doc = "Bit 25 - CNN Reset."]
    #[inline(always)]
    pub fn cnn(&mut self) -> CnnW<Rst0Spec> {
        CnnW::new(self, 25)
    }
    #[doc = "Bit 26 - ADC Reset."]
    #[inline(always)]
    pub fn adc(&mut self) -> AdcW<Rst0Spec> {
        AdcW::new(self, 26)
    }
    #[doc = "Bit 28 - UART2 Reset. Setting this bit to 1 resets all UART 2 blocks."]
    #[inline(always)]
    pub fn uart2(&mut self) -> Uart2W<Rst0Spec> {
        Uart2W::new(self, 28)
    }
    #[doc = "Bit 29 - Soft Reset. Setting this bit to 1 resets everything except the CPU and the watchdog timer."]
    #[inline(always)]
    pub fn soft(&mut self) -> SoftW<Rst0Spec> {
        SoftW::new(self, 29)
    }
    #[doc = "Bit 30 - Peripheral Reset. Setting this bit to 1 resets all peripherals. The CPU core, the watchdog timer, and all GPIO pins are unaffected by this reset."]
    #[inline(always)]
    pub fn periph(&mut self) -> PeriphW<Rst0Spec> {
        PeriphW::new(self, 30)
    }
    #[doc = "Bit 31 - System Reset. Setting this bit to 1 resets the CPU core and all peripherals, including the watchdog timer."]
    #[inline(always)]
    pub fn sys(&mut self) -> SysW<Rst0Spec> {
        SysW::new(self, 31)
    }
}
#[doc = "Reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`rst0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rst0Spec;
impl crate::RegisterSpec for Rst0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst0::R`](R) reader structure"]
impl crate::Readable for Rst0Spec {}
#[doc = "`write(|w| ..)` method takes [`rst0::W`](W) writer structure"]
impl crate::Writable for Rst0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST0 to value 0"]
impl crate::Resettable for Rst0Spec {
    const RESET_VALUE: u32 = 0;
}
