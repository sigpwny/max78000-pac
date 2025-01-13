#[doc = "Register `LPPWST` reader"]
pub type R = crate::R<LppwstSpec>;
#[doc = "Register `LPPWST` writer"]
pub type W = crate::W<LppwstSpec>;
#[doc = "Field `AINCOMP0` reader - Analog Input Comparator Wakeup Flag."]
pub type Aincomp0R = crate::BitReader;
#[doc = "Field `AINCOMP0` writer - Analog Input Comparator Wakeup Flag."]
pub type Aincomp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BACKUP` reader - Backup Mode Wakeup Flag."]
pub type BackupR = crate::BitReader;
#[doc = "Field `BACKUP` writer - Backup Mode Wakeup Flag."]
pub type BackupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - Reset Detected Wakeup Flag."]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - Reset Detected Wakeup Flag."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Analog Input Comparator Wakeup Flag."]
    #[inline(always)]
    pub fn aincomp0(&self) -> Aincomp0R {
        Aincomp0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup Mode Wakeup Flag."]
    #[inline(always)]
    pub fn backup(&self) -> BackupR {
        BackupR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset Detected Wakeup Flag."]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Analog Input Comparator Wakeup Flag."]
    #[inline(always)]
    pub fn aincomp0(&mut self) -> Aincomp0W<LppwstSpec> {
        Aincomp0W::new(self, 4)
    }
    #[doc = "Bit 16 - Backup Mode Wakeup Flag."]
    #[inline(always)]
    pub fn backup(&mut self) -> BackupW<LppwstSpec> {
        BackupW::new(self, 16)
    }
    #[doc = "Bit 17 - Reset Detected Wakeup Flag."]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<LppwstSpec> {
        ResetW::new(self, 17)
    }
}
#[doc = "Low Power Peripheral Wakeup Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lppwst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lppwst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LppwstSpec;
impl crate::RegisterSpec for LppwstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lppwst::R`](R) reader structure"]
impl crate::Readable for LppwstSpec {}
#[doc = "`write(|w| ..)` method takes [`lppwst::W`](W) writer structure"]
impl crate::Writable for LppwstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPPWST to value 0"]
impl crate::Resettable for LppwstSpec {
    const RESET_VALUE: u32 = 0;
}
