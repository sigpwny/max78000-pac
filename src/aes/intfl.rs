#[doc = "Register `INTFL` reader"]
pub type R = crate::R<IntflSpec>;
#[doc = "Register `INTFL` writer"]
pub type W = crate::W<IntflSpec>;
#[doc = "Field `DONE` reader - AES Done Interrupt"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - AES Done Interrupt"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_CHANGE` reader - External AES Key Changed Interrupt"]
pub type KeyChangeR = crate::BitReader;
#[doc = "Field `KEY_CHANGE` writer - External AES Key Changed Interrupt"]
pub type KeyChangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_ZERO` reader - External AES Key Zero Interrupt"]
pub type KeyZeroR = crate::BitReader;
#[doc = "Field `KEY_ZERO` writer - External AES Key Zero Interrupt"]
pub type KeyZeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OV` reader - Data Output FIFO Overrun Interrupt"]
pub type OvR = crate::BitReader;
#[doc = "Field `OV` writer - Data Output FIFO Overrun Interrupt"]
pub type OvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_ONE` reader - KEY_ONE"]
pub type KeyOneR = crate::BitReader;
#[doc = "Field `KEY_ONE` writer - KEY_ONE"]
pub type KeyOneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AES Done Interrupt"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External AES Key Changed Interrupt"]
    #[inline(always)]
    pub fn key_change(&self) -> KeyChangeR {
        KeyChangeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External AES Key Zero Interrupt"]
    #[inline(always)]
    pub fn key_zero(&self) -> KeyZeroR {
        KeyZeroR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Output FIFO Overrun Interrupt"]
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
    #[doc = "Bit 0 - AES Done Interrupt"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<IntflSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - External AES Key Changed Interrupt"]
    #[inline(always)]
    pub fn key_change(&mut self) -> KeyChangeW<IntflSpec> {
        KeyChangeW::new(self, 1)
    }
    #[doc = "Bit 2 - External AES Key Zero Interrupt"]
    #[inline(always)]
    pub fn key_zero(&mut self) -> KeyZeroW<IntflSpec> {
        KeyZeroW::new(self, 2)
    }
    #[doc = "Bit 3 - Data Output FIFO Overrun Interrupt"]
    #[inline(always)]
    pub fn ov(&mut self) -> OvW<IntflSpec> {
        OvW::new(self, 3)
    }
    #[doc = "Bit 4 - KEY_ONE"]
    #[inline(always)]
    pub fn key_one(&mut self) -> KeyOneW<IntflSpec> {
        KeyOneW::new(self, 4)
    }
}
#[doc = "AES Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
