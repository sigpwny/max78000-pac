#[doc = "Register `OUTEN` reader"]
pub type R = crate::R<OutenSpec>;
#[doc = "Register `OUTEN` writer"]
pub type W = crate::W<OutenSpec>;
#[doc = "Field `SQWOUT_EN` reader - Square Wave Output Enable."]
pub type SqwoutEnR = crate::BitReader;
#[doc = "Field `SQWOUT_EN` writer - Square Wave Output Enable."]
pub type SqwoutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDOWN_OUT_EN` reader - Power Down Output Enable."]
pub type PdownOutEnR = crate::BitReader;
#[doc = "Field `PDOWN_OUT_EN` writer - Power Down Output Enable."]
pub type PdownOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Square Wave Output Enable."]
    #[inline(always)]
    pub fn sqwout_en(&self) -> SqwoutEnR {
        SqwoutEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Down Output Enable."]
    #[inline(always)]
    pub fn pdown_out_en(&self) -> PdownOutEnR {
        PdownOutEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Square Wave Output Enable."]
    #[inline(always)]
    pub fn sqwout_en(&mut self) -> SqwoutEnW<OutenSpec> {
        SqwoutEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Power Down Output Enable."]
    #[inline(always)]
    pub fn pdown_out_en(&mut self) -> PdownOutEnW<OutenSpec> {
        PdownOutEnW::new(self, 1)
    }
}
#[doc = "Output Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`outen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutenSpec;
impl crate::RegisterSpec for OutenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outen::R`](R) reader structure"]
impl crate::Readable for OutenSpec {}
#[doc = "`write(|w| ..)` method takes [`outen::W`](W) writer structure"]
impl crate::Writable for OutenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTEN to value 0"]
impl crate::Resettable for OutenSpec {
    const RESET_VALUE: u32 = 0;
}
