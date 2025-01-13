#[doc = "Register `THRES_CMP` reader"]
pub type R = crate::R<ThresCmpSpec>;
#[doc = "Register `THRES_CMP` writer"]
pub type W = crate::W<ThresCmpSpec>;
#[doc = "Field `VCNTR_THRES_CNT` reader - Value used to determine 'low voltage' range"]
pub type VcntrThresCntR = crate::FieldReader;
#[doc = "Field `VCNTR_THRES_CNT` writer - Value used to determine 'low voltage' range"]
pub type VcntrThresCntW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `VCNTR_THRES_MASK` reader - Mask applied to threshold and vcount to determine if the device is in a low voltage range"]
pub type VcntrThresMaskR = crate::FieldReader;
#[doc = "Field `VCNTR_THRES_MASK` writer - Mask applied to threshold and vcount to determine if the device is in a low voltage range"]
pub type VcntrThresMaskW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Value used to determine 'low voltage' range"]
    #[inline(always)]
    pub fn vcntr_thres_cnt(&self) -> VcntrThresCntR {
        VcntrThresCntR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Mask applied to threshold and vcount to determine if the device is in a low voltage range"]
    #[inline(always)]
    pub fn vcntr_thres_mask(&self) -> VcntrThresMaskR {
        VcntrThresMaskR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Value used to determine 'low voltage' range"]
    #[inline(always)]
    pub fn vcntr_thres_cnt(&mut self) -> VcntrThresCntW<ThresCmpSpec> {
        VcntrThresCntW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Mask applied to threshold and vcount to determine if the device is in a low voltage range"]
    #[inline(always)]
    pub fn vcntr_thres_mask(&mut self) -> VcntrThresMaskW<ThresCmpSpec> {
        VcntrThresMaskW::new(self, 8)
    }
}
#[doc = "Up Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`thres_cmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres_cmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThresCmpSpec;
impl crate::RegisterSpec for ThresCmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thres_cmp::R`](R) reader structure"]
impl crate::Readable for ThresCmpSpec {}
#[doc = "`write(|w| ..)` method takes [`thres_cmp::W`](W) writer structure"]
impl crate::Writable for ThresCmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets THRES_CMP to value 0"]
impl crate::Resettable for ThresCmpSpec {
    const RESET_VALUE: u32 = 0;
}
