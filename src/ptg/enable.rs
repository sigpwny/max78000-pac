#[doc = "Register `ENABLE` reader"]
pub type R = crate::R<EnableSpec>;
#[doc = "Register `ENABLE` writer"]
pub type W = crate::W<EnableSpec>;
#[doc = "Field `pt0` reader - Enable/Disable control for PT0"]
pub type Pt0R = crate::BitReader;
#[doc = "Field `pt0` writer - Enable/Disable control for PT0"]
pub type Pt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt1` reader - Enable/Disable control for PT1"]
pub type Pt1R = crate::BitReader;
#[doc = "Field `pt1` writer - Enable/Disable control for PT1"]
pub type Pt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt2` reader - Enable/Disable control for PT2"]
pub type Pt2R = crate::BitReader;
#[doc = "Field `pt2` writer - Enable/Disable control for PT2"]
pub type Pt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt3` reader - Enable/Disable control for PT3"]
pub type Pt3R = crate::BitReader;
#[doc = "Field `pt3` writer - Enable/Disable control for PT3"]
pub type Pt3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable/Disable control for PT0"]
    #[inline(always)]
    pub fn pt0(&self) -> Pt0R {
        Pt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable/Disable control for PT1"]
    #[inline(always)]
    pub fn pt1(&self) -> Pt1R {
        Pt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable/Disable control for PT2"]
    #[inline(always)]
    pub fn pt2(&self) -> Pt2R {
        Pt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable/Disable control for PT3"]
    #[inline(always)]
    pub fn pt3(&self) -> Pt3R {
        Pt3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable/Disable control for PT0"]
    #[inline(always)]
    pub fn pt0(&mut self) -> Pt0W<EnableSpec> {
        Pt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable/Disable control for PT1"]
    #[inline(always)]
    pub fn pt1(&mut self) -> Pt1W<EnableSpec> {
        Pt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable/Disable control for PT2"]
    #[inline(always)]
    pub fn pt2(&mut self) -> Pt2W<EnableSpec> {
        Pt2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable/Disable control for PT3"]
    #[inline(always)]
    pub fn pt3(&mut self) -> Pt3W<EnableSpec> {
        Pt3W::new(self, 3)
    }
}
#[doc = "Global Enable/Disable Controls for All Pulse Trains\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableSpec;
impl crate::RegisterSpec for EnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for EnableSpec {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for EnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLE to value 0"]
impl crate::Resettable for EnableSpec {
    const RESET_VALUE: u32 = 0;
}
