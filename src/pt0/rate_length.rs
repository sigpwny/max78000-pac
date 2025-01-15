#[doc = "Register `RATE_LENGTH` reader"]
pub type R = crate::R<RateLengthSpec>;
#[doc = "Register `RATE_LENGTH` writer"]
pub type W = crate::W<RateLengthSpec>;
#[doc = "Field `rate_control` reader - Pulse Train Enable and Rate Control. Set to 0 to disable the Pulse Train."]
pub type RateControlR = crate::FieldReader<u32>;
#[doc = "Field `rate_control` writer - Pulse Train Enable and Rate Control. Set to 0 to disable the Pulse Train."]
pub type RateControlW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
#[doc = "Pulse Train Output Mode/Train Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Pulse train, 32 bit pattern."]
    _32Bit = 0,
    #[doc = "1: Square wave mode."]
    SquareWave = 1,
    #[doc = "2: Pulse train, 2 bit pattern."]
    _2Bit = 2,
    #[doc = "3: Pulse train, 3 bit pattern."]
    _3Bit = 3,
    #[doc = "4: Pulse train, 4 bit pattern."]
    _4Bit = 4,
    #[doc = "5: Pulse train, 5 bit pattern."]
    _5Bit = 5,
    #[doc = "6: Pulse train, 6 bit pattern."]
    _6Bit = 6,
    #[doc = "7: Pulse train, 7 bit pattern."]
    _7Bit = 7,
    #[doc = "8: Pulse train, 8 bit pattern."]
    _8Bit = 8,
    #[doc = "9: Pulse train, 9 bit pattern."]
    _9Bit = 9,
    #[doc = "10: Pulse train, 10 bit pattern."]
    _10Bit = 10,
    #[doc = "11: Pulse train, 11 bit pattern."]
    _11Bit = 11,
    #[doc = "12: Pulse train, 12 bit pattern."]
    _12Bit = 12,
    #[doc = "13: Pulse train, 13 bit pattern."]
    _13Bit = 13,
    #[doc = "14: Pulse train, 14 bit pattern."]
    _14Bit = 14,
    #[doc = "15: Pulse train, 15 bit pattern."]
    _15Bit = 15,
    #[doc = "16: Pulse train, 16 bit pattern."]
    _16Bit = 16,
    #[doc = "17: Pulse train, 17 bit pattern."]
    _17Bit = 17,
    #[doc = "18: Pulse train, 18 bit pattern."]
    _18Bit = 18,
    #[doc = "19: Pulse train, 19 bit pattern."]
    _19Bit = 19,
    #[doc = "20: Pulse train, 20 bit pattern."]
    _20Bit = 20,
    #[doc = "21: Pulse train, 21 bit pattern."]
    _21Bit = 21,
    #[doc = "22: Pulse train, 22 bit pattern."]
    _22Bit = 22,
    #[doc = "23: Pulse train, 23 bit pattern."]
    _23Bit = 23,
    #[doc = "24: Pulse train, 24 bit pattern."]
    _24Bit = 24,
    #[doc = "25: Pulse train, 25 bit pattern."]
    _25Bit = 25,
    #[doc = "26: Pulse train, 26 bit pattern."]
    _26Bit = 26,
    #[doc = "27: Pulse train, 27 bit pattern."]
    _27Bit = 27,
    #[doc = "28: Pulse train, 28 bit pattern."]
    _28Bit = 28,
    #[doc = "29: Pulse train, 29 bit pattern."]
    _29Bit = 29,
    #[doc = "30: Pulse train, 30 bit pattern."]
    _30Bit = 30,
    #[doc = "31: Pulse train, 31 bit pattern."]
    _31Bit = 31,
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
#[doc = "Field `mode` reader - Pulse Train Output Mode/Train Length"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::_32Bit,
            1 => Mode::SquareWave,
            2 => Mode::_2Bit,
            3 => Mode::_3Bit,
            4 => Mode::_4Bit,
            5 => Mode::_5Bit,
            6 => Mode::_6Bit,
            7 => Mode::_7Bit,
            8 => Mode::_8Bit,
            9 => Mode::_9Bit,
            10 => Mode::_10Bit,
            11 => Mode::_11Bit,
            12 => Mode::_12Bit,
            13 => Mode::_13Bit,
            14 => Mode::_14Bit,
            15 => Mode::_15Bit,
            16 => Mode::_16Bit,
            17 => Mode::_17Bit,
            18 => Mode::_18Bit,
            19 => Mode::_19Bit,
            20 => Mode::_20Bit,
            21 => Mode::_21Bit,
            22 => Mode::_22Bit,
            23 => Mode::_23Bit,
            24 => Mode::_24Bit,
            25 => Mode::_25Bit,
            26 => Mode::_26Bit,
            27 => Mode::_27Bit,
            28 => Mode::_28Bit,
            29 => Mode::_29Bit,
            30 => Mode::_30Bit,
            31 => Mode::_31Bit,
            _ => unreachable!(),
        }
    }
    #[doc = "Pulse train, 32 bit pattern."]
    #[inline(always)]
    pub fn is_32_bit(&self) -> bool {
        *self == Mode::_32Bit
    }
    #[doc = "Square wave mode."]
    #[inline(always)]
    pub fn is_square_wave(&self) -> bool {
        *self == Mode::SquareWave
    }
    #[doc = "Pulse train, 2 bit pattern."]
    #[inline(always)]
    pub fn is_2_bit(&self) -> bool {
        *self == Mode::_2Bit
    }
    #[doc = "Pulse train, 3 bit pattern."]
    #[inline(always)]
    pub fn is_3_bit(&self) -> bool {
        *self == Mode::_3Bit
    }
    #[doc = "Pulse train, 4 bit pattern."]
    #[inline(always)]
    pub fn is_4_bit(&self) -> bool {
        *self == Mode::_4Bit
    }
    #[doc = "Pulse train, 5 bit pattern."]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        *self == Mode::_5Bit
    }
    #[doc = "Pulse train, 6 bit pattern."]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        *self == Mode::_6Bit
    }
    #[doc = "Pulse train, 7 bit pattern."]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        *self == Mode::_7Bit
    }
    #[doc = "Pulse train, 8 bit pattern."]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == Mode::_8Bit
    }
    #[doc = "Pulse train, 9 bit pattern."]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        *self == Mode::_9Bit
    }
    #[doc = "Pulse train, 10 bit pattern."]
    #[inline(always)]
    pub fn is_10_bit(&self) -> bool {
        *self == Mode::_10Bit
    }
    #[doc = "Pulse train, 11 bit pattern."]
    #[inline(always)]
    pub fn is_11_bit(&self) -> bool {
        *self == Mode::_11Bit
    }
    #[doc = "Pulse train, 12 bit pattern."]
    #[inline(always)]
    pub fn is_12_bit(&self) -> bool {
        *self == Mode::_12Bit
    }
    #[doc = "Pulse train, 13 bit pattern."]
    #[inline(always)]
    pub fn is_13_bit(&self) -> bool {
        *self == Mode::_13Bit
    }
    #[doc = "Pulse train, 14 bit pattern."]
    #[inline(always)]
    pub fn is_14_bit(&self) -> bool {
        *self == Mode::_14Bit
    }
    #[doc = "Pulse train, 15 bit pattern."]
    #[inline(always)]
    pub fn is_15_bit(&self) -> bool {
        *self == Mode::_15Bit
    }
    #[doc = "Pulse train, 16 bit pattern."]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == Mode::_16Bit
    }
    #[doc = "Pulse train, 17 bit pattern."]
    #[inline(always)]
    pub fn is_17_bit(&self) -> bool {
        *self == Mode::_17Bit
    }
    #[doc = "Pulse train, 18 bit pattern."]
    #[inline(always)]
    pub fn is_18_bit(&self) -> bool {
        *self == Mode::_18Bit
    }
    #[doc = "Pulse train, 19 bit pattern."]
    #[inline(always)]
    pub fn is_19_bit(&self) -> bool {
        *self == Mode::_19Bit
    }
    #[doc = "Pulse train, 20 bit pattern."]
    #[inline(always)]
    pub fn is_20_bit(&self) -> bool {
        *self == Mode::_20Bit
    }
    #[doc = "Pulse train, 21 bit pattern."]
    #[inline(always)]
    pub fn is_21_bit(&self) -> bool {
        *self == Mode::_21Bit
    }
    #[doc = "Pulse train, 22 bit pattern."]
    #[inline(always)]
    pub fn is_22_bit(&self) -> bool {
        *self == Mode::_22Bit
    }
    #[doc = "Pulse train, 23 bit pattern."]
    #[inline(always)]
    pub fn is_23_bit(&self) -> bool {
        *self == Mode::_23Bit
    }
    #[doc = "Pulse train, 24 bit pattern."]
    #[inline(always)]
    pub fn is_24_bit(&self) -> bool {
        *self == Mode::_24Bit
    }
    #[doc = "Pulse train, 25 bit pattern."]
    #[inline(always)]
    pub fn is_25_bit(&self) -> bool {
        *self == Mode::_25Bit
    }
    #[doc = "Pulse train, 26 bit pattern."]
    #[inline(always)]
    pub fn is_26_bit(&self) -> bool {
        *self == Mode::_26Bit
    }
    #[doc = "Pulse train, 27 bit pattern."]
    #[inline(always)]
    pub fn is_27_bit(&self) -> bool {
        *self == Mode::_27Bit
    }
    #[doc = "Pulse train, 28 bit pattern."]
    #[inline(always)]
    pub fn is_28_bit(&self) -> bool {
        *self == Mode::_28Bit
    }
    #[doc = "Pulse train, 29 bit pattern."]
    #[inline(always)]
    pub fn is_29_bit(&self) -> bool {
        *self == Mode::_29Bit
    }
    #[doc = "Pulse train, 30 bit pattern."]
    #[inline(always)]
    pub fn is_30_bit(&self) -> bool {
        *self == Mode::_30Bit
    }
    #[doc = "Pulse train, 31 bit pattern."]
    #[inline(always)]
    pub fn is_31_bit(&self) -> bool {
        *self == Mode::_31Bit
    }
}
#[doc = "Field `mode` writer - Pulse Train Output Mode/Train Length"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 5, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pulse train, 32 bit pattern."]
    #[inline(always)]
    pub fn _32_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_32Bit)
    }
    #[doc = "Square wave mode."]
    #[inline(always)]
    pub fn square_wave(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::SquareWave)
    }
    #[doc = "Pulse train, 2 bit pattern."]
    #[inline(always)]
    pub fn _2_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_2Bit)
    }
    #[doc = "Pulse train, 3 bit pattern."]
    #[inline(always)]
    pub fn _3_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_3Bit)
    }
    #[doc = "Pulse train, 4 bit pattern."]
    #[inline(always)]
    pub fn _4_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_4Bit)
    }
    #[doc = "Pulse train, 5 bit pattern."]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_5Bit)
    }
    #[doc = "Pulse train, 6 bit pattern."]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_6Bit)
    }
    #[doc = "Pulse train, 7 bit pattern."]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_7Bit)
    }
    #[doc = "Pulse train, 8 bit pattern."]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_8Bit)
    }
    #[doc = "Pulse train, 9 bit pattern."]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_9Bit)
    }
    #[doc = "Pulse train, 10 bit pattern."]
    #[inline(always)]
    pub fn _10_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_10Bit)
    }
    #[doc = "Pulse train, 11 bit pattern."]
    #[inline(always)]
    pub fn _11_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_11Bit)
    }
    #[doc = "Pulse train, 12 bit pattern."]
    #[inline(always)]
    pub fn _12_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_12Bit)
    }
    #[doc = "Pulse train, 13 bit pattern."]
    #[inline(always)]
    pub fn _13_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_13Bit)
    }
    #[doc = "Pulse train, 14 bit pattern."]
    #[inline(always)]
    pub fn _14_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_14Bit)
    }
    #[doc = "Pulse train, 15 bit pattern."]
    #[inline(always)]
    pub fn _15_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_15Bit)
    }
    #[doc = "Pulse train, 16 bit pattern."]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_16Bit)
    }
    #[doc = "Pulse train, 17 bit pattern."]
    #[inline(always)]
    pub fn _17_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_17Bit)
    }
    #[doc = "Pulse train, 18 bit pattern."]
    #[inline(always)]
    pub fn _18_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_18Bit)
    }
    #[doc = "Pulse train, 19 bit pattern."]
    #[inline(always)]
    pub fn _19_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_19Bit)
    }
    #[doc = "Pulse train, 20 bit pattern."]
    #[inline(always)]
    pub fn _20_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_20Bit)
    }
    #[doc = "Pulse train, 21 bit pattern."]
    #[inline(always)]
    pub fn _21_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_21Bit)
    }
    #[doc = "Pulse train, 22 bit pattern."]
    #[inline(always)]
    pub fn _22_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_22Bit)
    }
    #[doc = "Pulse train, 23 bit pattern."]
    #[inline(always)]
    pub fn _23_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_23Bit)
    }
    #[doc = "Pulse train, 24 bit pattern."]
    #[inline(always)]
    pub fn _24_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_24Bit)
    }
    #[doc = "Pulse train, 25 bit pattern."]
    #[inline(always)]
    pub fn _25_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_25Bit)
    }
    #[doc = "Pulse train, 26 bit pattern."]
    #[inline(always)]
    pub fn _26_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_26Bit)
    }
    #[doc = "Pulse train, 27 bit pattern."]
    #[inline(always)]
    pub fn _27_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_27Bit)
    }
    #[doc = "Pulse train, 28 bit pattern."]
    #[inline(always)]
    pub fn _28_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_28Bit)
    }
    #[doc = "Pulse train, 29 bit pattern."]
    #[inline(always)]
    pub fn _29_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_29Bit)
    }
    #[doc = "Pulse train, 30 bit pattern."]
    #[inline(always)]
    pub fn _30_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_30Bit)
    }
    #[doc = "Pulse train, 31 bit pattern."]
    #[inline(always)]
    pub fn _31_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::_31Bit)
    }
}
impl R {
    #[doc = "Bits 0:26 - Pulse Train Enable and Rate Control. Set to 0 to disable the Pulse Train."]
    #[inline(always)]
    pub fn rate_control(&self) -> RateControlR {
        RateControlR::new(self.bits & 0x07ff_ffff)
    }
    #[doc = "Bits 27:31 - Pulse Train Output Mode/Train Length"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:26 - Pulse Train Enable and Rate Control. Set to 0 to disable the Pulse Train."]
    #[inline(always)]
    pub fn rate_control(&mut self) -> RateControlW<RateLengthSpec> {
        RateControlW::new(self, 0)
    }
    #[doc = "Bits 27:31 - Pulse Train Output Mode/Train Length"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<RateLengthSpec> {
        ModeW::new(self, 27)
    }
}
#[doc = "Pulse Train Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rate_length::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rate_length::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RateLengthSpec;
impl crate::RegisterSpec for RateLengthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rate_length::R`](R) reader structure"]
impl crate::Readable for RateLengthSpec {}
#[doc = "`write(|w| ..)` method takes [`rate_length::W`](W) writer structure"]
impl crate::Writable for RateLengthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RATE_LENGTH to value 0"]
impl crate::Resettable for RateLengthSpec {
    const RESET_VALUE: u32 = 0;
}
