#[doc = "Register `PCLKDIS0` reader"]
pub type R = crate::R<Pclkdis0Spec>;
#[doc = "Register `PCLKDIS0` writer"]
pub type W = crate::W<Pclkdis0Spec>;
#[doc = "GPIO0 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio0 {
    #[doc = "0: enable it."]
    En = 0,
    #[doc = "1: disable it."]
    Dis = 1,
}
impl From<Gpio0> for bool {
    #[inline(always)]
    fn from(variant: Gpio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO0` reader - GPIO0 Clock Disable."]
pub type Gpio0R = crate::BitReader<Gpio0>;
impl Gpio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0 {
        match self.bits {
            false => Gpio0::En,
            true => Gpio0::Dis,
        }
    }
    #[doc = "enable it."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Gpio0::En
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Gpio0::Dis
    }
}
#[doc = "Field `GPIO0` writer - GPIO0 Clock Disable."]
pub type Gpio0W<'a, REG> = crate::BitWriter<'a, REG, Gpio0>;
impl<'a, REG> Gpio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable it."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0::En)
    }
    #[doc = "disable it."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0::Dis)
    }
}
#[doc = "GPIO1 Clock Disable."]
pub use Gpio0 as Gpio1;
#[doc = "DMA Clock Disable."]
pub use Gpio0 as Dma;
#[doc = "SPI 1 Clock Disable."]
pub use Gpio0 as Spi1;
#[doc = "UART 0 Clock Disable."]
pub use Gpio0 as Uart0;
#[doc = "UART 1 Clock Disable."]
pub use Gpio0 as Uart1;
#[doc = "I2C 0 Clock Disable."]
pub use Gpio0 as I2c0;
#[doc = "Timer 0 Clock Disable."]
pub use Gpio0 as Tmr0;
#[doc = "Timer 1 Clock Disable."]
pub use Gpio0 as Tmr1;
#[doc = "Timer 2 Clock Disable."]
pub use Gpio0 as Tmr2;
#[doc = "Timer 3 Clock Disable."]
pub use Gpio0 as Tmr3;
#[doc = "ADC Clock Disable."]
pub use Gpio0 as Adc;
#[doc = "CNN Clock Disable."]
pub use Gpio0 as Cnn;
#[doc = "I2C 1 Clock Disable."]
pub use Gpio0 as I2c1;
#[doc = "Pluse Train Clock Disable."]
pub use Gpio0 as Pt;
#[doc = "Field `GPIO1` reader - GPIO1 Clock Disable."]
pub use Gpio0R as Gpio1R;
#[doc = "Field `DMA` reader - DMA Clock Disable."]
pub use Gpio0R as DmaR;
#[doc = "Field `SPI1` reader - SPI 1 Clock Disable."]
pub use Gpio0R as Spi1R;
#[doc = "Field `UART0` reader - UART 0 Clock Disable."]
pub use Gpio0R as Uart0R;
#[doc = "Field `UART1` reader - UART 1 Clock Disable."]
pub use Gpio0R as Uart1R;
#[doc = "Field `I2C0` reader - I2C 0 Clock Disable."]
pub use Gpio0R as I2c0R;
#[doc = "Field `TMR0` reader - Timer 0 Clock Disable."]
pub use Gpio0R as Tmr0R;
#[doc = "Field `TMR1` reader - Timer 1 Clock Disable."]
pub use Gpio0R as Tmr1R;
#[doc = "Field `TMR2` reader - Timer 2 Clock Disable."]
pub use Gpio0R as Tmr2R;
#[doc = "Field `TMR3` reader - Timer 3 Clock Disable."]
pub use Gpio0R as Tmr3R;
#[doc = "Field `ADC` reader - ADC Clock Disable."]
pub use Gpio0R as AdcR;
#[doc = "Field `CNN` reader - CNN Clock Disable."]
pub use Gpio0R as CnnR;
#[doc = "Field `I2C1` reader - I2C 1 Clock Disable."]
pub use Gpio0R as I2c1R;
#[doc = "Field `PT` reader - Pluse Train Clock Disable."]
pub use Gpio0R as PtR;
#[doc = "Field `GPIO1` writer - GPIO1 Clock Disable."]
pub use Gpio0W as Gpio1W;
#[doc = "Field `DMA` writer - DMA Clock Disable."]
pub use Gpio0W as DmaW;
#[doc = "Field `SPI1` writer - SPI 1 Clock Disable."]
pub use Gpio0W as Spi1W;
#[doc = "Field `UART0` writer - UART 0 Clock Disable."]
pub use Gpio0W as Uart0W;
#[doc = "Field `UART1` writer - UART 1 Clock Disable."]
pub use Gpio0W as Uart1W;
#[doc = "Field `I2C0` writer - I2C 0 Clock Disable."]
pub use Gpio0W as I2c0W;
#[doc = "Field `TMR0` writer - Timer 0 Clock Disable."]
pub use Gpio0W as Tmr0W;
#[doc = "Field `TMR1` writer - Timer 1 Clock Disable."]
pub use Gpio0W as Tmr1W;
#[doc = "Field `TMR2` writer - Timer 2 Clock Disable."]
pub use Gpio0W as Tmr2W;
#[doc = "Field `TMR3` writer - Timer 3 Clock Disable."]
pub use Gpio0W as Tmr3W;
#[doc = "Field `ADC` writer - ADC Clock Disable."]
pub use Gpio0W as AdcW;
#[doc = "Field `CNN` writer - CNN Clock Disable."]
pub use Gpio0W as CnnW;
#[doc = "Field `I2C1` writer - I2C 1 Clock Disable."]
pub use Gpio0W as I2c1W;
#[doc = "Field `PT` writer - Pluse Train Clock Disable."]
pub use Gpio0W as PtW;
impl R {
    #[doc = "Bit 0 - GPIO0 Clock Disable."]
    #[inline(always)]
    pub fn gpio0(&self) -> Gpio0R {
        Gpio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO1 Clock Disable."]
    #[inline(always)]
    pub fn gpio1(&self) -> Gpio1R {
        Gpio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Clock Disable."]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI 1 Clock Disable."]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - UART 0 Clock Disable."]
    #[inline(always)]
    pub fn uart0(&self) -> Uart0R {
        Uart0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART 1 Clock Disable."]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - I2C 0 Clock Disable."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Timer 0 Clock Disable."]
    #[inline(always)]
    pub fn tmr0(&self) -> Tmr0R {
        Tmr0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timer 1 Clock Disable."]
    #[inline(always)]
    pub fn tmr1(&self) -> Tmr1R {
        Tmr1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer 2 Clock Disable."]
    #[inline(always)]
    pub fn tmr2(&self) -> Tmr2R {
        Tmr2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer 3 Clock Disable."]
    #[inline(always)]
    pub fn tmr3(&self) -> Tmr3R {
        Tmr3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 23 - ADC Clock Disable."]
    #[inline(always)]
    pub fn adc(&self) -> AdcR {
        AdcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CNN Clock Disable."]
    #[inline(always)]
    pub fn cnn(&self) -> CnnR {
        CnnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - I2C 1 Clock Disable."]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Pluse Train Clock Disable."]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO0 Clock Disable."]
    #[inline(always)]
    pub fn gpio0(&mut self) -> Gpio0W<Pclkdis0Spec> {
        Gpio0W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO1 Clock Disable."]
    #[inline(always)]
    pub fn gpio1(&mut self) -> Gpio1W<Pclkdis0Spec> {
        Gpio1W::new(self, 1)
    }
    #[doc = "Bit 5 - DMA Clock Disable."]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<Pclkdis0Spec> {
        DmaW::new(self, 5)
    }
    #[doc = "Bit 6 - SPI 1 Clock Disable."]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<Pclkdis0Spec> {
        Spi1W::new(self, 6)
    }
    #[doc = "Bit 9 - UART 0 Clock Disable."]
    #[inline(always)]
    pub fn uart0(&mut self) -> Uart0W<Pclkdis0Spec> {
        Uart0W::new(self, 9)
    }
    #[doc = "Bit 10 - UART 1 Clock Disable."]
    #[inline(always)]
    pub fn uart1(&mut self) -> Uart1W<Pclkdis0Spec> {
        Uart1W::new(self, 10)
    }
    #[doc = "Bit 13 - I2C 0 Clock Disable."]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2c0W<Pclkdis0Spec> {
        I2c0W::new(self, 13)
    }
    #[doc = "Bit 15 - Timer 0 Clock Disable."]
    #[inline(always)]
    pub fn tmr0(&mut self) -> Tmr0W<Pclkdis0Spec> {
        Tmr0W::new(self, 15)
    }
    #[doc = "Bit 16 - Timer 1 Clock Disable."]
    #[inline(always)]
    pub fn tmr1(&mut self) -> Tmr1W<Pclkdis0Spec> {
        Tmr1W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer 2 Clock Disable."]
    #[inline(always)]
    pub fn tmr2(&mut self) -> Tmr2W<Pclkdis0Spec> {
        Tmr2W::new(self, 17)
    }
    #[doc = "Bit 18 - Timer 3 Clock Disable."]
    #[inline(always)]
    pub fn tmr3(&mut self) -> Tmr3W<Pclkdis0Spec> {
        Tmr3W::new(self, 18)
    }
    #[doc = "Bit 23 - ADC Clock Disable."]
    #[inline(always)]
    pub fn adc(&mut self) -> AdcW<Pclkdis0Spec> {
        AdcW::new(self, 23)
    }
    #[doc = "Bit 25 - CNN Clock Disable."]
    #[inline(always)]
    pub fn cnn(&mut self) -> CnnW<Pclkdis0Spec> {
        CnnW::new(self, 25)
    }
    #[doc = "Bit 28 - I2C 1 Clock Disable."]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2c1W<Pclkdis0Spec> {
        I2c1W::new(self, 28)
    }
    #[doc = "Bit 29 - Pluse Train Clock Disable."]
    #[inline(always)]
    pub fn pt(&mut self) -> PtW<Pclkdis0Spec> {
        PtW::new(self, 29)
    }
}
#[doc = "Peripheral Clock Disable.\n\nYou can [`read`](crate::Reg::read) this register and get [`pclkdis0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pclkdis0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pclkdis0Spec;
impl crate::RegisterSpec for Pclkdis0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclkdis0::R`](R) reader structure"]
impl crate::Readable for Pclkdis0Spec {}
#[doc = "`write(|w| ..)` method takes [`pclkdis0::W`](W) writer structure"]
impl crate::Writable for Pclkdis0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCLKDIS0 to value 0"]
impl crate::Resettable for Pclkdis0Spec {
    const RESET_VALUE: u32 = 0;
}
