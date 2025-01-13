#[doc = "Register `ADJ_DWN` reader"]
pub type R = crate::R<AdjDwnSpec>;
#[doc = "Register `ADJ_DWN` writer"]
pub type W = crate::W<AdjDwnSpec>;
#[doc = "Field `DLY` reader - Number of prescaled clocks between updates of the adjustment delay counter"]
pub type DlyR = crate::FieldReader<u16>;
#[doc = "Field `DLY` writer - Number of prescaled clocks between updates of the adjustment delay counter"]
pub type DlyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PRE` reader - Number of clocks before DVS_ADJ_DWN_DLY is decremented"]
pub type PreR = crate::FieldReader;
#[doc = "Field `PRE` writer - Number of clocks before DVS_ADJ_DWN_DLY is decremented"]
pub type PreW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Number of prescaled clocks between updates of the adjustment delay counter"]
    #[inline(always)]
    pub fn dly(&self) -> DlyR {
        DlyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Number of clocks before DVS_ADJ_DWN_DLY is decremented"]
    #[inline(always)]
    pub fn pre(&self) -> PreR {
        PreR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of prescaled clocks between updates of the adjustment delay counter"]
    #[inline(always)]
    pub fn dly(&mut self) -> DlyW<AdjDwnSpec> {
        DlyW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Number of clocks before DVS_ADJ_DWN_DLY is decremented"]
    #[inline(always)]
    pub fn pre(&mut self) -> PreW<AdjDwnSpec> {
        PreW::new(self, 16)
    }
}
#[doc = "Down Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adj_dwn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adj_dwn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdjDwnSpec;
impl crate::RegisterSpec for AdjDwnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adj_dwn::R`](R) reader structure"]
impl crate::Readable for AdjDwnSpec {}
#[doc = "`write(|w| ..)` method takes [`adj_dwn::W`](W) writer structure"]
impl crate::Writable for AdjDwnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADJ_DWN to value 0"]
impl crate::Resettable for AdjDwnSpec {
    const RESET_VALUE: u32 = 0;
}
