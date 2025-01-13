#[doc = "Register `SFSTAT` reader"]
pub type R = crate::R<SfstatSpec>;
#[doc = "TRNG Function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trng {
    #[doc = "0: `0`"]
    No = 0,
    #[doc = "1: `1`"]
    Yes = 1,
}
impl From<Trng> for bool {
    #[inline(always)]
    fn from(variant: Trng) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRNG` reader - TRNG Function."]
pub type TrngR = crate::BitReader<Trng>;
impl TrngR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trng {
        match self.bits {
            false => Trng::No,
            true => Trng::Yes,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Trng::No
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Trng::Yes
    }
}
#[doc = "AES Block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aes {
    #[doc = "0: `0`"]
    No = 0,
    #[doc = "1: `1`"]
    Yes = 1,
}
impl From<Aes> for bool {
    #[inline(always)]
    fn from(variant: Aes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AES` reader - AES Block."]
pub type AesR = crate::BitReader<Aes>;
impl AesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aes {
        match self.bits {
            false => Aes::No,
            true => Aes::Yes,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == Aes::No
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == Aes::Yes
    }
}
impl R {
    #[doc = "Bit 0 - TRNG Function."]
    #[inline(always)]
    pub fn trng(&self) -> TrngR {
        TrngR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - AES Block."]
    #[inline(always)]
    pub fn aes(&self) -> AesR {
        AesR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Security function status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sfstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfstatSpec;
impl crate::RegisterSpec for SfstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfstat::R`](R) reader structure"]
impl crate::Readable for SfstatSpec {}
#[doc = "`reset()` method sets SFSTAT to value 0"]
impl crate::Resettable for SfstatSpec {
    const RESET_VALUE: u32 = 0;
}
