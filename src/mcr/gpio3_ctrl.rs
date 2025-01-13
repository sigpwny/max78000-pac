#[doc = "Register `GPIO3_CTRL` reader"]
pub type R = crate::R<Gpio3CtrlSpec>;
#[doc = "Register `GPIO3_CTRL` writer"]
pub type W = crate::W<Gpio3CtrlSpec>;
#[doc = "Field `P30_DO` reader - GPIO3 Pin 0 Data Output."]
pub type P30DoR = crate::BitReader;
#[doc = "Field `P30_DO` writer - GPIO3 Pin 0 Data Output."]
pub type P30DoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30_OE` reader - GPIO3 Pin 0 Output Enable."]
pub type P30OeR = crate::BitReader;
#[doc = "Field `P30_OE` writer - GPIO3 Pin 0 Output Enable."]
pub type P30OeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30_PE` reader - GPIO3 Pin 0 Pull-up Enable."]
pub type P30PeR = crate::BitReader;
#[doc = "Field `P30_PE` writer - GPIO3 Pin 0 Pull-up Enable."]
pub type P30PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P30_IN` reader - GPIO3 Pin 0 Input Status."]
pub type P30InR = crate::BitReader;
#[doc = "Field `P30_IN` writer - GPIO3 Pin 0 Input Status."]
pub type P30InW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31_DO` reader - GPIO3 Pin 1 Data Output."]
pub type P31DoR = crate::BitReader;
#[doc = "Field `P31_DO` writer - GPIO3 Pin 1 Data Output."]
pub type P31DoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31_OE` reader - GPIO3 Pin 1 Output Enable."]
pub type P31OeR = crate::BitReader;
#[doc = "Field `P31_OE` writer - GPIO3 Pin 1 Output Enable."]
pub type P31OeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31_PE` reader - GPIO3 Pin 1 Pull-up Enable."]
pub type P31PeR = crate::BitReader;
#[doc = "Field `P31_PE` writer - GPIO3 Pin 1 Pull-up Enable."]
pub type P31PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31_IN` reader - GPIO3 Pin 1 Input Status."]
pub type P31InR = crate::BitReader;
#[doc = "Field `P31_IN` writer - GPIO3 Pin 1 Input Status."]
pub type P31InW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO3 Pin 0 Data Output."]
    #[inline(always)]
    pub fn p30_do(&self) -> P30DoR {
        P30DoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO3 Pin 0 Output Enable."]
    #[inline(always)]
    pub fn p30_oe(&self) -> P30OeR {
        P30OeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO3 Pin 0 Pull-up Enable."]
    #[inline(always)]
    pub fn p30_pe(&self) -> P30PeR {
        P30PeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO3 Pin 0 Input Status."]
    #[inline(always)]
    pub fn p30_in(&self) -> P30InR {
        P30InR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO3 Pin 1 Data Output."]
    #[inline(always)]
    pub fn p31_do(&self) -> P31DoR {
        P31DoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO3 Pin 1 Output Enable."]
    #[inline(always)]
    pub fn p31_oe(&self) -> P31OeR {
        P31OeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO3 Pin 1 Pull-up Enable."]
    #[inline(always)]
    pub fn p31_pe(&self) -> P31PeR {
        P31PeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO3 Pin 1 Input Status."]
    #[inline(always)]
    pub fn p31_in(&self) -> P31InR {
        P31InR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO3 Pin 0 Data Output."]
    #[inline(always)]
    pub fn p30_do(&mut self) -> P30DoW<Gpio3CtrlSpec> {
        P30DoW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO3 Pin 0 Output Enable."]
    #[inline(always)]
    pub fn p30_oe(&mut self) -> P30OeW<Gpio3CtrlSpec> {
        P30OeW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO3 Pin 0 Pull-up Enable."]
    #[inline(always)]
    pub fn p30_pe(&mut self) -> P30PeW<Gpio3CtrlSpec> {
        P30PeW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO3 Pin 0 Input Status."]
    #[inline(always)]
    pub fn p30_in(&mut self) -> P30InW<Gpio3CtrlSpec> {
        P30InW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO3 Pin 1 Data Output."]
    #[inline(always)]
    pub fn p31_do(&mut self) -> P31DoW<Gpio3CtrlSpec> {
        P31DoW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO3 Pin 1 Output Enable."]
    #[inline(always)]
    pub fn p31_oe(&mut self) -> P31OeW<Gpio3CtrlSpec> {
        P31OeW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO3 Pin 1 Pull-up Enable."]
    #[inline(always)]
    pub fn p31_pe(&mut self) -> P31PeW<Gpio3CtrlSpec> {
        P31PeW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO3 Pin 1 Input Status."]
    #[inline(always)]
    pub fn p31_in(&mut self) -> P31InW<Gpio3CtrlSpec> {
        P31InW::new(self, 7)
    }
}
#[doc = "GPIO3 Pin Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio3CtrlSpec;
impl crate::RegisterSpec for Gpio3CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio3_ctrl::R`](R) reader structure"]
impl crate::Readable for Gpio3CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio3_ctrl::W`](W) writer structure"]
impl crate::Writable for Gpio3CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO3_CTRL to value 0"]
impl crate::Resettable for Gpio3CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
