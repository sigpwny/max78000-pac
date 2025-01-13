#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Windowed Watchdog Interrupt Upper Limit. Sets the number of WDTCLK cycles until a windowed watchdog timer interrupt is generated (if enabled) if the CPU does not write the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntLateVal {
    #[doc = "0: 2**31 clock cycles."]
    Wdt2pow31 = 0,
    #[doc = "1: 2**30 clock cycles."]
    Wdt2pow30 = 1,
    #[doc = "2: 2**29 clock cycles."]
    Wdt2pow29 = 2,
    #[doc = "3: 2**28 clock cycles."]
    Wdt2pow28 = 3,
    #[doc = "4: 2^27 clock cycles."]
    Wdt2pow27 = 4,
    #[doc = "5: 2**26 clock cycles."]
    Wdt2pow26 = 5,
    #[doc = "6: 2**25 clock cycles."]
    Wdt2pow25 = 6,
    #[doc = "7: 2**24 clock cycles."]
    Wdt2pow24 = 7,
    #[doc = "8: 2**23 clock cycles."]
    Wdt2pow23 = 8,
    #[doc = "9: 2**22 clock cycles."]
    Wdt2pow22 = 9,
    #[doc = "10: 2**21 clock cycles."]
    Wdt2pow21 = 10,
    #[doc = "11: 2**20 clock cycles."]
    Wdt2pow20 = 11,
    #[doc = "12: 2**19 clock cycles."]
    Wdt2pow19 = 12,
    #[doc = "13: 2**18 clock cycles."]
    Wdt2pow18 = 13,
    #[doc = "14: 2**17 clock cycles."]
    Wdt2pow17 = 14,
    #[doc = "15: 2**16 clock cycles."]
    Wdt2pow16 = 15,
}
impl From<IntLateVal> for u8 {
    #[inline(always)]
    fn from(variant: IntLateVal) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntLateVal {
    type Ux = u8;
}
impl crate::IsEnum for IntLateVal {}
#[doc = "Field `INT_LATE_VAL` reader - Windowed Watchdog Interrupt Upper Limit. Sets the number of WDTCLK cycles until a windowed watchdog timer interrupt is generated (if enabled) if the CPU does not write the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub type IntLateValR = crate::FieldReader<IntLateVal>;
impl IntLateValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntLateVal {
        match self.bits {
            0 => IntLateVal::Wdt2pow31,
            1 => IntLateVal::Wdt2pow30,
            2 => IntLateVal::Wdt2pow29,
            3 => IntLateVal::Wdt2pow28,
            4 => IntLateVal::Wdt2pow27,
            5 => IntLateVal::Wdt2pow26,
            6 => IntLateVal::Wdt2pow25,
            7 => IntLateVal::Wdt2pow24,
            8 => IntLateVal::Wdt2pow23,
            9 => IntLateVal::Wdt2pow22,
            10 => IntLateVal::Wdt2pow21,
            11 => IntLateVal::Wdt2pow20,
            12 => IntLateVal::Wdt2pow19,
            13 => IntLateVal::Wdt2pow18,
            14 => IntLateVal::Wdt2pow17,
            15 => IntLateVal::Wdt2pow16,
            _ => unreachable!(),
        }
    }
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow31(&self) -> bool {
        *self == IntLateVal::Wdt2pow31
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow30(&self) -> bool {
        *self == IntLateVal::Wdt2pow30
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow29(&self) -> bool {
        *self == IntLateVal::Wdt2pow29
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow28(&self) -> bool {
        *self == IntLateVal::Wdt2pow28
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow27(&self) -> bool {
        *self == IntLateVal::Wdt2pow27
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow26(&self) -> bool {
        *self == IntLateVal::Wdt2pow26
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow25(&self) -> bool {
        *self == IntLateVal::Wdt2pow25
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow24(&self) -> bool {
        *self == IntLateVal::Wdt2pow24
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow23(&self) -> bool {
        *self == IntLateVal::Wdt2pow23
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow22(&self) -> bool {
        *self == IntLateVal::Wdt2pow22
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow21(&self) -> bool {
        *self == IntLateVal::Wdt2pow21
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow20(&self) -> bool {
        *self == IntLateVal::Wdt2pow20
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow19(&self) -> bool {
        *self == IntLateVal::Wdt2pow19
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow18(&self) -> bool {
        *self == IntLateVal::Wdt2pow18
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow17(&self) -> bool {
        *self == IntLateVal::Wdt2pow17
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow16(&self) -> bool {
        *self == IntLateVal::Wdt2pow16
    }
}
#[doc = "Field `INT_LATE_VAL` writer - Windowed Watchdog Interrupt Upper Limit. Sets the number of WDTCLK cycles until a windowed watchdog timer interrupt is generated (if enabled) if the CPU does not write the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub type IntLateValW<'a, REG> = crate::FieldWriter<'a, REG, 4, IntLateVal, crate::Safe>;
impl<'a, REG> IntLateValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow31(self) -> &'a mut crate::W<REG> {
        self.variant(IntLateVal::Wdt2pow31)
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow30(self) -> &'a mut crate::W<REG> {
        self.variant(IntLateVal::Wdt2pow30)
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow29(self) -> &'a mut crate::W<REG> {
        self.variant(IntLateVal::Wdt2pow29)
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow28(self) -> &'a mut crate::W<REG> {
        self.variant(IntLateVal::Wdt2pow28)
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow27(self) -> &'a mut crate::W<REG> {
        self.variant(IntLateVal::Wdt2pow27)
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow26(self) -> &'a mut crate::W<REG> {
        self.variant(IntLateVal::Wdt2pow26)
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow25(self) -> &'a mut crate::W<REG> {
        self.variant(IntLateVal::Wdt2pow25)
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow24(self) -> &'a mut crate::W<REG> {
        self.variant(IntLateVal::Wdt2pow24)
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow23(self) -> &'a mut crate::W<REG> {
        self.variant(IntLateVal::Wdt2pow23)
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow22(self) -> &'a mut crate::W<REG> {
        self.variant(IntLateVal::Wdt2pow22)
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow21(self) -> &'a mut crate::W<REG> {
        self.variant(IntLateVal::Wdt2pow21)
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow20(self) -> &'a mut crate::W<REG> {
        self.variant(IntLateVal::Wdt2pow20)
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow19(self) -> &'a mut crate::W<REG> {
        self.variant(IntLateVal::Wdt2pow19)
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow18(self) -> &'a mut crate::W<REG> {
        self.variant(IntLateVal::Wdt2pow18)
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow17(self) -> &'a mut crate::W<REG> {
        self.variant(IntLateVal::Wdt2pow17)
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow16(self) -> &'a mut crate::W<REG> {
        self.variant(IntLateVal::Wdt2pow16)
    }
}
#[doc = "Windowed Watchdog Reset Upper Limit. Sets the number of WDTCLK cycles until a system reset occurs (if enabled) if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RstLateVal {
    #[doc = "0: 2**31 clock cycles."]
    Wdt2pow31 = 0,
    #[doc = "1: 2**30 clock cycles."]
    Wdt2pow30 = 1,
    #[doc = "2: 2**29 clock cycles."]
    Wdt2pow29 = 2,
    #[doc = "3: 2**28 clock cycles."]
    Wdt2pow28 = 3,
    #[doc = "4: 2^27 clock cycles."]
    Wdt2pow27 = 4,
    #[doc = "5: 2**26 clock cycles."]
    Wdt2pow26 = 5,
    #[doc = "6: 2**25 clock cycles."]
    Wdt2pow25 = 6,
    #[doc = "7: 2**24 clock cycles."]
    Wdt2pow24 = 7,
    #[doc = "8: 2**23 clock cycles."]
    Wdt2pow23 = 8,
    #[doc = "9: 2**22 clock cycles."]
    Wdt2pow22 = 9,
    #[doc = "10: 2**21 clock cycles."]
    Wdt2pow21 = 10,
    #[doc = "11: 2**20 clock cycles."]
    Wdt2pow20 = 11,
    #[doc = "12: 2**19 clock cycles."]
    Wdt2pow19 = 12,
    #[doc = "13: 2**18 clock cycles."]
    Wdt2pow18 = 13,
    #[doc = "14: 2**17 clock cycles."]
    Wdt2pow17 = 14,
    #[doc = "15: 2**16 clock cycles."]
    Wdt2pow16 = 15,
}
impl From<RstLateVal> for u8 {
    #[inline(always)]
    fn from(variant: RstLateVal) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RstLateVal {
    type Ux = u8;
}
impl crate::IsEnum for RstLateVal {}
#[doc = "Field `RST_LATE_VAL` reader - Windowed Watchdog Reset Upper Limit. Sets the number of WDTCLK cycles until a system reset occurs (if enabled) if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub type RstLateValR = crate::FieldReader<RstLateVal>;
impl RstLateValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstLateVal {
        match self.bits {
            0 => RstLateVal::Wdt2pow31,
            1 => RstLateVal::Wdt2pow30,
            2 => RstLateVal::Wdt2pow29,
            3 => RstLateVal::Wdt2pow28,
            4 => RstLateVal::Wdt2pow27,
            5 => RstLateVal::Wdt2pow26,
            6 => RstLateVal::Wdt2pow25,
            7 => RstLateVal::Wdt2pow24,
            8 => RstLateVal::Wdt2pow23,
            9 => RstLateVal::Wdt2pow22,
            10 => RstLateVal::Wdt2pow21,
            11 => RstLateVal::Wdt2pow20,
            12 => RstLateVal::Wdt2pow19,
            13 => RstLateVal::Wdt2pow18,
            14 => RstLateVal::Wdt2pow17,
            15 => RstLateVal::Wdt2pow16,
            _ => unreachable!(),
        }
    }
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow31(&self) -> bool {
        *self == RstLateVal::Wdt2pow31
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow30(&self) -> bool {
        *self == RstLateVal::Wdt2pow30
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow29(&self) -> bool {
        *self == RstLateVal::Wdt2pow29
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow28(&self) -> bool {
        *self == RstLateVal::Wdt2pow28
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow27(&self) -> bool {
        *self == RstLateVal::Wdt2pow27
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow26(&self) -> bool {
        *self == RstLateVal::Wdt2pow26
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow25(&self) -> bool {
        *self == RstLateVal::Wdt2pow25
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow24(&self) -> bool {
        *self == RstLateVal::Wdt2pow24
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow23(&self) -> bool {
        *self == RstLateVal::Wdt2pow23
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow22(&self) -> bool {
        *self == RstLateVal::Wdt2pow22
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow21(&self) -> bool {
        *self == RstLateVal::Wdt2pow21
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow20(&self) -> bool {
        *self == RstLateVal::Wdt2pow20
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow19(&self) -> bool {
        *self == RstLateVal::Wdt2pow19
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow18(&self) -> bool {
        *self == RstLateVal::Wdt2pow18
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow17(&self) -> bool {
        *self == RstLateVal::Wdt2pow17
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow16(&self) -> bool {
        *self == RstLateVal::Wdt2pow16
    }
}
#[doc = "Field `RST_LATE_VAL` writer - Windowed Watchdog Reset Upper Limit. Sets the number of WDTCLK cycles until a system reset occurs (if enabled) if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub type RstLateValW<'a, REG> = crate::FieldWriter<'a, REG, 4, RstLateVal, crate::Safe>;
impl<'a, REG> RstLateValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow31(self) -> &'a mut crate::W<REG> {
        self.variant(RstLateVal::Wdt2pow31)
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow30(self) -> &'a mut crate::W<REG> {
        self.variant(RstLateVal::Wdt2pow30)
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow29(self) -> &'a mut crate::W<REG> {
        self.variant(RstLateVal::Wdt2pow29)
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow28(self) -> &'a mut crate::W<REG> {
        self.variant(RstLateVal::Wdt2pow28)
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow27(self) -> &'a mut crate::W<REG> {
        self.variant(RstLateVal::Wdt2pow27)
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow26(self) -> &'a mut crate::W<REG> {
        self.variant(RstLateVal::Wdt2pow26)
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow25(self) -> &'a mut crate::W<REG> {
        self.variant(RstLateVal::Wdt2pow25)
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow24(self) -> &'a mut crate::W<REG> {
        self.variant(RstLateVal::Wdt2pow24)
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow23(self) -> &'a mut crate::W<REG> {
        self.variant(RstLateVal::Wdt2pow23)
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow22(self) -> &'a mut crate::W<REG> {
        self.variant(RstLateVal::Wdt2pow22)
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow21(self) -> &'a mut crate::W<REG> {
        self.variant(RstLateVal::Wdt2pow21)
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow20(self) -> &'a mut crate::W<REG> {
        self.variant(RstLateVal::Wdt2pow20)
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow19(self) -> &'a mut crate::W<REG> {
        self.variant(RstLateVal::Wdt2pow19)
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow18(self) -> &'a mut crate::W<REG> {
        self.variant(RstLateVal::Wdt2pow18)
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow17(self) -> &'a mut crate::W<REG> {
        self.variant(RstLateVal::Wdt2pow17)
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow16(self) -> &'a mut crate::W<REG> {
        self.variant(RstLateVal::Wdt2pow16)
    }
}
#[doc = "Windowed Watchdog Timer Enable.\n\nValue on reset: 0"]
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
#[doc = "Field `EN` reader - Windowed Watchdog Timer Enable."]
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
#[doc = "Field `EN` writer - Windowed Watchdog Timer Enable."]
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
#[doc = "Windowed Watchdog Timer Interrupt Flag Too Late.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntLate {
    #[doc = "0: No interrupt is pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<IntLate> for bool {
    #[inline(always)]
    fn from(variant: IntLate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_LATE` reader - Windowed Watchdog Timer Interrupt Flag Too Late."]
pub type IntLateR = crate::BitReader<IntLate>;
impl IntLateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntLate {
        match self.bits {
            false => IntLate::Inactive,
            true => IntLate::Pending,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IntLate::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IntLate::Pending
    }
}
#[doc = "Field `INT_LATE` writer - Windowed Watchdog Timer Interrupt Flag Too Late."]
pub type IntLateW<'a, REG> = crate::BitWriter<'a, REG, IntLate>;
impl<'a, REG> IntLateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(IntLate::Inactive)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntLate::Pending)
    }
}
#[doc = "Windowed Watchdog Timer Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdtIntEn {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<WdtIntEn> for bool {
    #[inline(always)]
    fn from(variant: WdtIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT_INT_EN` reader - Windowed Watchdog Timer Interrupt Enable."]
pub type WdtIntEnR = crate::BitReader<WdtIntEn>;
impl WdtIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtIntEn {
        match self.bits {
            false => WdtIntEn::Dis,
            true => WdtIntEn::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WdtIntEn::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WdtIntEn::En
    }
}
#[doc = "Field `WDT_INT_EN` writer - Windowed Watchdog Timer Interrupt Enable."]
pub type WdtIntEnW<'a, REG> = crate::BitWriter<'a, REG, WdtIntEn>;
impl<'a, REG> WdtIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(WdtIntEn::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(WdtIntEn::En)
    }
}
#[doc = "Windowed Watchdog Timer Reset Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdtRstEn {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<WdtRstEn> for bool {
    #[inline(always)]
    fn from(variant: WdtRstEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT_RST_EN` reader - Windowed Watchdog Timer Reset Enable."]
pub type WdtRstEnR = crate::BitReader<WdtRstEn>;
impl WdtRstEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtRstEn {
        match self.bits {
            false => WdtRstEn::Dis,
            true => WdtRstEn::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WdtRstEn::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WdtRstEn::En
    }
}
#[doc = "Field `WDT_RST_EN` writer - Windowed Watchdog Timer Reset Enable."]
pub type WdtRstEnW<'a, REG> = crate::BitWriter<'a, REG, WdtRstEn>;
impl<'a, REG> WdtRstEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(WdtRstEn::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(WdtRstEn::En)
    }
}
#[doc = "Windowed Watchdog Timer Interrupt Flag Too Soon.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEarly {
    #[doc = "0: No interrupt is pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<IntEarly> for bool {
    #[inline(always)]
    fn from(variant: IntEarly) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EARLY` reader - Windowed Watchdog Timer Interrupt Flag Too Soon."]
pub type IntEarlyR = crate::BitReader<IntEarly>;
impl IntEarlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEarly {
        match self.bits {
            false => IntEarly::Inactive,
            true => IntEarly::Pending,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IntEarly::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == IntEarly::Pending
    }
}
#[doc = "Field `INT_EARLY` writer - Windowed Watchdog Timer Interrupt Flag Too Soon."]
pub type IntEarlyW<'a, REG> = crate::BitWriter<'a, REG, IntEarly>;
impl<'a, REG> IntEarlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarly::Inactive)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarly::Pending)
    }
}
#[doc = "Windowed Watchdog Interrupt Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A windowed watchdog timer interrupt is generated (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntEarlyVal {
    #[doc = "0: 2**31 clock cycles."]
    Wdt2pow31 = 0,
    #[doc = "1: 2**30 clock cycles."]
    Wdt2pow30 = 1,
    #[doc = "2: 2**29 clock cycles."]
    Wdt2pow29 = 2,
    #[doc = "3: 2**28 clock cycles."]
    Wdt2pow28 = 3,
    #[doc = "4: 2^27 clock cycles."]
    Wdt2pow27 = 4,
    #[doc = "5: 2**26 clock cycles."]
    Wdt2pow26 = 5,
    #[doc = "6: 2**25 clock cycles."]
    Wdt2pow25 = 6,
    #[doc = "7: 2**24 clock cycles."]
    Wdt2pow24 = 7,
    #[doc = "8: 2**23 clock cycles."]
    Wdt2pow23 = 8,
    #[doc = "9: 2**22 clock cycles."]
    Wdt2pow22 = 9,
    #[doc = "10: 2**21 clock cycles."]
    Wdt2pow21 = 10,
    #[doc = "11: 2**20 clock cycles."]
    Wdt2pow20 = 11,
    #[doc = "12: 2**19 clock cycles."]
    Wdt2pow19 = 12,
    #[doc = "13: 2**18 clock cycles."]
    Wdt2pow18 = 13,
    #[doc = "14: 2**17 clock cycles."]
    Wdt2pow17 = 14,
    #[doc = "15: 2**16 clock cycles."]
    Wdt2pow16 = 15,
}
impl From<IntEarlyVal> for u8 {
    #[inline(always)]
    fn from(variant: IntEarlyVal) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntEarlyVal {
    type Ux = u8;
}
impl crate::IsEnum for IntEarlyVal {}
#[doc = "Field `INT_EARLY_VAL` reader - Windowed Watchdog Interrupt Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A windowed watchdog timer interrupt is generated (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub type IntEarlyValR = crate::FieldReader<IntEarlyVal>;
impl IntEarlyValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEarlyVal {
        match self.bits {
            0 => IntEarlyVal::Wdt2pow31,
            1 => IntEarlyVal::Wdt2pow30,
            2 => IntEarlyVal::Wdt2pow29,
            3 => IntEarlyVal::Wdt2pow28,
            4 => IntEarlyVal::Wdt2pow27,
            5 => IntEarlyVal::Wdt2pow26,
            6 => IntEarlyVal::Wdt2pow25,
            7 => IntEarlyVal::Wdt2pow24,
            8 => IntEarlyVal::Wdt2pow23,
            9 => IntEarlyVal::Wdt2pow22,
            10 => IntEarlyVal::Wdt2pow21,
            11 => IntEarlyVal::Wdt2pow20,
            12 => IntEarlyVal::Wdt2pow19,
            13 => IntEarlyVal::Wdt2pow18,
            14 => IntEarlyVal::Wdt2pow17,
            15 => IntEarlyVal::Wdt2pow16,
            _ => unreachable!(),
        }
    }
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow31(&self) -> bool {
        *self == IntEarlyVal::Wdt2pow31
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow30(&self) -> bool {
        *self == IntEarlyVal::Wdt2pow30
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow29(&self) -> bool {
        *self == IntEarlyVal::Wdt2pow29
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow28(&self) -> bool {
        *self == IntEarlyVal::Wdt2pow28
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow27(&self) -> bool {
        *self == IntEarlyVal::Wdt2pow27
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow26(&self) -> bool {
        *self == IntEarlyVal::Wdt2pow26
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow25(&self) -> bool {
        *self == IntEarlyVal::Wdt2pow25
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow24(&self) -> bool {
        *self == IntEarlyVal::Wdt2pow24
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow23(&self) -> bool {
        *self == IntEarlyVal::Wdt2pow23
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow22(&self) -> bool {
        *self == IntEarlyVal::Wdt2pow22
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow21(&self) -> bool {
        *self == IntEarlyVal::Wdt2pow21
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow20(&self) -> bool {
        *self == IntEarlyVal::Wdt2pow20
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow19(&self) -> bool {
        *self == IntEarlyVal::Wdt2pow19
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow18(&self) -> bool {
        *self == IntEarlyVal::Wdt2pow18
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow17(&self) -> bool {
        *self == IntEarlyVal::Wdt2pow17
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow16(&self) -> bool {
        *self == IntEarlyVal::Wdt2pow16
    }
}
#[doc = "Field `INT_EARLY_VAL` writer - Windowed Watchdog Interrupt Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A windowed watchdog timer interrupt is generated (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub type IntEarlyValW<'a, REG> = crate::FieldWriter<'a, REG, 4, IntEarlyVal, crate::Safe>;
impl<'a, REG> IntEarlyValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow31(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarlyVal::Wdt2pow31)
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow30(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarlyVal::Wdt2pow30)
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow29(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarlyVal::Wdt2pow29)
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow28(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarlyVal::Wdt2pow28)
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow27(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarlyVal::Wdt2pow27)
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow26(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarlyVal::Wdt2pow26)
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow25(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarlyVal::Wdt2pow25)
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow24(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarlyVal::Wdt2pow24)
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow23(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarlyVal::Wdt2pow23)
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow22(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarlyVal::Wdt2pow22)
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow21(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarlyVal::Wdt2pow21)
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow20(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarlyVal::Wdt2pow20)
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow19(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarlyVal::Wdt2pow19)
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow18(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarlyVal::Wdt2pow18)
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow17(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarlyVal::Wdt2pow17)
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow16(self) -> &'a mut crate::W<REG> {
        self.variant(IntEarlyVal::Wdt2pow16)
    }
}
#[doc = "Windowed Watchdog Reset Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A system reset occurs (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RstEarlyVal {
    #[doc = "0: 2**31 clock cycles."]
    Wdt2pow31 = 0,
    #[doc = "1: 2**30 clock cycles."]
    Wdt2pow30 = 1,
    #[doc = "2: 2**29 clock cycles."]
    Wdt2pow29 = 2,
    #[doc = "3: 2**28 clock cycles."]
    Wdt2pow28 = 3,
    #[doc = "4: 2^27 clock cycles."]
    Wdt2pow27 = 4,
    #[doc = "5: 2**26 clock cycles."]
    Wdt2pow26 = 5,
    #[doc = "6: 2**25 clock cycles."]
    Wdt2pow25 = 6,
    #[doc = "7: 2**24 clock cycles."]
    Wdt2pow24 = 7,
    #[doc = "8: 2**23 clock cycles."]
    Wdt2pow23 = 8,
    #[doc = "9: 2**22 clock cycles."]
    Wdt2pow22 = 9,
    #[doc = "10: 2**21 clock cycles."]
    Wdt2pow21 = 10,
    #[doc = "11: 2**20 clock cycles."]
    Wdt2pow20 = 11,
    #[doc = "12: 2**19 clock cycles."]
    Wdt2pow19 = 12,
    #[doc = "13: 2**18 clock cycles."]
    Wdt2pow18 = 13,
    #[doc = "14: 2**17 clock cycles."]
    Wdt2pow17 = 14,
    #[doc = "15: 2**16 clock cycles."]
    Wdt2pow16 = 15,
}
impl From<RstEarlyVal> for u8 {
    #[inline(always)]
    fn from(variant: RstEarlyVal) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RstEarlyVal {
    type Ux = u8;
}
impl crate::IsEnum for RstEarlyVal {}
#[doc = "Field `RST_EARLY_VAL` reader - Windowed Watchdog Reset Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A system reset occurs (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub type RstEarlyValR = crate::FieldReader<RstEarlyVal>;
impl RstEarlyValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstEarlyVal {
        match self.bits {
            0 => RstEarlyVal::Wdt2pow31,
            1 => RstEarlyVal::Wdt2pow30,
            2 => RstEarlyVal::Wdt2pow29,
            3 => RstEarlyVal::Wdt2pow28,
            4 => RstEarlyVal::Wdt2pow27,
            5 => RstEarlyVal::Wdt2pow26,
            6 => RstEarlyVal::Wdt2pow25,
            7 => RstEarlyVal::Wdt2pow24,
            8 => RstEarlyVal::Wdt2pow23,
            9 => RstEarlyVal::Wdt2pow22,
            10 => RstEarlyVal::Wdt2pow21,
            11 => RstEarlyVal::Wdt2pow20,
            12 => RstEarlyVal::Wdt2pow19,
            13 => RstEarlyVal::Wdt2pow18,
            14 => RstEarlyVal::Wdt2pow17,
            15 => RstEarlyVal::Wdt2pow16,
            _ => unreachable!(),
        }
    }
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow31(&self) -> bool {
        *self == RstEarlyVal::Wdt2pow31
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow30(&self) -> bool {
        *self == RstEarlyVal::Wdt2pow30
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow29(&self) -> bool {
        *self == RstEarlyVal::Wdt2pow29
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow28(&self) -> bool {
        *self == RstEarlyVal::Wdt2pow28
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow27(&self) -> bool {
        *self == RstEarlyVal::Wdt2pow27
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow26(&self) -> bool {
        *self == RstEarlyVal::Wdt2pow26
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow25(&self) -> bool {
        *self == RstEarlyVal::Wdt2pow25
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow24(&self) -> bool {
        *self == RstEarlyVal::Wdt2pow24
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow23(&self) -> bool {
        *self == RstEarlyVal::Wdt2pow23
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow22(&self) -> bool {
        *self == RstEarlyVal::Wdt2pow22
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow21(&self) -> bool {
        *self == RstEarlyVal::Wdt2pow21
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow20(&self) -> bool {
        *self == RstEarlyVal::Wdt2pow20
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow19(&self) -> bool {
        *self == RstEarlyVal::Wdt2pow19
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow18(&self) -> bool {
        *self == RstEarlyVal::Wdt2pow18
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow17(&self) -> bool {
        *self == RstEarlyVal::Wdt2pow17
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn is_wdt2pow16(&self) -> bool {
        *self == RstEarlyVal::Wdt2pow16
    }
}
#[doc = "Field `RST_EARLY_VAL` writer - Windowed Watchdog Reset Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A system reset occurs (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
pub type RstEarlyValW<'a, REG> = crate::FieldWriter<'a, REG, 4, RstEarlyVal, crate::Safe>;
impl<'a, REG> RstEarlyValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2**31 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow31(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarlyVal::Wdt2pow31)
    }
    #[doc = "2**30 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow30(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarlyVal::Wdt2pow30)
    }
    #[doc = "2**29 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow29(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarlyVal::Wdt2pow29)
    }
    #[doc = "2**28 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow28(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarlyVal::Wdt2pow28)
    }
    #[doc = "2^27 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow27(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarlyVal::Wdt2pow27)
    }
    #[doc = "2**26 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow26(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarlyVal::Wdt2pow26)
    }
    #[doc = "2**25 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow25(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarlyVal::Wdt2pow25)
    }
    #[doc = "2**24 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow24(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarlyVal::Wdt2pow24)
    }
    #[doc = "2**23 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow23(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarlyVal::Wdt2pow23)
    }
    #[doc = "2**22 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow22(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarlyVal::Wdt2pow22)
    }
    #[doc = "2**21 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow21(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarlyVal::Wdt2pow21)
    }
    #[doc = "2**20 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow20(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarlyVal::Wdt2pow20)
    }
    #[doc = "2**19 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow19(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarlyVal::Wdt2pow19)
    }
    #[doc = "2**18 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow18(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarlyVal::Wdt2pow18)
    }
    #[doc = "2**17 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow17(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarlyVal::Wdt2pow17)
    }
    #[doc = "2**16 clock cycles."]
    #[inline(always)]
    pub fn wdt2pow16(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarlyVal::Wdt2pow16)
    }
}
#[doc = "Field `CLKRDY_IE` reader - Switch Ready Interrupt Enable. Fires an interrupt when it is safe to swithc the clock."]
pub type ClkrdyIeR = crate::BitReader;
#[doc = "Field `CLKRDY_IE` writer - Switch Ready Interrupt Enable. Fires an interrupt when it is safe to swithc the clock."]
pub type ClkrdyIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKRDY` reader - Clock Status."]
pub type ClkrdyR = crate::BitReader;
#[doc = "Field `CLKRDY` writer - Clock Status."]
pub type ClkrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enables the Windowed Watchdog Function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WinEn {
    #[doc = "0: Windowed Mode Disabled (i.e. Compatibility Mode)."]
    Dis = 0,
    #[doc = "1: Windowed Mode Enabled."]
    En = 1,
}
impl From<WinEn> for bool {
    #[inline(always)]
    fn from(variant: WinEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN_EN` reader - Enables the Windowed Watchdog Function."]
pub type WinEnR = crate::BitReader<WinEn>;
impl WinEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WinEn {
        match self.bits {
            false => WinEn::Dis,
            true => WinEn::En,
        }
    }
    #[doc = "Windowed Mode Disabled (i.e. Compatibility Mode)."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WinEn::Dis
    }
    #[doc = "Windowed Mode Enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WinEn::En
    }
}
#[doc = "Field `WIN_EN` writer - Enables the Windowed Watchdog Function."]
pub type WinEnW<'a, REG> = crate::BitWriter<'a, REG, WinEn>;
impl<'a, REG> WinEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Windowed Mode Disabled (i.e. Compatibility Mode)."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(WinEn::Dis)
    }
    #[doc = "Windowed Mode Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(WinEn::En)
    }
}
#[doc = "Windowed Watchdog Timer Reset Flag Too Soon.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstEarly {
    #[doc = "0: The event has not occurred."]
    NoEvent = 0,
    #[doc = "1: The event has occurred."]
    Occurred = 1,
}
impl From<RstEarly> for bool {
    #[inline(always)]
    fn from(variant: RstEarly) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST_EARLY` reader - Windowed Watchdog Timer Reset Flag Too Soon."]
pub type RstEarlyR = crate::BitReader<RstEarly>;
impl RstEarlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstEarly {
        match self.bits {
            false => RstEarly::NoEvent,
            true => RstEarly::Occurred,
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == RstEarly::NoEvent
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == RstEarly::Occurred
    }
}
#[doc = "Field `RST_EARLY` writer - Windowed Watchdog Timer Reset Flag Too Soon."]
pub type RstEarlyW<'a, REG> = crate::BitWriter<'a, REG, RstEarly>;
impl<'a, REG> RstEarlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarly::NoEvent)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut crate::W<REG> {
        self.variant(RstEarly::Occurred)
    }
}
#[doc = "Windowed Watchdog Timer Reset Flag Too Late.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstLate {
    #[doc = "0: The event has not occurred."]
    NoEvent = 0,
    #[doc = "1: The event has occurred."]
    Occurred = 1,
}
impl From<RstLate> for bool {
    #[inline(always)]
    fn from(variant: RstLate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST_LATE` reader - Windowed Watchdog Timer Reset Flag Too Late."]
pub type RstLateR = crate::BitReader<RstLate>;
impl RstLateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstLate {
        match self.bits {
            false => RstLate::NoEvent,
            true => RstLate::Occurred,
        }
    }
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == RstLate::NoEvent
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn is_occurred(&self) -> bool {
        *self == RstLate::Occurred
    }
}
#[doc = "Field `RST_LATE` writer - Windowed Watchdog Timer Reset Flag Too Late."]
pub type RstLateW<'a, REG> = crate::BitWriter<'a, REG, RstLate>;
impl<'a, REG> RstLateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The event has not occurred."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(RstLate::NoEvent)
    }
    #[doc = "The event has occurred."]
    #[inline(always)]
    pub fn occurred(self) -> &'a mut crate::W<REG> {
        self.variant(RstLate::Occurred)
    }
}
impl R {
    #[doc = "Bits 0:3 - Windowed Watchdog Interrupt Upper Limit. Sets the number of WDTCLK cycles until a windowed watchdog timer interrupt is generated (if enabled) if the CPU does not write the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    pub fn int_late_val(&self) -> IntLateValR {
        IntLateValR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Windowed Watchdog Reset Upper Limit. Sets the number of WDTCLK cycles until a system reset occurs (if enabled) if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    pub fn rst_late_val(&self) -> RstLateValR {
        RstLateValR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Windowed Watchdog Timer Enable."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Windowed Watchdog Timer Interrupt Flag Too Late."]
    #[inline(always)]
    pub fn int_late(&self) -> IntLateR {
        IntLateR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Windowed Watchdog Timer Interrupt Enable."]
    #[inline(always)]
    pub fn wdt_int_en(&self) -> WdtIntEnR {
        WdtIntEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Windowed Watchdog Timer Reset Enable."]
    #[inline(always)]
    pub fn wdt_rst_en(&self) -> WdtRstEnR {
        WdtRstEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Windowed Watchdog Timer Interrupt Flag Too Soon."]
    #[inline(always)]
    pub fn int_early(&self) -> IntEarlyR {
        IntEarlyR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Windowed Watchdog Interrupt Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A windowed watchdog timer interrupt is generated (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    pub fn int_early_val(&self) -> IntEarlyValR {
        IntEarlyValR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Windowed Watchdog Reset Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A system reset occurs (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    pub fn rst_early_val(&self) -> RstEarlyValR {
        RstEarlyValR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - Switch Ready Interrupt Enable. Fires an interrupt when it is safe to swithc the clock."]
    #[inline(always)]
    pub fn clkrdy_ie(&self) -> ClkrdyIeR {
        ClkrdyIeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Clock Status."]
    #[inline(always)]
    pub fn clkrdy(&self) -> ClkrdyR {
        ClkrdyR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enables the Windowed Watchdog Function."]
    #[inline(always)]
    pub fn win_en(&self) -> WinEnR {
        WinEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Windowed Watchdog Timer Reset Flag Too Soon."]
    #[inline(always)]
    pub fn rst_early(&self) -> RstEarlyR {
        RstEarlyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Windowed Watchdog Timer Reset Flag Too Late."]
    #[inline(always)]
    pub fn rst_late(&self) -> RstLateR {
        RstLateR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Windowed Watchdog Interrupt Upper Limit. Sets the number of WDTCLK cycles until a windowed watchdog timer interrupt is generated (if enabled) if the CPU does not write the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    pub fn int_late_val(&mut self) -> IntLateValW<CtrlSpec> {
        IntLateValW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Windowed Watchdog Reset Upper Limit. Sets the number of WDTCLK cycles until a system reset occurs (if enabled) if the CPU does not write the watchdog reset sequence to the WDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    pub fn rst_late_val(&mut self) -> RstLateValW<CtrlSpec> {
        RstLateValW::new(self, 4)
    }
    #[doc = "Bit 8 - Windowed Watchdog Timer Enable."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CtrlSpec> {
        EnW::new(self, 8)
    }
    #[doc = "Bit 9 - Windowed Watchdog Timer Interrupt Flag Too Late."]
    #[inline(always)]
    pub fn int_late(&mut self) -> IntLateW<CtrlSpec> {
        IntLateW::new(self, 9)
    }
    #[doc = "Bit 10 - Windowed Watchdog Timer Interrupt Enable."]
    #[inline(always)]
    pub fn wdt_int_en(&mut self) -> WdtIntEnW<CtrlSpec> {
        WdtIntEnW::new(self, 10)
    }
    #[doc = "Bit 11 - Windowed Watchdog Timer Reset Enable."]
    #[inline(always)]
    pub fn wdt_rst_en(&mut self) -> WdtRstEnW<CtrlSpec> {
        WdtRstEnW::new(self, 11)
    }
    #[doc = "Bit 12 - Windowed Watchdog Timer Interrupt Flag Too Soon."]
    #[inline(always)]
    pub fn int_early(&mut self) -> IntEarlyW<CtrlSpec> {
        IntEarlyW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Windowed Watchdog Interrupt Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A windowed watchdog timer interrupt is generated (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    pub fn int_early_val(&mut self) -> IntEarlyValW<CtrlSpec> {
        IntEarlyValW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Windowed Watchdog Reset Lower Limit. Sets the number of WDTCLK cycles that establishes the lower boundary of the watchdog window. A system reset occurs (if enabled) if the CPU writes the windowed watchdog reset sequence to the WWDT_RST register before the watchdog timer has counted this time period since the last timer reset."]
    #[inline(always)]
    pub fn rst_early_val(&mut self) -> RstEarlyValW<CtrlSpec> {
        RstEarlyValW::new(self, 20)
    }
    #[doc = "Bit 27 - Switch Ready Interrupt Enable. Fires an interrupt when it is safe to swithc the clock."]
    #[inline(always)]
    pub fn clkrdy_ie(&mut self) -> ClkrdyIeW<CtrlSpec> {
        ClkrdyIeW::new(self, 27)
    }
    #[doc = "Bit 28 - Clock Status."]
    #[inline(always)]
    pub fn clkrdy(&mut self) -> ClkrdyW<CtrlSpec> {
        ClkrdyW::new(self, 28)
    }
    #[doc = "Bit 29 - Enables the Windowed Watchdog Function."]
    #[inline(always)]
    pub fn win_en(&mut self) -> WinEnW<CtrlSpec> {
        WinEnW::new(self, 29)
    }
    #[doc = "Bit 30 - Windowed Watchdog Timer Reset Flag Too Soon."]
    #[inline(always)]
    pub fn rst_early(&mut self) -> RstEarlyW<CtrlSpec> {
        RstEarlyW::new(self, 30)
    }
    #[doc = "Bit 31 - Windowed Watchdog Timer Reset Flag Too Late."]
    #[inline(always)]
    pub fn rst_late(&mut self) -> RstLateW<CtrlSpec> {
        RstLateW::new(self, 31)
    }
}
#[doc = "Watchdog Timer Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
