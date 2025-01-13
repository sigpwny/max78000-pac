#[doc = "Register `PNR` reader"]
pub type R = crate::R<PnrSpec>;
#[doc = "Register `PNR` writer"]
pub type W = crate::W<PnrSpec>;
#[doc = "Field `CTS` reader - Current sampled value of CTS IO"]
pub type CtsR = crate::BitReader;
#[doc = "Field `RTS` reader - This bit controls the value to apply on the RTS IO. If set to 1, the RTS IO is set to high level. If set to 0, the RTS IO is set to low level."]
pub type RtsR = crate::BitReader;
#[doc = "Field `RTS` writer - This bit controls the value to apply on the RTS IO. If set to 1, the RTS IO is set to high level. If set to 0, the RTS IO is set to low level."]
pub type RtsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Current sampled value of CTS IO"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit controls the value to apply on the RTS IO. If set to 1, the RTS IO is set to high level. If set to 0, the RTS IO is set to low level."]
    #[inline(always)]
    pub fn rts(&self) -> RtsR {
        RtsR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This bit controls the value to apply on the RTS IO. If set to 1, the RTS IO is set to high level. If set to 0, the RTS IO is set to low level."]
    #[inline(always)]
    pub fn rts(&mut self) -> RtsW<PnrSpec> {
        RtsW::new(self, 1)
    }
}
#[doc = "Pin register\n\nYou can [`read`](crate::Reg::read) this register and get [`pnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PnrSpec;
impl crate::RegisterSpec for PnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pnr::R`](R) reader structure"]
impl crate::Readable for PnrSpec {}
#[doc = "`write(|w| ..)` method takes [`pnr::W`](W) writer structure"]
impl crate::Writable for PnrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PNR to value 0"]
impl crate::Resettable for PnrSpec {
    const RESET_VALUE: u32 = 0;
}
