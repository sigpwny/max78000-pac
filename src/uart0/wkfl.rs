#[doc = "Register `WKFL` reader"]
pub type R = crate::R<WkflSpec>;
#[doc = "Register `WKFL` writer"]
pub type W = crate::W<WkflSpec>;
#[doc = "Field `RX_NE` reader - Wake-Up Flag for RX FIFO Not Empty"]
pub type RxNeR = crate::BitReader;
#[doc = "Field `RX_NE` writer - Wake-Up Flag for RX FIFO Not Empty"]
pub type RxNeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FULL` reader - Wake-Up Flag for RX FIFO Full"]
pub type RxFullR = crate::BitReader;
#[doc = "Field `RX_FULL` writer - Wake-Up Flag for RX FIFO Full"]
pub type RxFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_THD` reader - Wake-Up Flag for RX FIFO Threshold Met"]
pub type RxThdR = crate::BitReader;
#[doc = "Field `RX_THD` writer - Wake-Up Flag for RX FIFO Threshold Met"]
pub type RxThdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wake-Up Flag for RX FIFO Not Empty"]
    #[inline(always)]
    pub fn rx_ne(&self) -> RxNeR {
        RxNeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-Up Flag for RX FIFO Full"]
    #[inline(always)]
    pub fn rx_full(&self) -> RxFullR {
        RxFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-Up Flag for RX FIFO Threshold Met"]
    #[inline(always)]
    pub fn rx_thd(&self) -> RxThdR {
        RxThdR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-Up Flag for RX FIFO Not Empty"]
    #[inline(always)]
    pub fn rx_ne(&mut self) -> RxNeW<WkflSpec> {
        RxNeW::new(self, 0)
    }
    #[doc = "Bit 1 - Wake-Up Flag for RX FIFO Full"]
    #[inline(always)]
    pub fn rx_full(&mut self) -> RxFullW<WkflSpec> {
        RxFullW::new(self, 1)
    }
    #[doc = "Bit 2 - Wake-Up Flag for RX FIFO Threshold Met"]
    #[inline(always)]
    pub fn rx_thd(&mut self) -> RxThdW<WkflSpec> {
        RxThdW::new(self, 2)
    }
}
#[doc = "Wake up Flags register\n\nYou can [`read`](crate::Reg::read) this register and get [`wkfl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkfl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkflSpec;
impl crate::RegisterSpec for WkflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wkfl::R`](R) reader structure"]
impl crate::Readable for WkflSpec {}
#[doc = "`write(|w| ..)` method takes [`wkfl::W`](W) writer structure"]
impl crate::Writable for WkflSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WKFL to value 0"]
impl crate::Resettable for WkflSpec {
    const RESET_VALUE: u32 = 0;
}
