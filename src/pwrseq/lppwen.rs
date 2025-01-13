#[doc = "Register `LPPWEN` reader"]
pub type R = crate::R<LppwenSpec>;
#[doc = "Register `LPPWEN` writer"]
pub type W = crate::W<LppwenSpec>;
#[doc = "Field `AINCOMP0` reader - AINCOMP0 Wakeup Enable. This bit allows wakeup from the AINCOMP0."]
pub type Aincomp0R = crate::BitReader;
#[doc = "Field `AINCOMP0` writer - AINCOMP0 Wakeup Enable. This bit allows wakeup from the AINCOMP0."]
pub type Aincomp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT0` reader - WDT0 Wakeup Enable. This bit allows wakeup from the WDT0."]
pub type Wdt0R = crate::BitReader;
#[doc = "Field `WDT0` writer - WDT0 Wakeup Enable. This bit allows wakeup from the WDT0."]
pub type Wdt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT1` reader - WDT1 Wakeup Enable. This bit allows wakeup from the WDT1."]
pub type Wdt1R = crate::BitReader;
#[doc = "Field `WDT1` writer - WDT1 Wakeup Enable. This bit allows wakeup from the WDT1."]
pub type Wdt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU1` reader - CPU1 Wakeup Enable. This bit allows wakeup from the CPU1."]
pub type Cpu1R = crate::BitReader;
#[doc = "Field `CPU1` writer - CPU1 Wakeup Enable. This bit allows wakeup from the CPU1."]
pub type Cpu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR0` reader - TMR0 Wakeup Enable. This bit allows wakeup from the TMR0."]
pub type Tmr0R = crate::BitReader;
#[doc = "Field `TMR0` writer - TMR0 Wakeup Enable. This bit allows wakeup from the TMR0."]
pub type Tmr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1` reader - TMR1 Wakeup Enable. This bit allows wakeup from the TMR1."]
pub type Tmr1R = crate::BitReader;
#[doc = "Field `TMR1` writer - TMR1 Wakeup Enable. This bit allows wakeup from the TMR1."]
pub type Tmr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR2` reader - TMR2 Wakeup Enable. This bit allows wakeup from the TMR2."]
pub type Tmr2R = crate::BitReader;
#[doc = "Field `TMR2` writer - TMR2 Wakeup Enable. This bit allows wakeup from the TMR2."]
pub type Tmr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3` reader - TMR3 Wakeup Enable. This bit allows wakeup from the TMR3."]
pub type Tmr3R = crate::BitReader;
#[doc = "Field `TMR3` writer - TMR3 Wakeup Enable. This bit allows wakeup from the TMR3."]
pub type Tmr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4` reader - TMR4 Wakeup Enable. This bit allows wakeup from the TMR4."]
pub type Tmr4R = crate::BitReader;
#[doc = "Field `TMR4` writer - TMR4 Wakeup Enable. This bit allows wakeup from the TMR4."]
pub type Tmr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR5` reader - TMR5 Wakeup Enable. This bit allows wakeup from the TMR5."]
pub type Tmr5R = crate::BitReader;
#[doc = "Field `TMR5` writer - TMR5 Wakeup Enable. This bit allows wakeup from the TMR5."]
pub type Tmr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0` reader - UART0 Wakeup Enable. This bit allows wakeup from the UART0."]
pub type Uart0R = crate::BitReader;
#[doc = "Field `UART0` writer - UART0 Wakeup Enable. This bit allows wakeup from the UART0."]
pub type Uart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - UART1 Wakeup Enable. This bit allows wakeup from the UART1."]
pub type Uart1R = crate::BitReader;
#[doc = "Field `UART1` writer - UART1 Wakeup Enable. This bit allows wakeup from the UART1."]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2` reader - UART2 Wakeup Enable. This bit allows wakeup from the UART2."]
pub type Uart2R = crate::BitReader;
#[doc = "Field `UART2` writer - UART2 Wakeup Enable. This bit allows wakeup from the UART2."]
pub type Uart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3` reader - UART3 Wakeup Enable. This bit allows wakeup from the UART3."]
pub type Uart3R = crate::BitReader;
#[doc = "Field `UART3` writer - UART3 Wakeup Enable. This bit allows wakeup from the UART3."]
pub type Uart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0` reader - I2C0 Wakeup Enable. This bit allows wakeup from the I2C0."]
pub type I2c0R = crate::BitReader;
#[doc = "Field `I2C0` writer - I2C0 Wakeup Enable. This bit allows wakeup from the I2C0."]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - I2C1 Wakeup Enable. This bit allows wakeup from the I2C1."]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C1 Wakeup Enable. This bit allows wakeup from the I2C1."]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2` reader - I2C2 Wakeup Enable. This bit allows wakeup from the I2C2."]
pub type I2c2R = crate::BitReader;
#[doc = "Field `I2C2` writer - I2C2 Wakeup Enable. This bit allows wakeup from the I2C2."]
pub type I2c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S` reader - I2S Wakeup Enable. This bit allows wakeup from the I2S."]
pub type I2sR = crate::BitReader;
#[doc = "Field `I2S` writer - I2S Wakeup Enable. This bit allows wakeup from the I2S."]
pub type I2sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - SPI1 Wakeup Enable. This bit allows wakeup from the SPI1."]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI1` writer - SPI1 Wakeup Enable. This bit allows wakeup from the SPI1."]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCMP` reader - LPCMP Wakeup Enable. This bit allows wakeup from the LPCMP."]
pub type LpcmpR = crate::BitReader;
#[doc = "Field `LPCMP` writer - LPCMP Wakeup Enable. This bit allows wakeup from the LPCMP."]
pub type LpcmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - AINCOMP0 Wakeup Enable. This bit allows wakeup from the AINCOMP0."]
    #[inline(always)]
    pub fn aincomp0(&self) -> Aincomp0R {
        Aincomp0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - WDT0 Wakeup Enable. This bit allows wakeup from the WDT0."]
    #[inline(always)]
    pub fn wdt0(&self) -> Wdt0R {
        Wdt0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WDT1 Wakeup Enable. This bit allows wakeup from the WDT1."]
    #[inline(always)]
    pub fn wdt1(&self) -> Wdt1R {
        Wdt1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU1 Wakeup Enable. This bit allows wakeup from the CPU1."]
    #[inline(always)]
    pub fn cpu1(&self) -> Cpu1R {
        Cpu1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TMR0 Wakeup Enable. This bit allows wakeup from the TMR0."]
    #[inline(always)]
    pub fn tmr0(&self) -> Tmr0R {
        Tmr0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TMR1 Wakeup Enable. This bit allows wakeup from the TMR1."]
    #[inline(always)]
    pub fn tmr1(&self) -> Tmr1R {
        Tmr1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TMR2 Wakeup Enable. This bit allows wakeup from the TMR2."]
    #[inline(always)]
    pub fn tmr2(&self) -> Tmr2R {
        Tmr2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TMR3 Wakeup Enable. This bit allows wakeup from the TMR3."]
    #[inline(always)]
    pub fn tmr3(&self) -> Tmr3R {
        Tmr3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TMR4 Wakeup Enable. This bit allows wakeup from the TMR4."]
    #[inline(always)]
    pub fn tmr4(&self) -> Tmr4R {
        Tmr4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TMR5 Wakeup Enable. This bit allows wakeup from the TMR5."]
    #[inline(always)]
    pub fn tmr5(&self) -> Tmr5R {
        Tmr5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - UART0 Wakeup Enable. This bit allows wakeup from the UART0."]
    #[inline(always)]
    pub fn uart0(&self) -> Uart0R {
        Uart0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - UART1 Wakeup Enable. This bit allows wakeup from the UART1."]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART2 Wakeup Enable. This bit allows wakeup from the UART2."]
    #[inline(always)]
    pub fn uart2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART3 Wakeup Enable. This bit allows wakeup from the UART3."]
    #[inline(always)]
    pub fn uart3(&self) -> Uart3R {
        Uart3R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 Wakeup Enable. This bit allows wakeup from the I2C0."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 Wakeup Enable. This bit allows wakeup from the I2C1."]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C2 Wakeup Enable. This bit allows wakeup from the I2C2."]
    #[inline(always)]
    pub fn i2c2(&self) -> I2c2R {
        I2c2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I2S Wakeup Enable. This bit allows wakeup from the I2S."]
    #[inline(always)]
    pub fn i2s(&self) -> I2sR {
        I2sR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SPI1 Wakeup Enable. This bit allows wakeup from the SPI1."]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LPCMP Wakeup Enable. This bit allows wakeup from the LPCMP."]
    #[inline(always)]
    pub fn lpcmp(&self) -> LpcmpR {
        LpcmpR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - AINCOMP0 Wakeup Enable. This bit allows wakeup from the AINCOMP0."]
    #[inline(always)]
    pub fn aincomp0(&mut self) -> Aincomp0W<LppwenSpec> {
        Aincomp0W::new(self, 4)
    }
    #[doc = "Bit 8 - WDT0 Wakeup Enable. This bit allows wakeup from the WDT0."]
    #[inline(always)]
    pub fn wdt0(&mut self) -> Wdt0W<LppwenSpec> {
        Wdt0W::new(self, 8)
    }
    #[doc = "Bit 9 - WDT1 Wakeup Enable. This bit allows wakeup from the WDT1."]
    #[inline(always)]
    pub fn wdt1(&mut self) -> Wdt1W<LppwenSpec> {
        Wdt1W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU1 Wakeup Enable. This bit allows wakeup from the CPU1."]
    #[inline(always)]
    pub fn cpu1(&mut self) -> Cpu1W<LppwenSpec> {
        Cpu1W::new(self, 10)
    }
    #[doc = "Bit 11 - TMR0 Wakeup Enable. This bit allows wakeup from the TMR0."]
    #[inline(always)]
    pub fn tmr0(&mut self) -> Tmr0W<LppwenSpec> {
        Tmr0W::new(self, 11)
    }
    #[doc = "Bit 12 - TMR1 Wakeup Enable. This bit allows wakeup from the TMR1."]
    #[inline(always)]
    pub fn tmr1(&mut self) -> Tmr1W<LppwenSpec> {
        Tmr1W::new(self, 12)
    }
    #[doc = "Bit 13 - TMR2 Wakeup Enable. This bit allows wakeup from the TMR2."]
    #[inline(always)]
    pub fn tmr2(&mut self) -> Tmr2W<LppwenSpec> {
        Tmr2W::new(self, 13)
    }
    #[doc = "Bit 14 - TMR3 Wakeup Enable. This bit allows wakeup from the TMR3."]
    #[inline(always)]
    pub fn tmr3(&mut self) -> Tmr3W<LppwenSpec> {
        Tmr3W::new(self, 14)
    }
    #[doc = "Bit 15 - TMR4 Wakeup Enable. This bit allows wakeup from the TMR4."]
    #[inline(always)]
    pub fn tmr4(&mut self) -> Tmr4W<LppwenSpec> {
        Tmr4W::new(self, 15)
    }
    #[doc = "Bit 16 - TMR5 Wakeup Enable. This bit allows wakeup from the TMR5."]
    #[inline(always)]
    pub fn tmr5(&mut self) -> Tmr5W<LppwenSpec> {
        Tmr5W::new(self, 16)
    }
    #[doc = "Bit 17 - UART0 Wakeup Enable. This bit allows wakeup from the UART0."]
    #[inline(always)]
    pub fn uart0(&mut self) -> Uart0W<LppwenSpec> {
        Uart0W::new(self, 17)
    }
    #[doc = "Bit 18 - UART1 Wakeup Enable. This bit allows wakeup from the UART1."]
    #[inline(always)]
    pub fn uart1(&mut self) -> Uart1W<LppwenSpec> {
        Uart1W::new(self, 18)
    }
    #[doc = "Bit 19 - UART2 Wakeup Enable. This bit allows wakeup from the UART2."]
    #[inline(always)]
    pub fn uart2(&mut self) -> Uart2W<LppwenSpec> {
        Uart2W::new(self, 19)
    }
    #[doc = "Bit 20 - UART3 Wakeup Enable. This bit allows wakeup from the UART3."]
    #[inline(always)]
    pub fn uart3(&mut self) -> Uart3W<LppwenSpec> {
        Uart3W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C0 Wakeup Enable. This bit allows wakeup from the I2C0."]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2c0W<LppwenSpec> {
        I2c0W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C1 Wakeup Enable. This bit allows wakeup from the I2C1."]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2c1W<LppwenSpec> {
        I2c1W::new(self, 22)
    }
    #[doc = "Bit 23 - I2C2 Wakeup Enable. This bit allows wakeup from the I2C2."]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2c2W<LppwenSpec> {
        I2c2W::new(self, 23)
    }
    #[doc = "Bit 24 - I2S Wakeup Enable. This bit allows wakeup from the I2S."]
    #[inline(always)]
    pub fn i2s(&mut self) -> I2sW<LppwenSpec> {
        I2sW::new(self, 24)
    }
    #[doc = "Bit 25 - SPI1 Wakeup Enable. This bit allows wakeup from the SPI1."]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<LppwenSpec> {
        Spi1W::new(self, 25)
    }
    #[doc = "Bit 26 - LPCMP Wakeup Enable. This bit allows wakeup from the LPCMP."]
    #[inline(always)]
    pub fn lpcmp(&mut self) -> LpcmpW<LppwenSpec> {
        LpcmpW::new(self, 26)
    }
}
#[doc = "Low Power Peripheral Wakeup Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lppwen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lppwen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LppwenSpec;
impl crate::RegisterSpec for LppwenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lppwen::R`](R) reader structure"]
impl crate::Readable for LppwenSpec {}
#[doc = "`write(|w| ..)` method takes [`lppwen::W`](W) writer structure"]
impl crate::Writable for LppwenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPPWEN to value 0"]
impl crate::Resettable for LppwenSpec {
    const RESET_VALUE: u32 = 0;
}
