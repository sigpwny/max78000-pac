#[doc = "Register `PS` reader"]
pub type R = crate::R<PsSpec>;
#[doc = "Register `PS` writer"]
pub type W = crate::W<PsSpec>;
#[doc = "Field `ALL` reader - Mask of all of the pins on the port."]
pub type AllR = crate::FieldReader<u32>;
#[doc = "Field `ALL` writer - Mask of all of the pins on the port."]
pub type AllW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn all(&self) -> AllR {
        AllR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn all(&mut self) -> AllW<PsSpec> {
        AllW::new(self, 0)
    }
}
#[doc = "GPIO Pull Select Mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`ps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsSpec;
impl crate::RegisterSpec for PsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ps::R`](R) reader structure"]
impl crate::Readable for PsSpec {}
#[doc = "`write(|w| ..)` method takes [`ps::W`](W) writer structure"]
impl crate::Writable for PsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PS to value 0"]
impl crate::Resettable for PsSpec {
    const RESET_VALUE: u32 = 0;
}
