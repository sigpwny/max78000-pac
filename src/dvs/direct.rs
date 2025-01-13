#[doc = "Register `DIRECT` reader"]
pub type R = crate::R<DirectSpec>;
#[doc = "Register `DIRECT` writer"]
pub type W = crate::W<DirectSpec>;
#[doc = "Field `VOLTAGE` reader - Sets the target power supply value"]
pub type VoltageR = crate::FieldReader;
#[doc = "Field `VOLTAGE` writer - Sets the target power supply value"]
pub type VoltageW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Sets the target power supply value"]
    #[inline(always)]
    pub fn voltage(&self) -> VoltageR {
        VoltageR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Sets the target power supply value"]
    #[inline(always)]
    pub fn voltage(&mut self) -> VoltageW<DirectSpec> {
        VoltageW::new(self, 0)
    }
}
#[doc = "Direct control of target voltage\n\nYou can [`read`](crate::Reg::read) this register and get [`direct::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`direct::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirectSpec;
impl crate::RegisterSpec for DirectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`direct::R`](R) reader structure"]
impl crate::Readable for DirectSpec {}
#[doc = "`write(|w| ..)` method takes [`direct::W`](W) writer structure"]
impl crate::Writable for DirectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIRECT to value 0"]
impl crate::Resettable for DirectSpec {
    const RESET_VALUE: u32 = 0;
}
