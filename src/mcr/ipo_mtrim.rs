#[doc = "Register `IPO_MTRIM` reader"]
pub type R = crate::R<IpoMtrimSpec>;
#[doc = "Register `IPO_MTRIM` writer"]
pub type W = crate::W<IpoMtrimSpec>;
#[doc = "Field `MTRIM` reader - Manual Trim Value."]
pub type MtrimR = crate::FieldReader;
#[doc = "Field `MTRIM` writer - Manual Trim Value."]
pub type MtrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRIM_RANGE` reader - Trim Range Select."]
pub type TrimRangeR = crate::BitReader;
#[doc = "Field `TRIM_RANGE` writer - Trim Range Select."]
pub type TrimRangeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Manual Trim Value."]
    #[inline(always)]
    pub fn mtrim(&self) -> MtrimR {
        MtrimR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Trim Range Select."]
    #[inline(always)]
    pub fn trim_range(&self) -> TrimRangeR {
        TrimRangeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Manual Trim Value."]
    #[inline(always)]
    pub fn mtrim(&mut self) -> MtrimW<IpoMtrimSpec> {
        MtrimW::new(self, 0)
    }
    #[doc = "Bit 8 - Trim Range Select."]
    #[inline(always)]
    pub fn trim_range(&mut self) -> TrimRangeW<IpoMtrimSpec> {
        TrimRangeW::new(self, 8)
    }
}
#[doc = "IPO Manual Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipo_mtrim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipo_mtrim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpoMtrimSpec;
impl crate::RegisterSpec for IpoMtrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipo_mtrim::R`](R) reader structure"]
impl crate::Readable for IpoMtrimSpec {}
#[doc = "`write(|w| ..)` method takes [`ipo_mtrim::W`](W) writer structure"]
impl crate::Writable for IpoMtrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPO_MTRIM to value 0"]
impl crate::Resettable for IpoMtrimSpec {
    const RESET_VALUE: u32 = 0;
}
