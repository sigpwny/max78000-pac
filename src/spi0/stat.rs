#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "SPI active status. In Master mode, set when transaction starts, cleared when last bit of last character is acted upon and Slave Select de-assertion would occur. In Slave mode, set when Slave Select is asserted, cleared when Slave Select is de-asserted. Not used in Timer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: SPI not active."]
    Not = 0,
    #[doc = "1: SPI active."]
    Active = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - SPI active status. In Master mode, set when transaction starts, cleared when last bit of last character is acted upon and Slave Select de-assertion would occur. In Slave mode, set when Slave Select is asserted, cleared when Slave Select is de-asserted. Not used in Timer mode."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Not,
            true => Busy::Active,
        }
    }
    #[doc = "SPI not active."]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == Busy::Not
    }
    #[doc = "SPI active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Busy::Active
    }
}
impl R {
    #[doc = "Bit 0 - SPI active status. In Master mode, set when transaction starts, cleared when last bit of last character is acted upon and Slave Select de-assertion would occur. In Slave mode, set when Slave Select is asserted, cleared when Slave Select is de-asserted. Not used in Timer mode."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
}
#[doc = "SPI Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
