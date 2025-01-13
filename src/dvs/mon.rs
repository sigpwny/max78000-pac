#[doc = "Register `MON` reader"]
pub type R = crate::R<MonSpec>;
#[doc = "Register `MON` writer"]
pub type W = crate::W<MonSpec>;
#[doc = "Field `DLY` reader - Number of prescaled clocks between delay line samples"]
pub type DlyR = crate::FieldReader<u32>;
#[doc = "Field `DLY` writer - Number of prescaled clocks between delay line samples"]
pub type DlyW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `PRE` reader - Number of clocks before DVS_MON_DLY is decremented"]
pub type PreR = crate::FieldReader;
#[doc = "Field `PRE` writer - Number of clocks before DVS_MON_DLY is decremented"]
pub type PreW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - Number of prescaled clocks between delay line samples"]
    #[inline(always)]
    pub fn dly(&self) -> DlyR {
        DlyR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - Number of clocks before DVS_MON_DLY is decremented"]
    #[inline(always)]
    pub fn pre(&self) -> PreR {
        PreR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Number of prescaled clocks between delay line samples"]
    #[inline(always)]
    pub fn dly(&mut self) -> DlyW<MonSpec> {
        DlyW::new(self, 0)
    }
    #[doc = "Bits 24:31 - Number of clocks before DVS_MON_DLY is decremented"]
    #[inline(always)]
    pub fn pre(&mut self) -> PreW<MonSpec> {
        PreW::new(self, 24)
    }
}
#[doc = "Monitor Delay\n\nYou can [`read`](crate::Reg::read) this register and get [`mon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MonSpec;
impl crate::RegisterSpec for MonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mon::R`](R) reader structure"]
impl crate::Readable for MonSpec {}
#[doc = "`write(|w| ..)` method takes [`mon::W`](W) writer structure"]
impl crate::Writable for MonSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MON to value 0"]
impl crate::Resettable for MonSpec {
    const RESET_VALUE: u32 = 0;
}
