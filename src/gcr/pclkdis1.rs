#[doc = "Register `PCLKDIS1` reader"]
pub type R = crate::R<Pclkdis1Spec>;
#[doc = "Register `PCLKDIS1` writer"]
pub type W = crate::W<Pclkdis1Spec>;
#[doc = "UART2 Clock Disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart2 {
    #[doc = "0: Enable."]
    En = 0,
    #[doc = "1: Disable."]
    Dis = 1,
}
impl From<Uart2> for bool {
    #[inline(always)]
    fn from(variant: Uart2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART2` reader - UART2 Clock Disable."]
pub type Uart2R = crate::BitReader<Uart2>;
impl Uart2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart2 {
        match self.bits {
            false => Uart2::En,
            true => Uart2::Dis,
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Uart2::En
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Uart2::Dis
    }
}
#[doc = "Field `UART2` writer - UART2 Clock Disable."]
pub type Uart2W<'a, REG> = crate::BitWriter<'a, REG, Uart2>;
impl<'a, REG> Uart2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Uart2::En)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Uart2::Dis)
    }
}
#[doc = "TRNG Clock Disable."]
pub use Uart2 as Trng;
#[doc = "SMPHR Clock Disable."]
pub use Uart2 as Smphr;
#[doc = "One-Wire Clock Disable."]
pub use Uart2 as Owm;
#[doc = "CRC Clock Disable."]
pub use Uart2 as Crc;
#[doc = "AES Clock Disable."]
pub use Uart2 as Aes;
#[doc = "SPI 0 Clock Disable."]
pub use Uart2 as Spi0;
#[doc = "Parallel Camera Interface Clock Disable."]
pub use Uart2 as Pcif;
#[doc = "I2S Clock Disable."]
pub use Uart2 as I2s;
#[doc = "I2C2 Clock Disable."]
pub use Uart2 as I2c2;
#[doc = "Watch Dog Timer 0 Clock Disable."]
pub use Uart2 as Wdt0;
#[doc = "CPU1 Clock Disable."]
pub use Uart2 as Cpu1;
#[doc = "Field `TRNG` reader - TRNG Clock Disable."]
pub use Uart2R as TrngR;
#[doc = "Field `SMPHR` reader - SMPHR Clock Disable."]
pub use Uart2R as SmphrR;
#[doc = "Field `OWM` reader - One-Wire Clock Disable."]
pub use Uart2R as OwmR;
#[doc = "Field `CRC` reader - CRC Clock Disable."]
pub use Uart2R as CrcR;
#[doc = "Field `AES` reader - AES Clock Disable."]
pub use Uart2R as AesR;
#[doc = "Field `SPI0` reader - SPI 0 Clock Disable."]
pub use Uart2R as Spi0R;
#[doc = "Field `PCIF` reader - Parallel Camera Interface Clock Disable."]
pub use Uart2R as PcifR;
#[doc = "Field `I2S` reader - I2S Clock Disable."]
pub use Uart2R as I2sR;
#[doc = "Field `I2C2` reader - I2C2 Clock Disable."]
pub use Uart2R as I2c2R;
#[doc = "Field `WDT0` reader - Watch Dog Timer 0 Clock Disable."]
pub use Uart2R as Wdt0R;
#[doc = "Field `CPU1` reader - CPU1 Clock Disable."]
pub use Uart2R as Cpu1R;
#[doc = "Field `TRNG` writer - TRNG Clock Disable."]
pub use Uart2W as TrngW;
#[doc = "Field `SMPHR` writer - SMPHR Clock Disable."]
pub use Uart2W as SmphrW;
#[doc = "Field `OWM` writer - One-Wire Clock Disable."]
pub use Uart2W as OwmW;
#[doc = "Field `CRC` writer - CRC Clock Disable."]
pub use Uart2W as CrcW;
#[doc = "Field `AES` writer - AES Clock Disable."]
pub use Uart2W as AesW;
#[doc = "Field `SPI0` writer - SPI 0 Clock Disable."]
pub use Uart2W as Spi0W;
#[doc = "Field `PCIF` writer - Parallel Camera Interface Clock Disable."]
pub use Uart2W as PcifW;
#[doc = "Field `I2S` writer - I2S Clock Disable."]
pub use Uart2W as I2sW;
#[doc = "Field `I2C2` writer - I2C2 Clock Disable."]
pub use Uart2W as I2c2W;
#[doc = "Field `WDT0` writer - Watch Dog Timer 0 Clock Disable."]
pub use Uart2W as Wdt0W;
#[doc = "Field `CPU1` writer - CPU1 Clock Disable."]
pub use Uart2W as Cpu1W;
impl R {
    #[doc = "Bit 1 - UART2 Clock Disable."]
    #[inline(always)]
    pub fn uart2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TRNG Clock Disable."]
    #[inline(always)]
    pub fn trng(&self) -> TrngR {
        TrngR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - SMPHR Clock Disable."]
    #[inline(always)]
    pub fn smphr(&self) -> SmphrR {
        SmphrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - One-Wire Clock Disable."]
    #[inline(always)]
    pub fn owm(&self) -> OwmR {
        OwmR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CRC Clock Disable."]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AES Clock Disable."]
    #[inline(always)]
    pub fn aes(&self) -> AesR {
        AesR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI 0 Clock Disable."]
    #[inline(always)]
    pub fn spi0(&self) -> Spi0R {
        Spi0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Parallel Camera Interface Clock Disable."]
    #[inline(always)]
    pub fn pcif(&self) -> PcifR {
        PcifR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 23 - I2S Clock Disable."]
    #[inline(always)]
    pub fn i2s(&self) -> I2sR {
        I2sR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I2C2 Clock Disable."]
    #[inline(always)]
    pub fn i2c2(&self) -> I2c2R {
        I2c2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - Watch Dog Timer 0 Clock Disable."]
    #[inline(always)]
    pub fn wdt0(&self) -> Wdt0R {
        Wdt0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU1 Clock Disable."]
    #[inline(always)]
    pub fn cpu1(&self) -> Cpu1R {
        Cpu1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - UART2 Clock Disable."]
    #[inline(always)]
    pub fn uart2(&mut self) -> Uart2W<Pclkdis1Spec> {
        Uart2W::new(self, 1)
    }
    #[doc = "Bit 2 - TRNG Clock Disable."]
    #[inline(always)]
    pub fn trng(&mut self) -> TrngW<Pclkdis1Spec> {
        TrngW::new(self, 2)
    }
    #[doc = "Bit 9 - SMPHR Clock Disable."]
    #[inline(always)]
    pub fn smphr(&mut self) -> SmphrW<Pclkdis1Spec> {
        SmphrW::new(self, 9)
    }
    #[doc = "Bit 13 - One-Wire Clock Disable."]
    #[inline(always)]
    pub fn owm(&mut self) -> OwmW<Pclkdis1Spec> {
        OwmW::new(self, 13)
    }
    #[doc = "Bit 14 - CRC Clock Disable."]
    #[inline(always)]
    pub fn crc(&mut self) -> CrcW<Pclkdis1Spec> {
        CrcW::new(self, 14)
    }
    #[doc = "Bit 15 - AES Clock Disable."]
    #[inline(always)]
    pub fn aes(&mut self) -> AesW<Pclkdis1Spec> {
        AesW::new(self, 15)
    }
    #[doc = "Bit 16 - SPI 0 Clock Disable."]
    #[inline(always)]
    pub fn spi0(&mut self) -> Spi0W<Pclkdis1Spec> {
        Spi0W::new(self, 16)
    }
    #[doc = "Bit 18 - Parallel Camera Interface Clock Disable."]
    #[inline(always)]
    pub fn pcif(&mut self) -> PcifW<Pclkdis1Spec> {
        PcifW::new(self, 18)
    }
    #[doc = "Bit 23 - I2S Clock Disable."]
    #[inline(always)]
    pub fn i2s(&mut self) -> I2sW<Pclkdis1Spec> {
        I2sW::new(self, 23)
    }
    #[doc = "Bit 24 - I2C2 Clock Disable."]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2c2W<Pclkdis1Spec> {
        I2c2W::new(self, 24)
    }
    #[doc = "Bit 27 - Watch Dog Timer 0 Clock Disable."]
    #[inline(always)]
    pub fn wdt0(&mut self) -> Wdt0W<Pclkdis1Spec> {
        Wdt0W::new(self, 27)
    }
    #[doc = "Bit 31 - CPU1 Clock Disable."]
    #[inline(always)]
    pub fn cpu1(&mut self) -> Cpu1W<Pclkdis1Spec> {
        Cpu1W::new(self, 31)
    }
}
#[doc = "Peripheral Clock Disable.\n\nYou can [`read`](crate::Reg::read) this register and get [`pclkdis1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pclkdis1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pclkdis1Spec;
impl crate::RegisterSpec for Pclkdis1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pclkdis1::R`](R) reader structure"]
impl crate::Readable for Pclkdis1Spec {}
#[doc = "`write(|w| ..)` method takes [`pclkdis1::W`](W) writer structure"]
impl crate::Writable for Pclkdis1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCLKDIS1 to value 0"]
impl crate::Resettable for Pclkdis1Spec {
    const RESET_VALUE: u32 = 0;
}
