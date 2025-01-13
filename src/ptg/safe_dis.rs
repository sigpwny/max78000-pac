#[doc = "Register `SAFE_DIS` writer"]
pub type W = crate::W<SafeDisSpec>;
#[doc = "Field `PT0` writer - "]
pub type Pt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT1` writer - "]
pub type Pt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT2` writer - "]
pub type Pt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT3` writer - "]
pub type Pt3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pt0(&mut self) -> Pt0W<SafeDisSpec> {
        Pt0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pt1(&mut self) -> Pt1W<SafeDisSpec> {
        Pt1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pt2(&mut self) -> Pt2W<SafeDisSpec> {
        Pt2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pt3(&mut self) -> Pt3W<SafeDisSpec> {
        Pt3W::new(self, 3)
    }
}
#[doc = "Pulse Train Global Safe Disable.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`safe_dis::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SafeDisSpec;
impl crate::RegisterSpec for SafeDisSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`safe_dis::W`](W) writer structure"]
impl crate::Writable for SafeDisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAFE_DIS to value 0"]
impl crate::Resettable for SafeDisSpec {
    const RESET_VALUE: u32 = 0;
}
