#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Flash Done Interrupt. This bit is set to 1 upon Flash write or erase completion.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "0: No interrupt is pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Flash Done Interrupt. This bit is set to 1 upon Flash write or erase completion."]
pub type DoneR = crate::BitReader<Done>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Done {
        match self.bits {
            false => Done::Inactive,
            true => Done::Pending,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Done::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Done::Pending
    }
}
#[doc = "Field `DONE` writer - Flash Done Interrupt. This bit is set to 1 upon Flash write or erase completion."]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG, Done>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Done::Inactive)
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(Done::Pending)
    }
}
#[doc = "Flash Access Fail. This bit is set when an attempt is made to write the flash while the flash is busy or the flash is locked. This bit can only be set to 1 by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Af {
    #[doc = "0: No Failure."]
    NoError = 0,
    #[doc = "1: Failure occurs."]
    Error = 1,
}
impl From<Af> for bool {
    #[inline(always)]
    fn from(variant: Af) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AF` reader - Flash Access Fail. This bit is set when an attempt is made to write the flash while the flash is busy or the flash is locked. This bit can only be set to 1 by hardware."]
pub type AfR = crate::BitReader<Af>;
impl AfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Af {
        match self.bits {
            false => Af::NoError,
            true => Af::Error,
        }
    }
    #[doc = "No Failure."]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Af::NoError
    }
    #[doc = "Failure occurs."]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Af::Error
    }
}
#[doc = "Field `AF` writer - Flash Access Fail. This bit is set when an attempt is made to write the flash while the flash is busy or the flash is locked. This bit can only be set to 1 by hardware."]
pub type AfW<'a, REG> = crate::BitWriter<'a, REG, Af>;
impl<'a, REG> AfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Failure."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(Af::NoError)
    }
    #[doc = "Failure occurs."]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(Af::Error)
    }
}
#[doc = "Flash Done Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doneie {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: Enable."]
    Enable = 1,
}
impl From<Doneie> for bool {
    #[inline(always)]
    fn from(variant: Doneie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONEIE` reader - Flash Done Interrupt Enable."]
pub type DoneieR = crate::BitReader<Doneie>;
impl DoneieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doneie {
        match self.bits {
            false => Doneie::Disable,
            true => Doneie::Enable,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Doneie::Disable
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Doneie::Enable
    }
}
#[doc = "Field `DONEIE` writer - Flash Done Interrupt Enable."]
pub type DoneieW<'a, REG> = crate::BitWriter<'a, REG, Doneie>;
impl<'a, REG> DoneieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doneie::Disable)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doneie::Enable)
    }
}
#[doc = "Flash Done Interrupt Enable."]
pub use Doneie as Afie;
#[doc = "Field `AFIE` reader - Flash Done Interrupt Enable."]
pub use DoneieR as AfieR;
#[doc = "Field `AFIE` writer - Flash Done Interrupt Enable."]
pub use DoneieW as AfieW;
impl R {
    #[doc = "Bit 0 - Flash Done Interrupt. This bit is set to 1 upon Flash write or erase completion."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Access Fail. This bit is set when an attempt is made to write the flash while the flash is busy or the flash is locked. This bit can only be set to 1 by hardware."]
    #[inline(always)]
    pub fn af(&self) -> AfR {
        AfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash Done Interrupt Enable."]
    #[inline(always)]
    pub fn doneie(&self) -> DoneieR {
        DoneieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flash Done Interrupt Enable."]
    #[inline(always)]
    pub fn afie(&self) -> AfieR {
        AfieR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Done Interrupt. This bit is set to 1 upon Flash write or erase completion."]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<IntrSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Flash Access Fail. This bit is set when an attempt is made to write the flash while the flash is busy or the flash is locked. This bit can only be set to 1 by hardware."]
    #[inline(always)]
    pub fn af(&mut self) -> AfW<IntrSpec> {
        AfW::new(self, 1)
    }
    #[doc = "Bit 8 - Flash Done Interrupt Enable."]
    #[inline(always)]
    pub fn doneie(&mut self) -> DoneieW<IntrSpec> {
        DoneieW::new(self, 8)
    }
    #[doc = "Bit 9 - Flash Done Interrupt Enable."]
    #[inline(always)]
    pub fn afie(&mut self) -> AfieW<IntrSpec> {
        AfieW::new(self, 9)
    }
}
#[doc = "Flash Interrupt Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSpec;
impl crate::RegisterSpec for IntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for IntrSpec {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for IntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for IntrSpec {
    const RESET_VALUE: u32 = 0;
}
