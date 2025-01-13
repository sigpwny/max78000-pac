#[doc = "Register `INTFL_CLR` reader"]
pub type R = crate::R<IntflClrSpec>;
#[doc = "Register `INTFL_CLR` writer"]
pub type W = crate::W<IntflClrSpec>;
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
    pub fn all(&mut self) -> AllW<IntflClrSpec> {
        AllW::new(self, 0)
    }
}
#[doc = "GPIO Status Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_STAT to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflClrSpec;
impl crate::RegisterSpec for IntflClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intfl_clr::R`](R) reader structure"]
impl crate::Readable for IntflClrSpec {}
#[doc = "`write(|w| ..)` method takes [`intfl_clr::W`](W) writer structure"]
impl crate::Writable for IntflClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFL_CLR to value 0"]
impl crate::Resettable for IntflClrSpec {
    const RESET_VALUE: u32 = 0;
}
