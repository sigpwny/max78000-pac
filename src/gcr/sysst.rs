#[doc = "Register `SYSST` reader"]
pub type R = crate::R<SysstSpec>;
#[doc = "Register `SYSST` writer"]
pub type W = crate::W<SysstSpec>;
#[doc = "ARM ICE Lock Status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icelock {
    #[doc = "0: ICE is unlocked."]
    Unlocked = 0,
    #[doc = "1: ICE is locked."]
    Locked = 1,
}
impl From<Icelock> for bool {
    #[inline(always)]
    fn from(variant: Icelock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICELOCK` reader - ARM ICE Lock Status."]
pub type IcelockR = crate::BitReader<Icelock>;
impl IcelockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icelock {
        match self.bits {
            false => Icelock::Unlocked,
            true => Icelock::Locked,
        }
    }
    #[doc = "ICE is unlocked."]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Icelock::Unlocked
    }
    #[doc = "ICE is locked."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Icelock::Locked
    }
}
#[doc = "Field `ICELOCK` writer - ARM ICE Lock Status."]
pub type IcelockW<'a, REG> = crate::BitWriter<'a, REG, Icelock>;
impl<'a, REG> IcelockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ICE is unlocked."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Icelock::Unlocked)
    }
    #[doc = "ICE is locked."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Icelock::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - ARM ICE Lock Status."]
    #[inline(always)]
    pub fn icelock(&self) -> IcelockR {
        IcelockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARM ICE Lock Status."]
    #[inline(always)]
    pub fn icelock(&mut self) -> IcelockW<SysstSpec> {
        IcelockW::new(self, 0)
    }
}
#[doc = "System Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sysst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysstSpec;
impl crate::RegisterSpec for SysstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysst::R`](R) reader structure"]
impl crate::Readable for SysstSpec {}
#[doc = "`write(|w| ..)` method takes [`sysst::W`](W) writer structure"]
impl crate::Writable for SysstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSST to value 0"]
impl crate::Resettable for SysstSpec {
    const RESET_VALUE: u32 = 0;
}
