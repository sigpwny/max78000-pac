#[doc = "Register `INT_FL` reader"]
pub type R = crate::R<IntFlSpec>;
#[doc = "Register `INT_FL` writer"]
pub type W = crate::W<IntFlSpec>;
#[doc = "Field `IMG_DONE` reader - Image Done."]
pub type ImgDoneR = crate::BitReader;
#[doc = "Field `IMG_DONE` writer - Image Done."]
pub type ImgDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_FULL` reader - FIFO Full."]
pub type FifoFullR = crate::BitReader;
#[doc = "Field `FIFO_FULL` writer - FIFO Full."]
pub type FifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_THRESH` reader - FIFO Threshold Level Met."]
pub type FifoThreshR = crate::BitReader;
#[doc = "Field `FIFO_THRESH` writer - FIFO Threshold Level Met."]
pub type FifoThreshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_NOT_EMPTY` reader - FIFO Not Empty."]
pub type FifoNotEmptyR = crate::BitReader;
#[doc = "Field `FIFO_NOT_EMPTY` writer - FIFO Not Empty."]
pub type FifoNotEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Image Done."]
    #[inline(always)]
    pub fn img_done(&self) -> ImgDoneR {
        ImgDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Full."]
    #[inline(always)]
    pub fn fifo_full(&self) -> FifoFullR {
        FifoFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO Threshold Level Met."]
    #[inline(always)]
    pub fn fifo_thresh(&self) -> FifoThreshR {
        FifoThreshR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO Not Empty."]
    #[inline(always)]
    pub fn fifo_not_empty(&self) -> FifoNotEmptyR {
        FifoNotEmptyR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Image Done."]
    #[inline(always)]
    pub fn img_done(&mut self) -> ImgDoneW<IntFlSpec> {
        ImgDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO Full."]
    #[inline(always)]
    pub fn fifo_full(&mut self) -> FifoFullW<IntFlSpec> {
        FifoFullW::new(self, 1)
    }
    #[doc = "Bit 2 - FIFO Threshold Level Met."]
    #[inline(always)]
    pub fn fifo_thresh(&mut self) -> FifoThreshW<IntFlSpec> {
        FifoThreshW::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO Not Empty."]
    #[inline(always)]
    pub fn fifo_not_empty(&mut self) -> FifoNotEmptyW<IntFlSpec> {
        FifoNotEmptyW::new(self, 3)
    }
}
#[doc = "Interupt Flag Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_fl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_fl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntFlSpec;
impl crate::RegisterSpec for IntFlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_fl::R`](R) reader structure"]
impl crate::Readable for IntFlSpec {}
#[doc = "`write(|w| ..)` method takes [`int_fl::W`](W) writer structure"]
impl crate::Writable for IntFlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_FL to value 0"]
impl crate::Resettable for IntFlSpec {
    const RESET_VALUE: u32 = 0;
}
