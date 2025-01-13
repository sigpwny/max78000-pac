#[doc = "Register `RELOAD` reader"]
pub type R = crate::R<ReloadSpec>;
#[doc = "Register `RELOAD` writer"]
pub type W = crate::W<ReloadSpec>;
#[doc = "Field `RELOAD` reader - Rerload Value."]
pub type ReloadR = crate::FieldReader<u32>;
#[doc = "Field `RELOAD` writer - Rerload Value."]
pub type ReloadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Rerload Value."]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rerload Value."]
    #[inline(always)]
    pub fn reload(&mut self) -> ReloadW<ReloadSpec> {
        ReloadW::new(self, 0)
    }
}
#[doc = "Reload register.\n\nYou can [`read`](crate::Reg::read) this register and get [`reload::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reload::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReloadSpec;
impl crate::RegisterSpec for ReloadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reload::R`](R) reader structure"]
impl crate::Readable for ReloadSpec {}
#[doc = "`write(|w| ..)` method takes [`reload::W`](W) writer structure"]
impl crate::Writable for ReloadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RELOAD to value 0"]
impl crate::Resettable for ReloadSpec {
    const RESET_VALUE: u32 = 0;
}
