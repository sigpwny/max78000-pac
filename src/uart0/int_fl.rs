#[doc = "Register `INT_FL` reader"]
pub type R = crate::R<IntFlSpec>;
#[doc = "Register `INT_FL` writer"]
pub type W = crate::W<IntFlSpec>;
#[doc = "Field `RX_FERR` reader - Flag for RX Frame Error Interrupt."]
pub type RxFerrR = crate::BitReader;
#[doc = "Field `RX_FERR` writer - Flag for RX Frame Error Interrupt."]
pub type RxFerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PAR` reader - Flag for RX Parity Error interrupt"]
pub type RxParR = crate::BitReader;
#[doc = "Field `RX_PAR` writer - Flag for RX Parity Error interrupt"]
pub type RxParW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_EV` reader - Flag for CTS signal change interrupt (hardware flow control disabled)"]
pub type CtsEvR = crate::BitReader;
#[doc = "Field `CTS_EV` writer - Flag for CTS signal change interrupt (hardware flow control disabled)"]
pub type CtsEvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OV` reader - Flag for RX FIFO Overrun interrupt"]
pub type RxOvR = crate::BitReader;
#[doc = "Field `RX_OV` writer - Flag for RX FIFO Overrun interrupt"]
pub type RxOvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_THD` reader - Flag for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field"]
pub type RxThdR = crate::BitReader;
#[doc = "Field `RX_THD` writer - Flag for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field"]
pub type RxThdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_OB` reader - Flag for interrupt when TX FIFO has one byte remaining"]
pub type TxObR = crate::BitReader;
#[doc = "Field `TX_OB` writer - Flag for interrupt when TX FIFO has one byte remaining"]
pub type TxObW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HE` reader - Flag for interrupt when TX FIFO is half empty"]
pub type TxHeR = crate::BitReader;
#[doc = "Field `TX_HE` writer - Flag for interrupt when TX FIFO is half empty"]
pub type TxHeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flag for RX Frame Error Interrupt."]
    #[inline(always)]
    pub fn rx_ferr(&self) -> RxFerrR {
        RxFerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flag for RX Parity Error interrupt"]
    #[inline(always)]
    pub fn rx_par(&self) -> RxParR {
        RxParR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flag for CTS signal change interrupt (hardware flow control disabled)"]
    #[inline(always)]
    pub fn cts_ev(&self) -> CtsEvR {
        CtsEvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flag for RX FIFO Overrun interrupt"]
    #[inline(always)]
    pub fn rx_ov(&self) -> RxOvR {
        RxOvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flag for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field"]
    #[inline(always)]
    pub fn rx_thd(&self) -> RxThdR {
        RxThdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flag for interrupt when TX FIFO has one byte remaining"]
    #[inline(always)]
    pub fn tx_ob(&self) -> TxObR {
        TxObR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Flag for interrupt when TX FIFO is half empty"]
    #[inline(always)]
    pub fn tx_he(&self) -> TxHeR {
        TxHeR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flag for RX Frame Error Interrupt."]
    #[inline(always)]
    pub fn rx_ferr(&mut self) -> RxFerrW<IntFlSpec> {
        RxFerrW::new(self, 0)
    }
    #[doc = "Bit 1 - Flag for RX Parity Error interrupt"]
    #[inline(always)]
    pub fn rx_par(&mut self) -> RxParW<IntFlSpec> {
        RxParW::new(self, 1)
    }
    #[doc = "Bit 2 - Flag for CTS signal change interrupt (hardware flow control disabled)"]
    #[inline(always)]
    pub fn cts_ev(&mut self) -> CtsEvW<IntFlSpec> {
        CtsEvW::new(self, 2)
    }
    #[doc = "Bit 3 - Flag for RX FIFO Overrun interrupt"]
    #[inline(always)]
    pub fn rx_ov(&mut self) -> RxOvW<IntFlSpec> {
        RxOvW::new(self, 3)
    }
    #[doc = "Bit 4 - Flag for interrupt when RX FIFO reaches the number of bytes configured by the RXTHD field"]
    #[inline(always)]
    pub fn rx_thd(&mut self) -> RxThdW<IntFlSpec> {
        RxThdW::new(self, 4)
    }
    #[doc = "Bit 5 - Flag for interrupt when TX FIFO has one byte remaining"]
    #[inline(always)]
    pub fn tx_ob(&mut self) -> TxObW<IntFlSpec> {
        TxObW::new(self, 5)
    }
    #[doc = "Bit 6 - Flag for interrupt when TX FIFO is half empty"]
    #[inline(always)]
    pub fn tx_he(&mut self) -> TxHeW<IntFlSpec> {
        TxHeW::new(self, 6)
    }
}
#[doc = "Interrupt status flags Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_fl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_fl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
