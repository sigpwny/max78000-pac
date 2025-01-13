#[doc = "Register `PRESET` reader"]
pub type R = crate::R<PresetSpec>;
#[doc = "Register `PRESET` writer"]
pub type W = crate::W<PresetSpec>;
#[doc = "Field `PRESET` reader - Preset Value."]
pub type PresetR = crate::FieldReader<u32>;
#[doc = "Field `PRESET` writer - Preset Value."]
pub type PresetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Preset Value."]
    #[inline(always)]
    pub fn preset(&self) -> PresetR {
        PresetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Preset Value."]
    #[inline(always)]
    pub fn preset(&mut self) -> PresetW<PresetSpec> {
        PresetW::new(self, 0)
    }
}
#[doc = "Preset register.\n\nYou can [`read`](crate::Reg::read) this register and get [`preset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`preset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PresetSpec;
impl crate::RegisterSpec for PresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`preset::R`](R) reader structure"]
impl crate::Readable for PresetSpec {}
#[doc = "`write(|w| ..)` method takes [`preset::W`](W) writer structure"]
impl crate::Writable for PresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRESET to value 0"]
impl crate::Resettable for PresetSpec {
    const RESET_VALUE: u32 = 0;
}
