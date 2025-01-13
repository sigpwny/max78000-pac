#[doc = "Register `MEMCTRL` reader"]
pub type R = crate::R<MemctrlSpec>;
#[doc = "Register `MEMCTRL` writer"]
pub type W = crate::W<MemctrlSpec>;
#[doc = "Field `FWS` reader - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
pub type FwsR = crate::FieldReader;
#[doc = "Field `FWS` writer - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
pub type FwsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SYSRAM0ECC` reader - SYSRAM0 ECC Select."]
pub type Sysram0eccR = crate::BitReader;
#[doc = "Field `SYSRAM0ECC` writer - SYSRAM0 ECC Select."]
pub type Sysram0eccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
    #[inline(always)]
    pub fn fws(&self) -> FwsR {
        FwsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 16 - SYSRAM0 ECC Select."]
    #[inline(always)]
    pub fn sysram0ecc(&self) -> Sysram0eccR {
        Sysram0eccR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Flash Wait State. These bits define the number of wait-state cycles per Flash data read access. Minimum wait state is 2."]
    #[inline(always)]
    pub fn fws(&mut self) -> FwsW<MemctrlSpec> {
        FwsW::new(self, 0)
    }
    #[doc = "Bit 16 - SYSRAM0 ECC Select."]
    #[inline(always)]
    pub fn sysram0ecc(&mut self) -> Sysram0eccW<MemctrlSpec> {
        Sysram0eccW::new(self, 16)
    }
}
#[doc = "Memory Clock Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`memctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemctrlSpec;
impl crate::RegisterSpec for MemctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memctrl::R`](R) reader structure"]
impl crate::Readable for MemctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`memctrl::W`](W) writer structure"]
impl crate::Writable for MemctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMCTRL to value 0"]
impl crate::Resettable for MemctrlSpec {
    const RESET_VALUE: u32 = 0;
}
