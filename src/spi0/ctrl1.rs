#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `TX_NUM_CHAR` reader - Nubmer of Characters to transmit."]
pub type TxNumCharR = crate::FieldReader<u16>;
#[doc = "Field `TX_NUM_CHAR` writer - Nubmer of Characters to transmit."]
pub type TxNumCharW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RX_NUM_CHAR` reader - Nubmer of Characters to receive."]
pub type RxNumCharR = crate::FieldReader<u16>;
#[doc = "Field `RX_NUM_CHAR` writer - Nubmer of Characters to receive."]
pub type RxNumCharW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Nubmer of Characters to transmit."]
    #[inline(always)]
    pub fn tx_num_char(&self) -> TxNumCharR {
        TxNumCharR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Nubmer of Characters to receive."]
    #[inline(always)]
    pub fn rx_num_char(&self) -> RxNumCharR {
        RxNumCharR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Nubmer of Characters to transmit."]
    #[inline(always)]
    pub fn tx_num_char(&mut self) -> TxNumCharW<Ctrl1Spec> {
        TxNumCharW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Nubmer of Characters to receive."]
    #[inline(always)]
    pub fn rx_num_char(&mut self) -> RxNumCharW<Ctrl1Spec> {
        RxNumCharW::new(self, 16)
    }
}
#[doc = "Register for controlling SPI peripheral.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
