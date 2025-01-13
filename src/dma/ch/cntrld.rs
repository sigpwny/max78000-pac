#[doc = "Register `CNTRLD` reader"]
pub type R = crate::R<CntrldSpec>;
#[doc = "Register `CNTRLD` writer"]
pub type W = crate::W<CntrldSpec>;
#[doc = "Field `CNT` reader - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
pub type CntR = crate::FieldReader<u32>;
#[doc = "Field `CNT` writer - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `EN` reader - Count Reload Enable."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Count Reload Enable."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - Count Reload Enable."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Count Reload Value. The value of this register is loaded into DMA0_CNT upon a count-to-zero condition."]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<CntrldSpec> {
        CntW::new(self, 0)
    }
    #[doc = "Bit 31 - Count Reload Enable."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CntrldSpec> {
        EnW::new(self, 31)
    }
}
#[doc = "DMA Channel Count Reload Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cntrld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntrld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntrldSpec;
impl crate::RegisterSpec for CntrldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntrld::R`](R) reader structure"]
impl crate::Readable for CntrldSpec {}
#[doc = "`write(|w| ..)` method takes [`cntrld::W`](W) writer structure"]
impl crate::Writable for CntrldSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTRLD to value 0"]
impl crate::Resettable for CntrldSpec {
    const RESET_VALUE: u32 = 0;
}
