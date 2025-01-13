#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `BUSY` reader - AES Busy Status"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - AES Busy Status"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_EM` reader - Data input FIFO empty status"]
pub type InputEmR = crate::BitReader;
#[doc = "Field `INPUT_EM` writer - Data input FIFO empty status"]
pub type InputEmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_FULL` reader - Data input FIFO full status"]
pub type InputFullR = crate::BitReader;
#[doc = "Field `INPUT_FULL` writer - Data input FIFO full status"]
pub type InputFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPUT_EM` reader - Data output FIFO empty status"]
pub type OutputEmR = crate::BitReader;
#[doc = "Field `OUTPUT_EM` writer - Data output FIFO empty status"]
pub type OutputEmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPUT_FULL` reader - Data output FIFO full status"]
pub type OutputFullR = crate::BitReader;
#[doc = "Field `OUTPUT_FULL` writer - Data output FIFO full status"]
pub type OutputFullW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AES Busy Status"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data input FIFO empty status"]
    #[inline(always)]
    pub fn input_em(&self) -> InputEmR {
        InputEmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data input FIFO full status"]
    #[inline(always)]
    pub fn input_full(&self) -> InputFullR {
        InputFullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data output FIFO empty status"]
    #[inline(always)]
    pub fn output_em(&self) -> OutputEmR {
        OutputEmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data output FIFO full status"]
    #[inline(always)]
    pub fn output_full(&self) -> OutputFullR {
        OutputFullR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Busy Status"]
    #[inline(always)]
    pub fn busy(&mut self) -> BusyW<StatusSpec> {
        BusyW::new(self, 0)
    }
    #[doc = "Bit 1 - Data input FIFO empty status"]
    #[inline(always)]
    pub fn input_em(&mut self) -> InputEmW<StatusSpec> {
        InputEmW::new(self, 1)
    }
    #[doc = "Bit 2 - Data input FIFO full status"]
    #[inline(always)]
    pub fn input_full(&mut self) -> InputFullW<StatusSpec> {
        InputFullW::new(self, 2)
    }
    #[doc = "Bit 3 - Data output FIFO empty status"]
    #[inline(always)]
    pub fn output_em(&mut self) -> OutputEmW<StatusSpec> {
        OutputEmW::new(self, 3)
    }
    #[doc = "Bit 4 - Data output FIFO full status"]
    #[inline(always)]
    pub fn output_full(&mut self) -> OutputFullW<StatusSpec> {
        OutputFullW::new(self, 4)
    }
}
#[doc = "AES Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
