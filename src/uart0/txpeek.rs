#[doc = "Register `TXPEEK` reader"]
pub type R = crate::R<TxpeekSpec>;
#[doc = "Register `TXPEEK` writer"]
pub type W = crate::W<TxpeekSpec>;
#[doc = "Field `DATA` reader - Read TX FIFO next data. Reading from this field does not affect the contents of TX FIFO. Note that the parity bit is available from this field."]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Read TX FIFO next data. Reading from this field does not affect the contents of TX FIFO. Note that the parity bit is available from this field."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Read TX FIFO next data. Reading from this field does not affect the contents of TX FIFO. Note that the parity bit is available from this field."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read TX FIFO next data. Reading from this field does not affect the contents of TX FIFO. Note that the parity bit is available from this field."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<TxpeekSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "TX FIFO Output Peek register\n\nYou can [`read`](crate::Reg::read) this register and get [`txpeek::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpeek::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxpeekSpec;
impl crate::RegisterSpec for TxpeekSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txpeek::R`](R) reader structure"]
impl crate::Readable for TxpeekSpec {}
#[doc = "`write(|w| ..)` method takes [`txpeek::W`](W) writer structure"]
impl crate::Writable for TxpeekSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXPEEK to value 0"]
impl crate::Resettable for TxpeekSpec {
    const RESET_VALUE: u32 = 0;
}
