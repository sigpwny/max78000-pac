#[doc = "Register `INT_EN` reader"]
pub type R = crate::R<IntEnSpec>;
#[doc = "Register `INT_EN` writer"]
pub type W = crate::W<IntEnSpec>;
#[doc = "Field `RX_FERR` reader - Enable Interrupt For RX Frame Error"]
pub type RxFerrR = crate::BitReader;
#[doc = "Field `RX_FERR` writer - Enable Interrupt For RX Frame Error"]
pub type RxFerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PAR` reader - Enable Interrupt For RX Parity Error"]
pub type RxParR = crate::BitReader;
#[doc = "Field `RX_PAR` writer - Enable Interrupt For RX Parity Error"]
pub type RxParW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_EV` reader - Enable Interrupt For CTS signal change Error"]
pub type CtsEvR = crate::BitReader;
#[doc = "Field `CTS_EV` writer - Enable Interrupt For CTS signal change Error"]
pub type CtsEvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_OV` reader - Enable Interrupt For RX FIFO Overrun Error"]
pub type RxOvR = crate::BitReader;
#[doc = "Field `RX_OV` writer - Enable Interrupt For RX FIFO Overrun Error"]
pub type RxOvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_THD` reader - Enable Interrupt For RX FIFO reaches the number of bytes configured by RXTHD"]
pub type RxThdR = crate::BitReader;
#[doc = "Field `RX_THD` writer - Enable Interrupt For RX FIFO reaches the number of bytes configured by RXTHD"]
pub type RxThdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_OB` reader - Enable Interrupt For TX FIFO has one byte remaining"]
pub type TxObR = crate::BitReader;
#[doc = "Field `TX_OB` writer - Enable Interrupt For TX FIFO has one byte remaining"]
pub type TxObW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HE` reader - Enable Interrupt For TX FIFO has half empty"]
pub type TxHeR = crate::BitReader;
#[doc = "Field `TX_HE` writer - Enable Interrupt For TX FIFO has half empty"]
pub type TxHeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Interrupt For RX Frame Error"]
    #[inline(always)]
    pub fn rx_ferr(&self) -> RxFerrR {
        RxFerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Interrupt For RX Parity Error"]
    #[inline(always)]
    pub fn rx_par(&self) -> RxParR {
        RxParR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Interrupt For CTS signal change Error"]
    #[inline(always)]
    pub fn cts_ev(&self) -> CtsEvR {
        CtsEvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Interrupt For RX FIFO Overrun Error"]
    #[inline(always)]
    pub fn rx_ov(&self) -> RxOvR {
        RxOvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Interrupt For RX FIFO reaches the number of bytes configured by RXTHD"]
    #[inline(always)]
    pub fn rx_thd(&self) -> RxThdR {
        RxThdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Interrupt For TX FIFO has one byte remaining"]
    #[inline(always)]
    pub fn tx_ob(&self) -> TxObR {
        TxObR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Interrupt For TX FIFO has half empty"]
    #[inline(always)]
    pub fn tx_he(&self) -> TxHeR {
        TxHeR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Interrupt For RX Frame Error"]
    #[inline(always)]
    pub fn rx_ferr(&mut self) -> RxFerrW<IntEnSpec> {
        RxFerrW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Interrupt For RX Parity Error"]
    #[inline(always)]
    pub fn rx_par(&mut self) -> RxParW<IntEnSpec> {
        RxParW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Interrupt For CTS signal change Error"]
    #[inline(always)]
    pub fn cts_ev(&mut self) -> CtsEvW<IntEnSpec> {
        CtsEvW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Interrupt For RX FIFO Overrun Error"]
    #[inline(always)]
    pub fn rx_ov(&mut self) -> RxOvW<IntEnSpec> {
        RxOvW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Interrupt For RX FIFO reaches the number of bytes configured by RXTHD"]
    #[inline(always)]
    pub fn rx_thd(&mut self) -> RxThdW<IntEnSpec> {
        RxThdW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Interrupt For TX FIFO has one byte remaining"]
    #[inline(always)]
    pub fn tx_ob(&mut self) -> TxObW<IntEnSpec> {
        TxObW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Interrupt For TX FIFO has half empty"]
    #[inline(always)]
    pub fn tx_he(&mut self) -> TxHeW<IntEnSpec> {
        TxHeW::new(self, 6)
    }
}
#[doc = "Interrupt Enable control register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnSpec;
impl crate::RegisterSpec for IntEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_en::R`](R) reader structure"]
impl crate::Readable for IntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`int_en::W`](W) writer structure"]
impl crate::Writable for IntEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for IntEnSpec {
    const RESET_VALUE: u32 = 0;
}
