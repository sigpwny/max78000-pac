#[doc = "Register `status` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `status` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `status0` reader - "]
pub type Status0R = crate::BitReader;
#[doc = "Field `status0` writer - "]
pub type Status0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `status1` reader - "]
pub type Status1R = crate::BitReader;
#[doc = "Field `status1` writer - "]
pub type Status1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `status2` reader - "]
pub type Status2R = crate::BitReader;
#[doc = "Field `status2` writer - "]
pub type Status2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `status3` reader - "]
pub type Status3R = crate::BitReader;
#[doc = "Field `status3` writer - "]
pub type Status3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `status4` reader - "]
pub type Status4R = crate::BitReader;
#[doc = "Field `status4` writer - "]
pub type Status4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `status5` reader - "]
pub type Status5R = crate::BitReader;
#[doc = "Field `status5` writer - "]
pub type Status5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `status6` reader - "]
pub type Status6R = crate::BitReader;
#[doc = "Field `status6` writer - "]
pub type Status6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `status7` reader - "]
pub type Status7R = crate::BitReader;
#[doc = "Field `status7` writer - "]
pub type Status7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn status0(&self) -> Status0R {
        Status0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn status1(&self) -> Status1R {
        Status1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn status2(&self) -> Status2R {
        Status2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn status3(&self) -> Status3R {
        Status3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn status4(&self) -> Status4R {
        Status4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn status5(&self) -> Status5R {
        Status5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn status6(&self) -> Status6R {
        Status6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn status7(&self) -> Status7R {
        Status7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn status0(&mut self) -> Status0W<StatusSpec> {
        Status0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn status1(&mut self) -> Status1W<StatusSpec> {
        Status1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn status2(&mut self) -> Status2W<StatusSpec> {
        Status2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn status3(&mut self) -> Status3W<StatusSpec> {
        Status3W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn status4(&mut self) -> Status4W<StatusSpec> {
        Status4W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn status5(&mut self) -> Status5W<StatusSpec> {
        Status5W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn status6(&mut self) -> Status6W<StatusSpec> {
        Status6W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn status7(&mut self) -> Status7W<StatusSpec> {
        Status7W::new(self, 7)
    }
}
#[doc = "Semaphore status bits. 0 indicates the semaphore is free, 1 indicates taken.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
