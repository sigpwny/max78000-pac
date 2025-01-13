#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "32-bit random data is ready to read from TRNG_DATA register. Reading TRNG_DATA when RND_RDY=0 will return all 0's. IRQ is generated when RND_RDY=1 if TRNG_CN.RND_IRQ_EN=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdy {
    #[doc = "0: TRNG Busy"]
    Busy = 0,
    #[doc = "1: 32 bit random data is ready"]
    Ready = 1,
}
impl From<Rdy> for bool {
    #[inline(always)]
    fn from(variant: Rdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY` reader - 32-bit random data is ready to read from TRNG_DATA register. Reading TRNG_DATA when RND_RDY=0 will return all 0's. IRQ is generated when RND_RDY=1 if TRNG_CN.RND_IRQ_EN=1."]
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
    #[doc = "TRNG Busy"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Rdy::Busy
    }
    #[doc = "32 bit random data is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Rdy::Ready
    }
}
#[doc = "Field `RDY` writer - 32-bit random data is ready to read from TRNG_DATA register. Reading TRNG_DATA when RND_RDY=0 will return all 0's. IRQ is generated when RND_RDY=1 if TRNG_CN.RND_IRQ_EN=1."]
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG, Rdy>;
impl<'a, REG> RdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRNG Busy"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut crate::W<REG> {
        self.variant(Rdy::Busy)
    }
    #[doc = "32 bit random data is ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(Rdy::Ready)
    }
}
impl R {
    #[doc = "Bit 0 - 32-bit random data is ready to read from TRNG_DATA register. Reading TRNG_DATA when RND_RDY=0 will return all 0's. IRQ is generated when RND_RDY=1 if TRNG_CN.RND_IRQ_EN=1."]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 32-bit random data is ready to read from TRNG_DATA register. Reading TRNG_DATA when RND_RDY=0 will return all 0's. IRQ is generated when RND_RDY=1 if TRNG_CN.RND_IRQ_EN=1."]
    #[inline(always)]
    pub fn rdy(&mut self) -> RdyW<StatusSpec> {
        RdyW::new(self, 0)
    }
}
#[doc = "Data. The content of this register is valid only when RNG_IS = 1. When TRNG is disabled, read returns 0x0000 0000.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
