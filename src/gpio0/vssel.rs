#[doc = "Register `VSSEL` reader"]
pub type R = crate::R<VsselSpec>;
#[doc = "Register `VSSEL` writer"]
pub type W = crate::W<VsselSpec>;
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
    pub fn all(&mut self) -> AllW<VsselSpec> {
        AllW::new(self, 0)
    }
}
#[doc = "GPIO Voltage Select.\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VsselSpec;
impl crate::RegisterSpec for VsselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vssel::R`](R) reader structure"]
impl crate::Readable for VsselSpec {}
#[doc = "`write(|w| ..)` method takes [`vssel::W`](W) writer structure"]
impl crate::Writable for VsselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VSSEL to value 0"]
impl crate::Resettable for VsselSpec {
    const RESET_VALUE: u32 = 0;
}
