#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Channel 0 Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0 {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<Ch0> for bool {
    #[inline(always)]
    fn from(variant: Ch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` reader - Channel 0 Interrupt Enable."]
pub type Ch0R = crate::BitReader<Ch0>;
impl Ch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0 {
        match self.bits {
            false => Ch0::Dis,
            true => Ch0::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ch0::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ch0::En
    }
}
#[doc = "Field `CH0` writer - Channel 0 Interrupt Enable."]
pub type Ch0W<'a, REG> = crate::BitWriter<'a, REG, Ch0>;
impl<'a, REG> Ch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::Dis)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0::En)
    }
}
#[doc = "Channel 1 Interrupt Enable."]
pub use Ch0 as Ch1;
#[doc = "Channel 2 Interrupt Enable."]
pub use Ch0 as Ch2;
#[doc = "Channel 3 Interrupt Enable."]
pub use Ch0 as Ch3;
#[doc = "Field `CH1` reader - Channel 1 Interrupt Enable."]
pub use Ch0R as Ch1R;
#[doc = "Field `CH2` reader - Channel 2 Interrupt Enable."]
pub use Ch0R as Ch2R;
#[doc = "Field `CH3` reader - Channel 3 Interrupt Enable."]
pub use Ch0R as Ch3R;
#[doc = "Field `CH1` writer - Channel 1 Interrupt Enable."]
pub use Ch0W as Ch1W;
#[doc = "Field `CH2` writer - Channel 2 Interrupt Enable."]
pub use Ch0W as Ch2W;
#[doc = "Field `CH3` writer - Channel 3 Interrupt Enable."]
pub use Ch0W as Ch3W;
impl R {
    #[doc = "Bit 0 - Channel 0 Interrupt Enable."]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Interrupt Enable."]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Interrupt Enable."]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Interrupt Enable."]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Interrupt Enable."]
    #[inline(always)]
    pub fn ch0(&mut self) -> Ch0W<IntenSpec> {
        Ch0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Interrupt Enable."]
    #[inline(always)]
    pub fn ch1(&mut self) -> Ch1W<IntenSpec> {
        Ch1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Interrupt Enable."]
    #[inline(always)]
    pub fn ch2(&mut self) -> Ch2W<IntenSpec> {
        Ch2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Interrupt Enable."]
    #[inline(always)]
    pub fn ch3(&mut self) -> Ch3W<IntenSpec> {
        Ch3W::new(self, 3)
    }
}
#[doc = "DMA Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
