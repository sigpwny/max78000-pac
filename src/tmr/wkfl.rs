#[doc = "Register `WKFL` reader"]
pub type R = crate::R<WkflSpec>;
#[doc = "Register `WKFL` writer"]
pub type W = crate::W<WkflSpec>;
#[doc = "Field `A` reader - Wake-Up Flag for Timer A"]
pub type AR = crate::BitReader;
#[doc = "Field `A` writer - Wake-Up Flag for Timer A"]
pub type AW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B` reader - Wake-Up Flag for Timer B"]
pub type BR = crate::BitReader;
#[doc = "Field `B` writer - Wake-Up Flag for Timer B"]
pub type BW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wake-Up Flag for Timer A"]
    #[inline(always)]
    pub fn a(&self) -> AR {
        AR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Wake-Up Flag for Timer B"]
    #[inline(always)]
    pub fn b(&self) -> BR {
        BR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-Up Flag for Timer A"]
    #[inline(always)]
    pub fn a(&mut self) -> AW<WkflSpec> {
        AW::new(self, 0)
    }
    #[doc = "Bit 16 - Wake-Up Flag for Timer B"]
    #[inline(always)]
    pub fn b(&mut self) -> BW<WkflSpec> {
        BW::new(self, 16)
    }
}
#[doc = "Timer Wakeup Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`wkfl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkfl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkflSpec;
impl crate::RegisterSpec for WkflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wkfl::R`](R) reader structure"]
impl crate::Readable for WkflSpec {}
#[doc = "`write(|w| ..)` method takes [`wkfl::W`](W) writer structure"]
impl crate::Writable for WkflSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WKFL to value 0"]
impl crate::Resettable for WkflSpec {
    const RESET_VALUE: u32 = 0;
}
