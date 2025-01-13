#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `RX_OV_CH0` reader - Enable for RX FIFO Overrun interrupt."]
pub type RxOvCh0R = crate::BitReader;
#[doc = "Field `RX_OV_CH0` writer - Enable for RX FIFO Overrun interrupt."]
pub type RxOvCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_THD_CH0` reader - Enable for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
pub type RxThdCh0R = crate::BitReader;
#[doc = "Field `RX_THD_CH0` writer - Enable for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
pub type RxThdCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_OB_CH0` reader - Enable for interrupt when TX FIFO has only one byte remaining."]
pub type TxObCh0R = crate::BitReader;
#[doc = "Field `TX_OB_CH0` writer - Enable for interrupt when TX FIFO has only one byte remaining."]
pub type TxObCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HE_CH0` reader - Enable for interrupt when TX FIFO is half empty."]
pub type TxHeCh0R = crate::BitReader;
#[doc = "Field `TX_HE_CH0` writer - Enable for interrupt when TX FIFO is half empty."]
pub type TxHeCh0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable for RX FIFO Overrun interrupt."]
    #[inline(always)]
    pub fn rx_ov_ch0(&self) -> RxOvCh0R {
        RxOvCh0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
    #[inline(always)]
    pub fn rx_thd_ch0(&self) -> RxThdCh0R {
        RxThdCh0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable for interrupt when TX FIFO has only one byte remaining."]
    #[inline(always)]
    pub fn tx_ob_ch0(&self) -> TxObCh0R {
        TxObCh0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable for interrupt when TX FIFO is half empty."]
    #[inline(always)]
    pub fn tx_he_ch0(&self) -> TxHeCh0R {
        TxHeCh0R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable for RX FIFO Overrun interrupt."]
    #[inline(always)]
    pub fn rx_ov_ch0(&mut self) -> RxOvCh0W<IntenSpec> {
        RxOvCh0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
    #[inline(always)]
    pub fn rx_thd_ch0(&mut self) -> RxThdCh0W<IntenSpec> {
        RxThdCh0W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable for interrupt when TX FIFO has only one byte remaining."]
    #[inline(always)]
    pub fn tx_ob_ch0(&mut self) -> TxObCh0W<IntenSpec> {
        TxObCh0W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable for interrupt when TX FIFO is half empty."]
    #[inline(always)]
    pub fn tx_he_ch0(&mut self) -> TxHeCh0W<IntenSpec> {
        TxHeCh0W::new(self, 3)
    }
}
#[doc = "Interrupt Enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
