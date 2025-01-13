#[doc = "Register `CMP_CTRL` reader"]
pub type R = crate::R<CmpCtrlSpec>;
#[doc = "Register `CMP_CTRL` writer"]
pub type W = crate::W<CmpCtrlSpec>;
#[doc = "Field `EN` reader - Comparator Enable."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Comparator Enable."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL` reader - Polarity Select"]
pub type PolR = crate::BitReader;
#[doc = "Field `POL` writer - Polarity Select"]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_EN` reader - IRQ Enable."]
pub type IntEnR = crate::BitReader;
#[doc = "Field `INT_EN` writer - IRQ Enable."]
pub type IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT` reader - Comparator Output State."]
pub type OutR = crate::BitReader;
#[doc = "Field `OUT` writer - Comparator Output State."]
pub type OutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_FL` reader - IRQ Flag"]
pub type IntFlR = crate::BitReader;
#[doc = "Field `INT_FL` writer - IRQ Flag"]
pub type IntFlW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn int_en(&self) -> IntEnR {
        IntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - Comparator Output State."]
    #[inline(always)]
    pub fn out(&self) -> OutR {
        OutR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - IRQ Flag"]
    #[inline(always)]
    pub fn int_fl(&self) -> IntFlR {
        IntFlR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator Enable."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CmpCtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 5 - Polarity Select"]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<CmpCtrlSpec> {
        PolW::new(self, 5)
    }
    #[doc = "Bit 6 - IRQ Enable."]
    #[inline(always)]
    pub fn int_en(&mut self) -> IntEnW<CmpCtrlSpec> {
        IntEnW::new(self, 6)
    }
    #[doc = "Bit 14 - Comparator Output State."]
    #[inline(always)]
    pub fn out(&mut self) -> OutW<CmpCtrlSpec> {
        OutW::new(self, 14)
    }
    #[doc = "Bit 15 - IRQ Flag"]
    #[inline(always)]
    pub fn int_fl(&mut self) -> IntFlW<CmpCtrlSpec> {
        IntFlW::new(self, 15)
    }
}
#[doc = "Comparator Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpCtrlSpec;
impl crate::RegisterSpec for CmpCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp_ctrl::R`](R) reader structure"]
impl crate::Readable for CmpCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp_ctrl::W`](W) writer structure"]
impl crate::Writable for CmpCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP_CTRL to value 0"]
impl crate::Resettable for CmpCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
