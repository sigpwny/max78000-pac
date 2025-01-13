#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "To enable IRQ generation when a new 32-bit Random number is ready.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RndIe {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<RndIe> for bool {
    #[inline(always)]
    fn from(variant: RndIe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RND_IE` reader - To enable IRQ generation when a new 32-bit Random number is ready."]
pub type RndIeR = crate::BitReader<RndIe>;
impl RndIeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RndIe {
        match self.bits {
            false => RndIe::Disable,
            true => RndIe::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RndIe::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RndIe::Enable
    }
}
#[doc = "Field `RND_IE` writer - To enable IRQ generation when a new 32-bit Random number is ready."]
pub type RndIeW<'a, REG> = crate::BitWriter<'a, REG, RndIe>;
impl<'a, REG> RndIeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RndIe::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RndIe::Enable)
    }
}
#[doc = "Field `KEYGEN` reader - AES Key Generate. When enabled, the key for securing NVSRAM is generated and transferred to the secure key register automatically without user visibility or intervention. This bit is cleared by hardware once the key has been transferred to the secure key register."]
pub type KeygenR = crate::BitReader;
#[doc = "Field `KEYGEN` writer - AES Key Generate. When enabled, the key for securing NVSRAM is generated and transferred to the secure key register automatically without user visibility or intervention. This bit is cleared by hardware once the key has been transferred to the secure key register."]
pub type KeygenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYWIPE` reader - To wipe the Battery Backed key."]
pub type KeywipeR = crate::BitReader;
#[doc = "Field `KEYWIPE` writer - To wipe the Battery Backed key."]
pub type KeywipeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - To enable IRQ generation when a new 32-bit Random number is ready."]
    #[inline(always)]
    pub fn rnd_ie(&self) -> RndIeR {
        RndIeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - AES Key Generate. When enabled, the key for securing NVSRAM is generated and transferred to the secure key register automatically without user visibility or intervention. This bit is cleared by hardware once the key has been transferred to the secure key register."]
    #[inline(always)]
    pub fn keygen(&self) -> KeygenR {
        KeygenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 15 - To wipe the Battery Backed key."]
    #[inline(always)]
    pub fn keywipe(&self) -> KeywipeR {
        KeywipeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - To enable IRQ generation when a new 32-bit Random number is ready."]
    #[inline(always)]
    pub fn rnd_ie(&mut self) -> RndIeW<CtrlSpec> {
        RndIeW::new(self, 1)
    }
    #[doc = "Bit 3 - AES Key Generate. When enabled, the key for securing NVSRAM is generated and transferred to the secure key register automatically without user visibility or intervention. This bit is cleared by hardware once the key has been transferred to the secure key register."]
    #[inline(always)]
    pub fn keygen(&mut self) -> KeygenW<CtrlSpec> {
        KeygenW::new(self, 3)
    }
    #[doc = "Bit 15 - To wipe the Battery Backed key."]
    #[inline(always)]
    pub fn keywipe(&mut self) -> KeywipeW<CtrlSpec> {
        KeywipeW::new(self, 15)
    }
}
#[doc = "TRNG Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0x03"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x03;
}
