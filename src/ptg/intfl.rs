#[doc = "Register `INTFL` reader"]
pub type R = crate::R<IntflSpec>;
#[doc = "Register `INTFL` writer"]
pub type W = crate::W<IntflSpec>;
#[doc = "Field `pt0` reader - Pulse Train 0 Stopped Interrupt Flag"]
pub type Pt0R = crate::BitReader;
#[doc = "Field `pt0` writer - Pulse Train 0 Stopped Interrupt Flag"]
pub type Pt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt1` reader - Pulse Train 1 Stopped Interrupt Flag"]
pub type Pt1R = crate::BitReader;
#[doc = "Field `pt1` writer - Pulse Train 1 Stopped Interrupt Flag"]
pub type Pt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt2` reader - Pulse Train 2 Stopped Interrupt Flag"]
pub type Pt2R = crate::BitReader;
#[doc = "Field `pt2` writer - Pulse Train 2 Stopped Interrupt Flag"]
pub type Pt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt3` reader - Pulse Train 3 Stopped Interrupt Flag"]
pub type Pt3R = crate::BitReader;
#[doc = "Field `pt3` writer - Pulse Train 3 Stopped Interrupt Flag"]
pub type Pt3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pulse Train 0 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt0(&self) -> Pt0R {
        Pt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pulse Train 1 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt1(&self) -> Pt1R {
        Pt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pulse Train 2 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt2(&self) -> Pt2R {
        Pt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pulse Train 3 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt3(&self) -> Pt3R {
        Pt3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Train 0 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt0(&mut self) -> Pt0W<IntflSpec> {
        Pt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pulse Train 1 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt1(&mut self) -> Pt1W<IntflSpec> {
        Pt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pulse Train 2 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt2(&mut self) -> Pt2W<IntflSpec> {
        Pt2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pulse Train 3 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt3(&mut self) -> Pt3W<IntflSpec> {
        Pt3W::new(self, 3)
    }
}
#[doc = "Pulse Train Interrupt Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
