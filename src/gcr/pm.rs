#[doc = "Register `PM` reader"]
pub type R = crate::R<PmSpec>;
#[doc = "Register `PM` writer"]
pub type W = crate::W<PmSpec>;
#[doc = "Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Active Mode."]
    Active = 0,
    #[doc = "1: Cortex-M4 Active, RISC-V Sleep Mode."]
    Sleep = 1,
    #[doc = "2: Standby Mode."]
    Standby = 2,
    #[doc = "4: Backup Mode."]
    Backup = 4,
    #[doc = "8: LPM or CM4 Deep Sleep Mode."]
    Lpm = 8,
    #[doc = "9: UPM."]
    Upm = 9,
    #[doc = "10: Power Down Mode."]
    Powerdown = 10,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Active),
            1 => Some(Mode::Sleep),
            2 => Some(Mode::Standby),
            4 => Some(Mode::Backup),
            8 => Some(Mode::Lpm),
            9 => Some(Mode::Upm),
            10 => Some(Mode::Powerdown),
            _ => None,
        }
    }
    #[doc = "Active Mode."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Mode::Active
    }
    #[doc = "Cortex-M4 Active, RISC-V Sleep Mode."]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == Mode::Sleep
    }
    #[doc = "Standby Mode."]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == Mode::Standby
    }
    #[doc = "Backup Mode."]
    #[inline(always)]
    pub fn is_backup(&self) -> bool {
        *self == Mode::Backup
    }
    #[doc = "LPM or CM4 Deep Sleep Mode."]
    #[inline(always)]
    pub fn is_lpm(&self) -> bool {
        *self == Mode::Lpm
    }
    #[doc = "UPM."]
    #[inline(always)]
    pub fn is_upm(&self) -> bool {
        *self == Mode::Upm
    }
    #[doc = "Power Down Mode."]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == Mode::Powerdown
    }
}
#[doc = "Field `MODE` writer - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Active Mode."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Active)
    }
    #[doc = "Cortex-M4 Active, RISC-V Sleep Mode."]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Sleep)
    }
    #[doc = "Standby Mode."]
    #[inline(always)]
    pub fn standby(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Standby)
    }
    #[doc = "Backup Mode."]
    #[inline(always)]
    pub fn backup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Backup)
    }
    #[doc = "LPM or CM4 Deep Sleep Mode."]
    #[inline(always)]
    pub fn lpm(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Lpm)
    }
    #[doc = "UPM."]
    #[inline(always)]
    pub fn upm(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Upm)
    }
    #[doc = "Power Down Mode."]
    #[inline(always)]
    pub fn powerdown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Powerdown)
    }
}
#[doc = "GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioWe {
    #[doc = "0: Wake Up Disable."]
    Dis = 0,
    #[doc = "1: Wake Up Enable."]
    En = 1,
}
impl From<GpioWe> for bool {
    #[inline(always)]
    fn from(variant: GpioWe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_WE` reader - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
pub type GpioWeR = crate::BitReader<GpioWe>;
impl GpioWeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioWe {
        match self.bits {
            false => GpioWe::Dis,
            true => GpioWe::En,
        }
    }
    #[doc = "Wake Up Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GpioWe::Dis
    }
    #[doc = "Wake Up Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GpioWe::En
    }
}
#[doc = "Field `GPIO_WE` writer - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
pub type GpioWeW<'a, REG> = crate::BitWriter<'a, REG, GpioWe>;
impl<'a, REG> GpioWeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake Up Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(GpioWe::Dis)
    }
    #[doc = "Wake Up Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(GpioWe::En)
    }
}
#[doc = "RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
pub use GpioWe as RtcWe;
#[doc = "WUT Wake Up Enable. This bit enables the Wake-Up Timer as wakeup source."]
pub use GpioWe as WutWe;
#[doc = "AIN COMP Wake Up Enable. This bit enables AIN COMP as wakeup source."]
pub use GpioWe as AincompWe;
#[doc = "Field `RTC_WE` reader - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
pub use GpioWeR as RtcWeR;
#[doc = "Field `WUT_WE` reader - WUT Wake Up Enable. This bit enables the Wake-Up Timer as wakeup source."]
pub use GpioWeR as WutWeR;
#[doc = "Field `AINCOMP_WE` reader - AIN COMP Wake Up Enable. This bit enables AIN COMP as wakeup source."]
pub use GpioWeR as AincompWeR;
#[doc = "Field `RTC_WE` writer - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
pub use GpioWeW as RtcWeW;
#[doc = "Field `WUT_WE` writer - WUT Wake Up Enable. This bit enables the Wake-Up Timer as wakeup source."]
pub use GpioWeW as WutWeW;
#[doc = "Field `AINCOMP_WE` writer - AIN COMP Wake Up Enable. This bit enables AIN COMP as wakeup source."]
pub use GpioWeW as AincompWeW;
#[doc = "60 MHz power down. This bit selects the 60 MHz clock power state in DEEPSLEEP mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsoPd {
    #[doc = "0: Mode is Active."]
    Active = 0,
    #[doc = "1: Powered down in DEEPSLEEP."]
    Deepsleep = 1,
}
impl From<IsoPd> for bool {
    #[inline(always)]
    fn from(variant: IsoPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISO_PD` reader - 60 MHz power down. This bit selects the 60 MHz clock power state in DEEPSLEEP mode."]
pub type IsoPdR = crate::BitReader<IsoPd>;
impl IsoPdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IsoPd {
        match self.bits {
            false => IsoPd::Active,
            true => IsoPd::Deepsleep,
        }
    }
    #[doc = "Mode is Active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == IsoPd::Active
    }
    #[doc = "Powered down in DEEPSLEEP."]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == IsoPd::Deepsleep
    }
}
#[doc = "Field `ISO_PD` writer - 60 MHz power down. This bit selects the 60 MHz clock power state in DEEPSLEEP mode."]
pub type IsoPdW<'a, REG> = crate::BitWriter<'a, REG, IsoPd>;
impl<'a, REG> IsoPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mode is Active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(IsoPd::Active)
    }
    #[doc = "Powered down in DEEPSLEEP."]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut crate::W<REG> {
        self.variant(IsoPd::Deepsleep)
    }
}
#[doc = "100 MHz power down. This bit selects 100 MHz clock power state in DEEPSLEEP mode."]
pub use IsoPd as IpoPd;
#[doc = "7.3725 MHz power down. This bit selects 7.3725 MHz clock power state in DEEPSLEEP mode."]
pub use IsoPd as IbroPd;
#[doc = "Field `IPO_PD` reader - 100 MHz power down. This bit selects 100 MHz clock power state in DEEPSLEEP mode."]
pub use IsoPdR as IpoPdR;
#[doc = "Field `IBRO_PD` reader - 7.3725 MHz power down. This bit selects 7.3725 MHz clock power state in DEEPSLEEP mode."]
pub use IsoPdR as IbroPdR;
#[doc = "Field `IPO_PD` writer - 100 MHz power down. This bit selects 100 MHz clock power state in DEEPSLEEP mode."]
pub use IsoPdW as IpoPdW;
#[doc = "Field `IBRO_PD` writer - 7.3725 MHz power down. This bit selects 7.3725 MHz clock power state in DEEPSLEEP mode."]
pub use IsoPdW as IbroPdW;
impl R {
    #[doc = "Bits 0:3 - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
    #[inline(always)]
    pub fn gpio_we(&self) -> GpioWeR {
        GpioWeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
    #[inline(always)]
    pub fn rtc_we(&self) -> RtcWeR {
        RtcWeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - WUT Wake Up Enable. This bit enables the Wake-Up Timer as wakeup source."]
    #[inline(always)]
    pub fn wut_we(&self) -> WutWeR {
        WutWeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - AIN COMP Wake Up Enable. This bit enables AIN COMP as wakeup source."]
    #[inline(always)]
    pub fn aincomp_we(&self) -> AincompWeR {
        AincompWeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - 60 MHz power down. This bit selects the 60 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn iso_pd(&self) -> IsoPdR {
        IsoPdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 100 MHz power down. This bit selects 100 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn ipo_pd(&self) -> IpoPdR {
        IpoPdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 7.3725 MHz power down. This bit selects 7.3725 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn ibro_pd(&self) -> IbroPdR {
        IbroPdR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Operating Mode. This two bit field selects the current operating mode for the device. Note that code execution only occurs during ACTIVE mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<PmSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 4 - GPIO Wake Up Enable. This bit enables all GPIO pins as potential wakeup sources. Any GPIO configured for wakeup is capable of causing an exit from IDLE or STANDBY modes when this bit is set."]
    #[inline(always)]
    pub fn gpio_we(&mut self) -> GpioWeW<PmSpec> {
        GpioWeW::new(self, 4)
    }
    #[doc = "Bit 5 - RTC Alarm Wake Up Enable. This bit enables RTC alarm as wakeup source. If enabled, the desired RTC alarm must be configured via the RTC control registers."]
    #[inline(always)]
    pub fn rtc_we(&mut self) -> RtcWeW<PmSpec> {
        RtcWeW::new(self, 5)
    }
    #[doc = "Bit 7 - WUT Wake Up Enable. This bit enables the Wake-Up Timer as wakeup source."]
    #[inline(always)]
    pub fn wut_we(&mut self) -> WutWeW<PmSpec> {
        WutWeW::new(self, 7)
    }
    #[doc = "Bit 9 - AIN COMP Wake Up Enable. This bit enables AIN COMP as wakeup source."]
    #[inline(always)]
    pub fn aincomp_we(&mut self) -> AincompWeW<PmSpec> {
        AincompWeW::new(self, 9)
    }
    #[doc = "Bit 15 - 60 MHz power down. This bit selects the 60 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn iso_pd(&mut self) -> IsoPdW<PmSpec> {
        IsoPdW::new(self, 15)
    }
    #[doc = "Bit 16 - 100 MHz power down. This bit selects 100 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn ipo_pd(&mut self) -> IpoPdW<PmSpec> {
        IpoPdW::new(self, 16)
    }
    #[doc = "Bit 17 - 7.3725 MHz power down. This bit selects 7.3725 MHz clock power state in DEEPSLEEP mode."]
    #[inline(always)]
    pub fn ibro_pd(&mut self) -> IbroPdW<PmSpec> {
        IbroPdW::new(self, 17)
    }
}
#[doc = "Power Management.\n\nYou can [`read`](crate::Reg::read) this register and get [`pm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmSpec;
impl crate::RegisterSpec for PmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pm::R`](R) reader structure"]
impl crate::Readable for PmSpec {}
#[doc = "`write(|w| ..)` method takes [`pm::W`](W) writer structure"]
impl crate::Writable for PmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PM to value 0"]
impl crate::Resettable for PmSpec {
    const RESET_VALUE: u32 = 0;
}
