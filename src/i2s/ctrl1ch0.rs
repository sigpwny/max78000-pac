#[doc = "Register `CTRL1CH0` reader"]
pub type R = crate::R<Ctrl1ch0Spec>;
#[doc = "Register `CTRL1CH0` writer"]
pub type W = crate::W<Ctrl1ch0Spec>;
#[doc = "Field `BITS_WORD` reader - I2S word length."]
pub type BitsWordR = crate::FieldReader;
#[doc = "Field `BITS_WORD` writer - I2S word length."]
pub type BitsWordW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EN` reader - I2S clock enable."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - I2S clock enable."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMP_SIZE` reader - I2S sample size length."]
pub type SmpSizeR = crate::FieldReader;
#[doc = "Field `SMP_SIZE` writer - I2S sample size length."]
pub type SmpSizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADJUST` reader - LSB/MSB Justify."]
pub type AdjustR = crate::BitReader;
#[doc = "Field `ADJUST` writer - LSB/MSB Justify."]
pub type AdjustW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDIV` reader - I2S clock frequency divisor."]
pub type ClkdivR = crate::FieldReader<u16>;
#[doc = "Field `CLKDIV` writer - I2S clock frequency divisor."]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - I2S word length."]
    #[inline(always)]
    pub fn bits_word(&self) -> BitsWordR {
        BitsWordR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - I2S clock enable."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:13 - I2S sample size length."]
    #[inline(always)]
    pub fn smp_size(&self) -> SmpSizeR {
        SmpSizeR::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - LSB/MSB Justify."]
    #[inline(always)]
    pub fn adjust(&self) -> AdjustR {
        AdjustR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - I2S clock frequency divisor."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - I2S word length."]
    #[inline(always)]
    pub fn bits_word(&mut self) -> BitsWordW<Ctrl1ch0Spec> {
        BitsWordW::new(self, 0)
    }
    #[doc = "Bit 8 - I2S clock enable."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<Ctrl1ch0Spec> {
        EnW::new(self, 8)
    }
    #[doc = "Bits 9:13 - I2S sample size length."]
    #[inline(always)]
    pub fn smp_size(&mut self) -> SmpSizeW<Ctrl1ch0Spec> {
        SmpSizeW::new(self, 9)
    }
    #[doc = "Bit 15 - LSB/MSB Justify."]
    #[inline(always)]
    pub fn adjust(&mut self) -> AdjustW<Ctrl1ch0Spec> {
        AdjustW::new(self, 15)
    }
    #[doc = "Bits 16:31 - I2S clock frequency divisor."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<Ctrl1ch0Spec> {
        ClkdivW::new(self, 16)
    }
}
#[doc = "Local channel Setup.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1ch0Spec;
impl crate::RegisterSpec for Ctrl1ch0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1ch0::R`](R) reader structure"]
impl crate::Readable for Ctrl1ch0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1ch0::W`](W) writer structure"]
impl crate::Writable for Ctrl1ch0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1CH0 to value 0"]
impl crate::Resettable for Ctrl1ch0Spec {
    const RESET_VALUE: u32 = 0;
}
