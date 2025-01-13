#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Real Time Clock Enable. This bit enables the Real Time Clock. This bit can only be written when WE=1 and BUSY =0. Change to this bit is effective only after BUSY is cleared from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Real Time Clock Enable. This bit enables the Real Time Clock. This bit can only be written when WE=1 and BUSY =0. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Dis,
            true => En::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En::En
    }
}
#[doc = "Field `EN` writer - Real Time Clock Enable. This bit enables the Real Time Clock. This bit can only be written when WE=1 and BUSY =0. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En::En)
    }
}
#[doc = "Alarm Time-of-Day Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TodAlarmIe {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<TodAlarmIe> for bool {
    #[inline(always)]
    fn from(variant: TodAlarmIe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOD_ALARM_IE` reader - Alarm Time-of-Day Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub type TodAlarmIeR = crate::BitReader<TodAlarmIe>;
impl TodAlarmIeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TodAlarmIe {
        match self.bits {
            false => TodAlarmIe::Dis,
            true => TodAlarmIe::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TodAlarmIe::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TodAlarmIe::En
    }
}
#[doc = "Field `TOD_ALARM_IE` writer - Alarm Time-of-Day Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub type TodAlarmIeW<'a, REG> = crate::BitWriter<'a, REG, TodAlarmIe>;
impl<'a, REG> TodAlarmIeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(TodAlarmIe::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(TodAlarmIe::En)
    }
}
#[doc = "Alarm Sub-second Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsecAlarmIe {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<SsecAlarmIe> for bool {
    #[inline(always)]
    fn from(variant: SsecAlarmIe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSEC_ALARM_IE` reader - Alarm Sub-second Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub type SsecAlarmIeR = crate::BitReader<SsecAlarmIe>;
impl SsecAlarmIeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsecAlarmIe {
        match self.bits {
            false => SsecAlarmIe::Dis,
            true => SsecAlarmIe::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SsecAlarmIe::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SsecAlarmIe::En
    }
}
#[doc = "Field `SSEC_ALARM_IE` writer - Alarm Sub-second Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
pub type SsecAlarmIeW<'a, REG> = crate::BitWriter<'a, REG, SsecAlarmIe>;
impl<'a, REG> SsecAlarmIeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(SsecAlarmIe::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(SsecAlarmIe::En)
    }
}
#[doc = "RTC Busy. This bit is set to 1 by hardware when changes to RTC registers required a synchronized version of the register to be in place. This bit is automatically cleared by hardware.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: Idle."]
    Idle = 0,
    #[doc = "1: Busy."]
    Busy = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - RTC Busy. This bit is set to 1 by hardware when changes to RTC registers required a synchronized version of the register to be in place. This bit is automatically cleared by hardware."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Idle,
            true => Busy::Busy,
        }
    }
    #[doc = "Idle."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Busy::Idle
    }
    #[doc = "Busy."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Busy::Busy
    }
}
#[doc = "RTC Ready. This bit is set to 1 by hardware when the RTC count registers update. It can be cleared to 0 by software at any time. It will also be cleared to 0 by hardware just prior to an update of the RTC count register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdy {
    #[doc = "0: Register has not updated."]
    Busy = 0,
    #[doc = "1: Ready."]
    Ready = 1,
}
impl From<Rdy> for bool {
    #[inline(always)]
    fn from(variant: Rdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY` reader - RTC Ready. This bit is set to 1 by hardware when the RTC count registers update. It can be cleared to 0 by software at any time. It will also be cleared to 0 by hardware just prior to an update of the RTC count register."]
pub type RdyR = crate::BitReader<Rdy>;
impl RdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdy {
        match self.bits {
            false => Rdy::Busy,
            true => Rdy::Ready,
        }
    }
    #[doc = "Register has not updated."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Rdy::Busy
    }
    #[doc = "Ready."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Rdy::Ready
    }
}
#[doc = "Field `RDY` writer - RTC Ready. This bit is set to 1 by hardware when the RTC count registers update. It can be cleared to 0 by software at any time. It will also be cleared to 0 by hardware just prior to an update of the RTC count register."]
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG, Rdy>;
impl<'a, REG> RdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Register has not updated."]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(Rdy::Busy)
    }
    #[doc = "Ready."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(Rdy::Ready)
    }
}
#[doc = "RTC Ready Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RdyIe {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<RdyIe> for bool {
    #[inline(always)]
    fn from(variant: RdyIe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY_IE` reader - RTC Ready Interrupt Enable."]
pub type RdyIeR = crate::BitReader<RdyIe>;
impl RdyIeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RdyIe {
        match self.bits {
            false => RdyIe::Dis,
            true => RdyIe::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RdyIe::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RdyIe::En
    }
}
#[doc = "Field `RDY_IE` writer - RTC Ready Interrupt Enable."]
pub type RdyIeW<'a, REG> = crate::BitWriter<'a, REG, RdyIe>;
impl<'a, REG> RdyIeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RdyIe::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RdyIe::En)
    }
}
#[doc = "Time-of-Day Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TodAlarm {
    #[doc = "0: Not active"]
    Inactive = 0,
    #[doc = "1: Active"]
    Pending = 1,
}
impl From<TodAlarm> for bool {
    #[inline(always)]
    fn from(variant: TodAlarm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOD_ALARM` reader - Time-of-Day Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor."]
pub type TodAlarmR = crate::BitReader<TodAlarm>;
impl TodAlarmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TodAlarm {
        match self.bits {
            false => TodAlarm::Inactive,
            true => TodAlarm::Pending,
        }
    }
    #[doc = "Not active"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == TodAlarm::Inactive
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == TodAlarm::Pending
    }
}
#[doc = "Sub-second Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsecAlarm {
    #[doc = "0: Not active"]
    Inactive = 0,
    #[doc = "1: Active"]
    Pending = 1,
}
impl From<SsecAlarm> for bool {
    #[inline(always)]
    fn from(variant: SsecAlarm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSEC_ALARM` reader - Sub-second Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor."]
pub type SsecAlarmR = crate::BitReader<SsecAlarm>;
impl SsecAlarmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsecAlarm {
        match self.bits {
            false => SsecAlarm::Inactive,
            true => SsecAlarm::Pending,
        }
    }
    #[doc = "Not active"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == SsecAlarm::Inactive
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SsecAlarm::Pending
    }
}
#[doc = "Square Wave Output Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SqwEn {
    #[doc = "0: Not active"]
    Inactive = 0,
    #[doc = "1: Active"]
    Pending = 1,
}
impl From<SqwEn> for bool {
    #[inline(always)]
    fn from(variant: SqwEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SQW_EN` reader - Square Wave Output Enable."]
pub type SqwEnR = crate::BitReader<SqwEn>;
impl SqwEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SqwEn {
        match self.bits {
            false => SqwEn::Inactive,
            true => SqwEn::Pending,
        }
    }
    #[doc = "Not active"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == SqwEn::Inactive
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == SqwEn::Pending
    }
}
#[doc = "Field `SQW_EN` writer - Square Wave Output Enable."]
pub type SqwEnW<'a, REG> = crate::BitWriter<'a, REG, SqwEn>;
impl<'a, REG> SqwEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not active"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(SqwEn::Inactive)
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(SqwEn::Pending)
    }
}
#[doc = "Frequency Output Selection. When SQE=1, these bits specify the output frequency on the SQW pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SqwSel {
    #[doc = "0: 1 Hz (Compensated)."]
    Freq1hz = 0,
    #[doc = "1: 512 Hz (Compensated)."]
    Freq512hz = 1,
    #[doc = "2: 4 KHz."]
    Freq4khz = 2,
    #[doc = "3: RTC Input Clock / 8."]
    ClkDiv8 = 3,
}
impl From<SqwSel> for u8 {
    #[inline(always)]
    fn from(variant: SqwSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SqwSel {
    type Ux = u8;
}
impl crate::IsEnum for SqwSel {}
#[doc = "Field `SQW_SEL` reader - Frequency Output Selection. When SQE=1, these bits specify the output frequency on the SQW pin."]
pub type SqwSelR = crate::FieldReader<SqwSel>;
impl SqwSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SqwSel {
        match self.bits {
            0 => SqwSel::Freq1hz,
            1 => SqwSel::Freq512hz,
            2 => SqwSel::Freq4khz,
            3 => SqwSel::ClkDiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "1 Hz (Compensated)."]
    #[inline(always)]
    pub fn is_freq1hz(&self) -> bool {
        *self == SqwSel::Freq1hz
    }
    #[doc = "512 Hz (Compensated)."]
    #[inline(always)]
    pub fn is_freq512hz(&self) -> bool {
        *self == SqwSel::Freq512hz
    }
    #[doc = "4 KHz."]
    #[inline(always)]
    pub fn is_freq4khz(&self) -> bool {
        *self == SqwSel::Freq4khz
    }
    #[doc = "RTC Input Clock / 8."]
    #[inline(always)]
    pub fn is_clk_div8(&self) -> bool {
        *self == SqwSel::ClkDiv8
    }
}
#[doc = "Field `SQW_SEL` writer - Frequency Output Selection. When SQE=1, these bits specify the output frequency on the SQW pin."]
pub type SqwSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, SqwSel, crate::Safe>;
impl<'a, REG> SqwSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 Hz (Compensated)."]
    #[inline(always)]
    pub fn freq1hz(self) -> &'a mut crate::W<REG> {
        self.variant(SqwSel::Freq1hz)
    }
    #[doc = "512 Hz (Compensated)."]
    #[inline(always)]
    pub fn freq512hz(self) -> &'a mut crate::W<REG> {
        self.variant(SqwSel::Freq512hz)
    }
    #[doc = "4 KHz."]
    #[inline(always)]
    pub fn freq4khz(self) -> &'a mut crate::W<REG> {
        self.variant(SqwSel::Freq4khz)
    }
    #[doc = "RTC Input Clock / 8."]
    #[inline(always)]
    pub fn clk_div8(self) -> &'a mut crate::W<REG> {
        self.variant(SqwSel::ClkDiv8)
    }
}
#[doc = "Field `RD_EN` reader - Asynchronous Counter Read Enable."]
pub type RdEnR = crate::BitReader;
#[doc = "Field `RD_EN` writer - Asynchronous Counter Read Enable."]
pub type RdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write Enable. This register bit serves as a protection mechanism against unintentional writes to critical RTC bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WrEn {
    #[doc = "0: Not active"]
    Inactive = 0,
    #[doc = "1: Active"]
    Pending = 1,
}
impl From<WrEn> for bool {
    #[inline(always)]
    fn from(variant: WrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WR_EN` reader - Write Enable. This register bit serves as a protection mechanism against unintentional writes to critical RTC bits."]
pub type WrEnR = crate::BitReader<WrEn>;
impl WrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WrEn {
        match self.bits {
            false => WrEn::Inactive,
            true => WrEn::Pending,
        }
    }
    #[doc = "Not active"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == WrEn::Inactive
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == WrEn::Pending
    }
}
#[doc = "Field `WR_EN` writer - Write Enable. This register bit serves as a protection mechanism against unintentional writes to critical RTC bits."]
pub type WrEnW<'a, REG> = crate::BitWriter<'a, REG, WrEn>;
impl<'a, REG> WrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not active"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(WrEn::Inactive)
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(WrEn::Pending)
    }
}
impl R {
    #[doc = "Bit 0 - Real Time Clock Enable. This bit enables the Real Time Clock. This bit can only be written when WE=1 and BUSY =0. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm Time-of-Day Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    pub fn tod_alarm_ie(&self) -> TodAlarmIeR {
        TodAlarmIeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Alarm Sub-second Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    pub fn ssec_alarm_ie(&self) -> SsecAlarmIeR {
        SsecAlarmIeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC Busy. This bit is set to 1 by hardware when changes to RTC registers required a synchronized version of the register to be in place. This bit is automatically cleared by hardware."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Ready. This bit is set to 1 by hardware when the RTC count registers update. It can be cleared to 0 by software at any time. It will also be cleared to 0 by hardware just prior to an update of the RTC count register."]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC Ready Interrupt Enable."]
    #[inline(always)]
    pub fn rdy_ie(&self) -> RdyIeR {
        RdyIeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Time-of-Day Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor."]
    #[inline(always)]
    pub fn tod_alarm(&self) -> TodAlarmR {
        TodAlarmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sub-second Alarm Interrupt Flag. This alarm is qualified as wake-up source to the processor."]
    #[inline(always)]
    pub fn ssec_alarm(&self) -> SsecAlarmR {
        SsecAlarmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Square Wave Output Enable."]
    #[inline(always)]
    pub fn sqw_en(&self) -> SqwEnR {
        SqwEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Frequency Output Selection. When SQE=1, these bits specify the output frequency on the SQW pin."]
    #[inline(always)]
    pub fn sqw_sel(&self) -> SqwSelR {
        SqwSelR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 14 - Asynchronous Counter Read Enable."]
    #[inline(always)]
    pub fn rd_en(&self) -> RdEnR {
        RdEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write Enable. This register bit serves as a protection mechanism against unintentional writes to critical RTC bits."]
    #[inline(always)]
    pub fn wr_en(&self) -> WrEnR {
        WrEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real Time Clock Enable. This bit enables the Real Time Clock. This bit can only be written when WE=1 and BUSY =0. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm Time-of-Day Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    pub fn tod_alarm_ie(&mut self) -> TodAlarmIeW<CtrlSpec> {
        TodAlarmIeW::new(self, 1)
    }
    #[doc = "Bit 2 - Alarm Sub-second Interrupt Enable. Change to this bit is effective only after BUSY is cleared from 1 to 0."]
    #[inline(always)]
    pub fn ssec_alarm_ie(&mut self) -> SsecAlarmIeW<CtrlSpec> {
        SsecAlarmIeW::new(self, 2)
    }
    #[doc = "Bit 4 - RTC Ready. This bit is set to 1 by hardware when the RTC count registers update. It can be cleared to 0 by software at any time. It will also be cleared to 0 by hardware just prior to an update of the RTC count register."]
    #[inline(always)]
    pub fn rdy(&mut self) -> RdyW<CtrlSpec> {
        RdyW::new(self, 4)
    }
    #[doc = "Bit 5 - RTC Ready Interrupt Enable."]
    #[inline(always)]
    pub fn rdy_ie(&mut self) -> RdyIeW<CtrlSpec> {
        RdyIeW::new(self, 5)
    }
    #[doc = "Bit 8 - Square Wave Output Enable."]
    #[inline(always)]
    pub fn sqw_en(&mut self) -> SqwEnW<CtrlSpec> {
        SqwEnW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Frequency Output Selection. When SQE=1, these bits specify the output frequency on the SQW pin."]
    #[inline(always)]
    pub fn sqw_sel(&mut self) -> SqwSelW<CtrlSpec> {
        SqwSelW::new(self, 9)
    }
    #[doc = "Bit 14 - Asynchronous Counter Read Enable."]
    #[inline(always)]
    pub fn rd_en(&mut self) -> RdEnW<CtrlSpec> {
        RdEnW::new(self, 14)
    }
    #[doc = "Bit 15 - Write Enable. This register bit serves as a protection mechanism against unintentional writes to critical RTC bits."]
    #[inline(always)]
    pub fn wr_en(&mut self) -> WrEnW<CtrlSpec> {
        WrEnW::new(self, 15)
    }
}
#[doc = "RTC Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x08"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x08;
}
