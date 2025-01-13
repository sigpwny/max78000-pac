#[doc = "Register `ACTRL` writer"]
pub type W = crate::W<ActrlSpec>;
#[doc = "Field `ACTRL` writer - Access control."]
pub type ActrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Access control."]
    #[inline(always)]
    pub fn actrl(&mut self) -> ActrlW<ActrlSpec> {
        ActrlW::new(self, 0)
    }
}
#[doc = "Access Control Register. Writing the ACTRL register with the following values in the order shown, allows read and write access to the system and user Information block: pflc-actrl = 0x3a7f5ca3; pflc-actrl = 0xa1e34f20; pflc-actrl = 0x9608b2c1. When unlocked, a write of any word will disable access to system and user information block. Readback of this register is always zero.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActrlSpec;
impl crate::RegisterSpec for ActrlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`actrl::W`](W) writer structure"]
impl crate::Writable for ActrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACTRL to value 0"]
impl crate::Resettable for ActrlSpec {
    const RESET_VALUE: u32 = 0;
}
