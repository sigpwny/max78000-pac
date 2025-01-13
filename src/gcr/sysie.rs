#[doc = "Register `SYSIE` reader"]
pub type R = crate::R<SysieSpec>;
#[doc = "Register `SYSIE` writer"]
pub type W = crate::W<SysieSpec>;
#[doc = "ARM ICE Unlock Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iceunlock {
    #[doc = "0: disabled."]
    Dis = 0,
    #[doc = "1: enabled."]
    En = 1,
}
impl From<Iceunlock> for bool {
    #[inline(always)]
    fn from(variant: Iceunlock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICEUNLOCK` reader - ARM ICE Unlock Interrupt Enable."]
pub type IceunlockR = crate::BitReader<Iceunlock>;
impl IceunlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iceunlock {
        match self.bits {
            false => Iceunlock::Dis,
            true => Iceunlock::En,
        }
    }
    #[doc = "disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Iceunlock::Dis
    }
    #[doc = "enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Iceunlock::En
    }
}
#[doc = "Field `ICEUNLOCK` writer - ARM ICE Unlock Interrupt Enable."]
pub type IceunlockW<'a, REG> = crate::BitWriter<'a, REG, Iceunlock>;
impl<'a, REG> IceunlockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Iceunlock::Dis)
    }
    #[doc = "enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Iceunlock::En)
    }
}
impl R {
    #[doc = "Bit 0 - ARM ICE Unlock Interrupt Enable."]
    #[inline(always)]
    pub fn iceunlock(&self) -> IceunlockR {
        IceunlockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARM ICE Unlock Interrupt Enable."]
    #[inline(always)]
    pub fn iceunlock(&mut self) -> IceunlockW<SysieSpec> {
        IceunlockW::new(self, 0)
    }
}
#[doc = "System Status Interrupt Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sysie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysieSpec;
impl crate::RegisterSpec for SysieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysie::R`](R) reader structure"]
impl crate::Readable for SysieSpec {}
#[doc = "`write(|w| ..)` method takes [`sysie::W`](W) writer structure"]
impl crate::Writable for SysieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSIE to value 0"]
impl crate::Resettable for SysieSpec {
    const RESET_VALUE: u32 = 0;
}
