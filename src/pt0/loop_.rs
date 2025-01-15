#[doc = "Register `LOOP` reader"]
pub type R = crate::R<LoopSpec>;
#[doc = "Register `LOOP` writer"]
pub type W = crate::W<LoopSpec>;
#[doc = "Field `count` reader - Number of loops for this pulse train to repeat."]
pub type CountR = crate::FieldReader<u16>;
#[doc = "Field `count` writer - Number of loops for this pulse train to repeat."]
pub type CountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `delay` reader - Delay between loops of the Pulse Train in PT Peripheral Clock cycles"]
pub type DelayR = crate::FieldReader<u16>;
#[doc = "Field `delay` writer - Delay between loops of the Pulse Train in PT Peripheral Clock cycles"]
pub type DelayW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of loops for this pulse train to repeat."]
    #[inline(always)]
    pub fn count(&self) -> CountR {
        CountR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:27 - Delay between loops of the Pulse Train in PT Peripheral Clock cycles"]
    #[inline(always)]
    pub fn delay(&self) -> DelayR {
        DelayR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of loops for this pulse train to repeat."]
    #[inline(always)]
    pub fn count(&mut self) -> CountW<LoopSpec> {
        CountW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Delay between loops of the Pulse Train in PT Peripheral Clock cycles"]
    #[inline(always)]
    pub fn delay(&mut self) -> DelayW<LoopSpec> {
        DelayW::new(self, 16)
    }
}
#[doc = "Pulse Train Loop Count\n\nYou can [`read`](crate::Reg::read) this register and get [`loop_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loop_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LoopSpec;
impl crate::RegisterSpec for LoopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`loop_::R`](R) reader structure"]
impl crate::Readable for LoopSpec {}
#[doc = "`write(|w| ..)` method takes [`loop_::W`](W) writer structure"]
impl crate::Writable for LoopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOOP to value 0"]
impl crate::Resettable for LoopSpec {
    const RESET_VALUE: u32 = 0;
}
