#[doc = "Register `EXTSETUP` reader"]
pub type R = crate::R<ExtsetupSpec>;
#[doc = "Register `EXTSETUP` writer"]
pub type W = crate::W<ExtsetupSpec>;
#[doc = "Field `EXT_BITS_WORD` reader - Word Length for ch_mode."]
pub type ExtBitsWordR = crate::FieldReader;
#[doc = "Field `EXT_BITS_WORD` writer - Word Length for ch_mode."]
pub type ExtBitsWordW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Word Length for ch_mode."]
    #[inline(always)]
    pub fn ext_bits_word(&self) -> ExtBitsWordR {
        ExtBitsWordR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Word Length for ch_mode."]
    #[inline(always)]
    pub fn ext_bits_word(&mut self) -> ExtBitsWordW<ExtsetupSpec> {
        ExtBitsWordW::new(self, 0)
    }
}
#[doc = "Ext Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`extsetup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extsetup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtsetupSpec;
impl crate::RegisterSpec for ExtsetupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extsetup::R`](R) reader structure"]
impl crate::Readable for ExtsetupSpec {}
#[doc = "`write(|w| ..)` method takes [`extsetup::W`](W) writer structure"]
impl crate::Writable for ExtsetupSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTSETUP to value 0"]
impl crate::Resettable for ExtsetupSpec {
    const RESET_VALUE: u32 = 0;
}
