#[doc = "Register `WKEN` reader"]
pub type R = crate::R<WkenSpec>;
#[doc = "Register `WKEN` writer"]
pub type W = crate::W<WkenSpec>;
#[doc = "Field `RX_NE` reader - Wake-Up Enable for RX FIFO Not Empty"]
pub type RxNeR = crate::BitReader;
#[doc = "Field `RX_NE` writer - Wake-Up Enable for RX FIFO Not Empty"]
pub type RxNeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FULL` reader - Wake-Up Enable for RX FIFO Full"]
pub type RxFullR = crate::BitReader;
#[doc = "Field `RX_FULL` writer - Wake-Up Enable for RX FIFO Full"]
pub type RxFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_THD` reader - Wake-Up Enable for RX FIFO Threshold Met"]
pub type RxThdR = crate::BitReader;
#[doc = "Field `RX_THD` writer - Wake-Up Enable for RX FIFO Threshold Met"]
pub type RxThdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wake-Up Enable for RX FIFO Not Empty"]
    #[inline(always)]
    pub fn rx_ne(&self) -> RxNeR {
        RxNeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-Up Enable for RX FIFO Full"]
    #[inline(always)]
    pub fn rx_full(&self) -> RxFullR {
        RxFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-Up Enable for RX FIFO Threshold Met"]
    #[inline(always)]
    pub fn rx_thd(&self) -> RxThdR {
        RxThdR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-Up Enable for RX FIFO Not Empty"]
    #[inline(always)]
    pub fn rx_ne(&mut self) -> RxNeW<WkenSpec> {
        RxNeW::new(self, 0)
    }
    #[doc = "Bit 1 - Wake-Up Enable for RX FIFO Full"]
    #[inline(always)]
    pub fn rx_full(&mut self) -> RxFullW<WkenSpec> {
        RxFullW::new(self, 1)
    }
    #[doc = "Bit 2 - Wake-Up Enable for RX FIFO Threshold Met"]
    #[inline(always)]
    pub fn rx_thd(&mut self) -> RxThdW<WkenSpec> {
        RxThdW::new(self, 2)
    }
}
#[doc = "Wake up enable Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`wken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkenSpec;
impl crate::RegisterSpec for WkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wken::R`](R) reader structure"]
impl crate::Readable for WkenSpec {}
#[doc = "`write(|w| ..)` method takes [`wken::W`](W) writer structure"]
impl crate::Writable for WkenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WKEN to value 0"]
impl crate::Resettable for WkenSpec {
    const RESET_VALUE: u32 = 0;
}
