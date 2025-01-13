#[doc = "Register `SISTAT` reader"]
pub type R = crate::R<SistatSpec>;
#[doc = "Magic Word Validation. This bit is set by the system initialization block following power-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Magic {
    #[doc = "0: Magic word was not set (OTP has not been initialized properly)."]
    MagicNotSet = 0,
    #[doc = "1: Magic word was set (OTP contains valid settings)."]
    MagicSet = 1,
}
impl From<Magic> for bool {
    #[inline(always)]
    fn from(variant: Magic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAGIC` reader - Magic Word Validation. This bit is set by the system initialization block following power-up."]
pub type MagicR = crate::BitReader<Magic>;
impl MagicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Magic {
        match self.bits {
            false => Magic::MagicNotSet,
            true => Magic::MagicSet,
        }
    }
    #[doc = "Magic word was not set (OTP has not been initialized properly)."]
    #[inline(always)]
    pub fn is_magic_not_set(&self) -> bool {
        *self == Magic::MagicNotSet
    }
    #[doc = "Magic word was set (OTP contains valid settings)."]
    #[inline(always)]
    pub fn is_magic_set(&self) -> bool {
        *self == Magic::MagicSet
    }
}
#[doc = "CRC Error Status. This bit is set by the system initialization block following power-up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcerr {
    #[doc = "0: No CRC errors occurred during the read of the OTP memory block."]
    NoError = 0,
    #[doc = "1: A CRC error occurred while reading the OTP. The address of the failure location in the OTP memory is stored in the ERRADDR register."]
    Error = 1,
}
impl From<Crcerr> for bool {
    #[inline(always)]
    fn from(variant: Crcerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERR` reader - CRC Error Status. This bit is set by the system initialization block following power-up."]
pub type CrcerrR = crate::BitReader<Crcerr>;
impl CrcerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcerr {
        match self.bits {
            false => Crcerr::NoError,
            true => Crcerr::Error,
        }
    }
    #[doc = "No CRC errors occurred during the read of the OTP memory block."]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Crcerr::NoError
    }
    #[doc = "A CRC error occurred while reading the OTP. The address of the failure location in the OTP memory is stored in the ERRADDR register."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Crcerr::Error
    }
}
impl R {
    #[doc = "Bit 0 - Magic Word Validation. This bit is set by the system initialization block following power-up."]
    #[inline(always)]
    pub fn magic(&self) -> MagicR {
        MagicR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRC Error Status. This bit is set by the system initialization block following power-up."]
    #[inline(always)]
    pub fn crcerr(&self) -> CrcerrR {
        CrcerrR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "System Initialization Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sistat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SistatSpec;
impl crate::RegisterSpec for SistatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sistat::R`](R) reader structure"]
impl crate::Readable for SistatSpec {}
#[doc = "`reset()` method sets SISTAT to value 0"]
impl crate::Resettable for SistatSpec {
    const RESET_VALUE: u32 = 0;
}
