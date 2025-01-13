#[doc = "Register `RTC` reader"]
pub type R = crate::R<RtcSpec>;
#[doc = "Register `RTC` writer"]
pub type W = crate::W<RtcSpec>;
#[doc = "Field `X1TRIM` reader - RTC X1 Trim."]
pub type X1trimR = crate::FieldReader;
#[doc = "Field `X1TRIM` writer - RTC X1 Trim."]
pub type X1trimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `X2TRIM` reader - RTC X2 Trim."]
pub type X2trimR = crate::FieldReader;
#[doc = "Field `X2TRIM` writer - RTC X2 Trim."]
pub type X2trimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LOCK` reader - Lock."]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock."]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:20 - RTC X1 Trim."]
    #[inline(always)]
    pub fn x1trim(&self) -> X1trimR {
        X1trimR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - RTC X2 Trim."]
    #[inline(always)]
    pub fn x2trim(&self) -> X2trimR {
        X2trimR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Lock."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:20 - RTC X1 Trim."]
    #[inline(always)]
    pub fn x1trim(&mut self) -> X1trimW<RtcSpec> {
        X1trimW::new(self, 16)
    }
    #[doc = "Bits 21:25 - RTC X2 Trim."]
    #[inline(always)]
    pub fn x2trim(&mut self) -> X2trimW<RtcSpec> {
        X2trimW::new(self, 21)
    }
    #[doc = "Bit 31 - Lock."]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<RtcSpec> {
        LockW::new(self, 31)
    }
}
#[doc = "RTC Trim System Initialization Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcSpec;
impl crate::RegisterSpec for RtcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc::R`](R) reader structure"]
impl crate::Readable for RtcSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc::W`](W) writer structure"]
impl crate::Writable for RtcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC to value 0"]
impl crate::Resettable for RtcSpec {
    const RESET_VALUE: u32 = 0;
}
