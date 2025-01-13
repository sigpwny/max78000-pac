#[doc = "Register `HSCLK` reader"]
pub type R = crate::R<HsclkSpec>;
#[doc = "Register `HSCLK` writer"]
pub type W = crate::W<HsclkSpec>;
#[doc = "Field `LO` reader - Clock Low. This field sets the Hs-Mode clock low count. In Slave mode, this is the time SCL is held low after data is output on SDA."]
pub type LoR = crate::FieldReader;
#[doc = "Field `LO` writer - Clock Low. This field sets the Hs-Mode clock low count. In Slave mode, this is the time SCL is held low after data is output on SDA."]
pub type LoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HI` reader - Clock High. This field sets the Hs-Mode clock high count. In Slave mode, this is the time SCL is held high after data is output on SDA"]
pub type HiR = crate::FieldReader;
#[doc = "Field `HI` writer - Clock High. This field sets the Hs-Mode clock high count. In Slave mode, this is the time SCL is held high after data is output on SDA"]
pub type HiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock Low. This field sets the Hs-Mode clock low count. In Slave mode, this is the time SCL is held low after data is output on SDA."]
    #[inline(always)]
    pub fn lo(&self) -> LoR {
        LoR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock High. This field sets the Hs-Mode clock high count. In Slave mode, this is the time SCL is held high after data is output on SDA"]
    #[inline(always)]
    pub fn hi(&self) -> HiR {
        HiR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Low. This field sets the Hs-Mode clock low count. In Slave mode, this is the time SCL is held low after data is output on SDA."]
    #[inline(always)]
    pub fn lo(&mut self) -> LoW<HsclkSpec> {
        LoW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Clock High. This field sets the Hs-Mode clock high count. In Slave mode, this is the time SCL is held high after data is output on SDA"]
    #[inline(always)]
    pub fn hi(&mut self) -> HiW<HsclkSpec> {
        HiW::new(self, 8)
    }
}
#[doc = "Clock high Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hsclk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsclk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsclkSpec;
impl crate::RegisterSpec for HsclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsclk::R`](R) reader structure"]
impl crate::Readable for HsclkSpec {}
#[doc = "`write(|w| ..)` method takes [`hsclk::W`](W) writer structure"]
impl crate::Writable for HsclkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSCLK to value 0"]
impl crate::Resettable for HsclkSpec {
    const RESET_VALUE: u32 = 0;
}
