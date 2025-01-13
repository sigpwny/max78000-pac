#[doc = "Register `VER` reader"]
pub type R = crate::R<VerSpec>;
#[doc = "Register `VER` writer"]
pub type W = crate::W<VerSpec>;
#[doc = "Field `minor` reader - Minor Version Number."]
pub type MinorR = crate::FieldReader;
#[doc = "Field `minor` writer - Minor Version Number."]
pub type MinorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `major` reader - Major Version Number."]
pub type MajorR = crate::FieldReader;
#[doc = "Field `major` writer - Major Version Number."]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Minor Version Number."]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Major Version Number."]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Minor Version Number."]
    #[inline(always)]
    pub fn minor(&mut self) -> MinorW<VerSpec> {
        MinorW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Major Version Number."]
    #[inline(always)]
    pub fn major(&mut self) -> MajorW<VerSpec> {
        MajorW::new(self, 8)
    }
}
#[doc = "Hardware Version.\n\nYou can [`read`](crate::Reg::read) this register and get [`ver::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ver::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VerSpec;
impl crate::RegisterSpec for VerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ver::R`](R) reader structure"]
impl crate::Readable for VerSpec {}
#[doc = "`write(|w| ..)` method takes [`ver::W`](W) writer structure"]
impl crate::Writable for VerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VER to value 0"]
impl crate::Resettable for VerSpec {
    const RESET_VALUE: u32 = 0;
}
