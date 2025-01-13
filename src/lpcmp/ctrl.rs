#[doc = "Register `CTRL[%s]` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL[%s]` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EN` reader - Comparator Enable."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Comparator Enable."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL` reader - Polarity Select"]
pub type PolR = crate::BitReader;
#[doc = "Field `POL` writer - Polarity Select"]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN` reader - IRQ Enable."]
pub type IntenR = crate::BitReader;
#[doc = "Field `INTEN` writer - IRQ Enable."]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT` reader - Raw Compartor Input."]
pub type OutR = crate::BitReader;
#[doc = "Field `OUT` writer - Raw Compartor Input."]
pub type OutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTFL` reader - IRQ Flag"]
pub type IntflR = crate::BitReader;
#[doc = "Field `INTFL` writer - IRQ Flag"]
pub type IntflW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator Enable."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Polarity Select"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IRQ Enable."]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - Raw Compartor Input."]
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IRQ Flag"]
    #[inline(always)]
    pub fn intfl(&self) -> IntflR {
        IntflR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator Enable."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 5 - Polarity Select"]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<CtrlSpec> {
        PolW::new(self, 5)
    }
    #[doc = "Bit 6 - IRQ Enable."]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<CtrlSpec> {
        IntenW::new(self, 6)
    }
    #[doc = "Bit 14 - Raw Compartor Input."]
    #[inline(always)]
    pub fn out(&mut self) -> OutW<CtrlSpec> {
        OutW::new(self, 14)
    }
    #[doc = "Bit 15 - IRQ Flag"]
    #[inline(always)]
    pub fn intfl(&mut self) -> IntflW<CtrlSpec> {
        IntflW::new(self, 15)
    }
}
#[doc = "Comparator Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL[%s]
to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
