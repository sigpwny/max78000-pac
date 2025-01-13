#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `DONE` reader - AES Done Interrupt Enable"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - AES Done Interrupt Enable"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_CHANGE` reader - External AES Key Changed Interrupt Enable"]
pub type KeyChangeR = crate::BitReader;
#[doc = "Field `KEY_CHANGE` writer - External AES Key Changed Interrupt Enable"]
pub type KeyChangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_ZERO` reader - External AES Key Zero Interrupt Enable"]
pub type KeyZeroR = crate::BitReader;
#[doc = "Field `KEY_ZERO` writer - External AES Key Zero Interrupt Enable"]
pub type KeyZeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OV` reader - Data Output FIFO Overrun Interrupt Enable"]
pub type OvR = crate::BitReader;
#[doc = "Field `OV` writer - Data Output FIFO Overrun Interrupt Enable"]
pub type OvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_ONE` reader - KEY_ONE"]
pub type KeyOneR = crate::BitReader;
#[doc = "Field `KEY_ONE` writer - KEY_ONE"]
pub type KeyOneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AES Done Interrupt Enable"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External AES Key Changed Interrupt Enable"]
    #[inline(always)]
    pub fn key_change(&self) -> KeyChangeR {
        KeyChangeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External AES Key Zero Interrupt Enable"]
    #[inline(always)]
    pub fn key_zero(&self) -> KeyZeroR {
        KeyZeroR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Output FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ov(&self) -> OvR {
        OvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - KEY_ONE"]
    #[inline(always)]
    pub fn key_one(&self) -> KeyOneR {
        KeyOneR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Done Interrupt Enable"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<IntenSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - External AES Key Changed Interrupt Enable"]
    #[inline(always)]
    pub fn key_change(&mut self) -> KeyChangeW<IntenSpec> {
        KeyChangeW::new(self, 1)
    }
    #[doc = "Bit 2 - External AES Key Zero Interrupt Enable"]
    #[inline(always)]
    pub fn key_zero(&mut self) -> KeyZeroW<IntenSpec> {
        KeyZeroW::new(self, 2)
    }
    #[doc = "Bit 3 - Data Output FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn ov(&mut self) -> OvW<IntenSpec> {
        OvW::new(self, 3)
    }
    #[doc = "Bit 4 - KEY_ONE"]
    #[inline(always)]
    pub fn key_one(&mut self) -> KeyOneW<IntenSpec> {
        KeyOneW::new(self, 4)
    }
}
#[doc = "AES Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
