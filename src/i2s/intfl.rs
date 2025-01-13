#[doc = "Register `INTFL` reader"]
pub type R = crate::R<IntflSpec>;
#[doc = "Register `INTFL` writer"]
pub type W = crate::W<IntflSpec>;
#[doc = "Field `RX_OV_CH0` reader - Status for RX FIFO Overrun interrupt."]
pub type RxOvCh0R = crate::BitReader;
#[doc = "Field `RX_OV_CH0` writer - Status for RX FIFO Overrun interrupt."]
pub type RxOvCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_THD_CH0` reader - Status for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
pub type RxThdCh0R = crate::BitReader;
#[doc = "Field `RX_THD_CH0` writer - Status for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
pub type RxThdCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_OB_CH0` reader - Status for interrupt when TX FIFO has only one byte remaining."]
pub type TxObCh0R = crate::BitReader;
#[doc = "Field `TX_OB_CH0` writer - Status for interrupt when TX FIFO has only one byte remaining."]
pub type TxObCh0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HE_CH0` reader - Status for interrupt when TX FIFO is half empty."]
pub type TxHeCh0R = crate::BitReader;
#[doc = "Field `TX_HE_CH0` writer - Status for interrupt when TX FIFO is half empty."]
pub type TxHeCh0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Status for RX FIFO Overrun interrupt."]
    #[inline(always)]
    pub fn rx_ov_ch0(&self) -> RxOvCh0R {
        RxOvCh0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
    #[inline(always)]
    pub fn rx_thd_ch0(&self) -> RxThdCh0R {
        RxThdCh0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status for interrupt when TX FIFO has only one byte remaining."]
    #[inline(always)]
    pub fn tx_ob_ch0(&self) -> TxObCh0R {
        TxObCh0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status for interrupt when TX FIFO is half empty."]
    #[inline(always)]
    pub fn tx_he_ch0(&self) -> TxHeCh0R {
        TxHeCh0R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Status for RX FIFO Overrun interrupt."]
    #[inline(always)]
    pub fn rx_ov_ch0(&mut self) -> RxOvCh0W<IntflSpec> {
        RxOvCh0W::new(self, 0)
    }
    #[doc = "Bit 1 - Status for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field."]
    #[inline(always)]
    pub fn rx_thd_ch0(&mut self) -> RxThdCh0W<IntflSpec> {
        RxThdCh0W::new(self, 1)
    }
    #[doc = "Bit 2 - Status for interrupt when TX FIFO has only one byte remaining."]
    #[inline(always)]
    pub fn tx_ob_ch0(&mut self) -> TxObCh0W<IntflSpec> {
        TxObCh0W::new(self, 2)
    }
    #[doc = "Bit 3 - Status for interrupt when TX FIFO is half empty."]
    #[inline(always)]
    pub fn tx_he_ch0(&mut self) -> TxHeCh0W<IntflSpec> {
        TxHeCh0W::new(self, 3)
    }
}
#[doc = "ISR Status.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflSpec;
impl crate::RegisterSpec for IntflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl::R`](R) reader structure"]
impl crate::Readable for IntflSpec {}
#[doc = "`write(|w| ..)` method takes [`intfl::W`](W) writer structure"]
impl crate::Writable for IntflSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for IntflSpec {
    const RESET_VALUE: u32 = 0;
}
