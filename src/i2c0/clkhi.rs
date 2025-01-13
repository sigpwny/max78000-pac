#[doc = "Register `CLKHI` reader"]
pub type R = crate::R<ClkhiSpec>;
#[doc = "Register `CLKHI` writer"]
pub type W = crate::W<ClkhiSpec>;
#[doc = "Field `HI` reader - Clock High. In master mode, these bits define the SCL high period."]
pub type HiR = crate::FieldReader<u16>;
#[doc = "Field `HI` writer - Clock High. In master mode, these bits define the SCL high period."]
pub type HiW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Clock High. In master mode, these bits define the SCL high period."]
    #[inline(always)]
    pub fn hi(&self) -> HiR {
        HiR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Clock High. In master mode, these bits define the SCL high period."]
    #[inline(always)]
    pub fn hi(&mut self) -> HiW<ClkhiSpec> {
        HiW::new(self, 0)
    }
}
#[doc = "Clock high Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkhi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkhi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkhiSpec;
impl crate::RegisterSpec for ClkhiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkhi::R`](R) reader structure"]
impl crate::Readable for ClkhiSpec {}
#[doc = "`write(|w| ..)` method takes [`clkhi::W`](W) writer structure"]
impl crate::Writable for ClkhiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKHI to value 0"]
impl crate::Resettable for ClkhiSpec {
    const RESET_VALUE: u32 = 0;
}
