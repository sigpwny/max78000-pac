#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `pt0` reader - Pulse Train 0 Stopped Interrupt Enable/Disable"]
pub type Pt0R = crate::BitReader;
#[doc = "Field `pt0` writer - Pulse Train 0 Stopped Interrupt Enable/Disable"]
pub type Pt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt1` reader - Pulse Train 1 Stopped Interrupt Enable/Disable"]
pub type Pt1R = crate::BitReader;
#[doc = "Field `pt1` writer - Pulse Train 1 Stopped Interrupt Enable/Disable"]
pub type Pt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt2` reader - Pulse Train 2 Stopped Interrupt Enable/Disable"]
pub type Pt2R = crate::BitReader;
#[doc = "Field `pt2` writer - Pulse Train 2 Stopped Interrupt Enable/Disable"]
pub type Pt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt3` reader - Pulse Train 3 Stopped Interrupt Enable/Disable"]
pub type Pt3R = crate::BitReader;
#[doc = "Field `pt3` writer - Pulse Train 3 Stopped Interrupt Enable/Disable"]
pub type Pt3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pulse Train 0 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt0(&self) -> Pt0R {
        Pt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pulse Train 1 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt1(&self) -> Pt1R {
        Pt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pulse Train 2 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt2(&self) -> Pt2R {
        Pt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pulse Train 3 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt3(&self) -> Pt3R {
        Pt3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Train 0 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt0(&mut self) -> Pt0W<IntenSpec> {
        Pt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pulse Train 1 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt1(&mut self) -> Pt1W<IntenSpec> {
        Pt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pulse Train 2 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt2(&mut self) -> Pt2W<IntenSpec> {
        Pt2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pulse Train 3 Stopped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn pt3(&mut self) -> Pt3W<IntenSpec> {
        Pt3W::new(self, 3)
    }
}
#[doc = "Pulse Train Interrupt Enable/Disable\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
