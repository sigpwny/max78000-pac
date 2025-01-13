#[doc = "Register `OUTEN_CLR` reader"]
pub type R = crate::R<OutenClrSpec>;
#[doc = "Register `OUTEN_CLR` writer"]
pub type W = crate::W<OutenClrSpec>;
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
    pub fn all(&mut self) -> AllW<OutenClrSpec> {
        AllW::new(self, 0)
    }
}
#[doc = "GPIO Output Enable Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT_EN to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`outen_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outen_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutenClrSpec;
impl crate::RegisterSpec for OutenClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outen_clr::R`](R) reader structure"]
impl crate::Readable for OutenClrSpec {}
#[doc = "`write(|w| ..)` method takes [`outen_clr::W`](W) writer structure"]
impl crate::Writable for OutenClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTEN_CLR to value 0"]
impl crate::Resettable for OutenClrSpec {
    const RESET_VALUE: u32 = 0;
}
