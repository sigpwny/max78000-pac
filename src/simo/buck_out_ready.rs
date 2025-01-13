#[doc = "Register `BUCK_OUT_READY` reader"]
pub type R = crate::R<BuckOutReadySpec>;
#[doc = "When set, indicates that the output voltage has reached its regulated value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Buckoutrdya {
    #[doc = "0: Output voltage not in range"]
    Notrdy = 0,
    #[doc = "1: Output voltage in range"]
    Rdy = 1,
}
impl From<Buckoutrdya> for bool {
    #[inline(always)]
    fn from(variant: Buckoutrdya) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUCKOUTRDYA` reader - When set, indicates that the output voltage has reached its regulated value"]
pub type BuckoutrdyaR = crate::BitReader<Buckoutrdya>;
impl BuckoutrdyaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Buckoutrdya {
        match self.bits {
            false => Buckoutrdya::Notrdy,
            true => Buckoutrdya::Rdy,
        }
    }
    #[doc = "Output voltage not in range"]
    #[inline(always)]
    pub fn is_notrdy(&self) -> bool {
        *self == Buckoutrdya::Notrdy
    }
    #[doc = "Output voltage in range"]
    #[inline(always)]
    pub fn is_rdy(&self) -> bool {
        *self == Buckoutrdya::Rdy
    }
}
#[doc = "When set, indicates that the output voltage has reached its regulated value"]
pub use Buckoutrdya as Buckoutrdyb;
#[doc = "When set, indicates that the output voltage has reached its regulated value"]
pub use Buckoutrdya as Buckoutrdyc;
#[doc = "When set, indicates that the output voltage has reached its regulated value"]
pub use Buckoutrdya as Buckoutrdyd;
#[doc = "Field `BUCKOUTRDYB` reader - When set, indicates that the output voltage has reached its regulated value"]
pub use BuckoutrdyaR as BuckoutrdybR;
#[doc = "Field `BUCKOUTRDYC` reader - When set, indicates that the output voltage has reached its regulated value"]
pub use BuckoutrdyaR as BuckoutrdycR;
#[doc = "Field `BUCKOUTRDYD` reader - When set, indicates that the output voltage has reached its regulated value"]
pub use BuckoutrdyaR as BuckoutrdydR;
impl R {
    #[doc = "Bit 0 - When set, indicates that the output voltage has reached its regulated value"]
    #[inline(always)]
    pub fn buckoutrdya(&self) -> BuckoutrdyaR {
        BuckoutrdyaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set, indicates that the output voltage has reached its regulated value"]
    #[inline(always)]
    pub fn buckoutrdyb(&self) -> BuckoutrdybR {
        BuckoutrdybR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set, indicates that the output voltage has reached its regulated value"]
    #[inline(always)]
    pub fn buckoutrdyc(&self) -> BuckoutrdycR {
        BuckoutrdycR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set, indicates that the output voltage has reached its regulated value"]
    #[inline(always)]
    pub fn buckoutrdyd(&self) -> BuckoutrdydR {
        BuckoutrdydR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Buck Regulator Output Ready Register\n\nYou can [`read`](crate::Reg::read) this register and get [`buck_out_ready::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BuckOutReadySpec;
impl crate::RegisterSpec for BuckOutReadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buck_out_ready::R`](R) reader structure"]
impl crate::Readable for BuckOutReadySpec {}
#[doc = "`reset()` method sets BUCK_OUT_READY to value 0"]
impl crate::Resettable for BuckOutReadySpec {
    const RESET_VALUE: u32 = 0;
}
