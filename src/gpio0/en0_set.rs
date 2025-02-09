#[doc = "Register `EN0_SET` reader"]
pub type R = crate::R<En0SetSpec>;
#[doc = "Register `EN0_SET` writer"]
pub type W = crate::W<En0SetSpec>;
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
    pub fn all(&mut self) -> AllW<En0SetSpec> {
        AllW::new(self, 0)
    }
}
#[doc = "GPIO Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN to 1, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`en0_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en0_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct En0SetSpec;
impl crate::RegisterSpec for En0SetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en0_set::R`](R) reader structure"]
impl crate::Readable for En0SetSpec {}
#[doc = "`write(|w| ..)` method takes [`en0_set::W`](W) writer structure"]
impl crate::Writable for En0SetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EN0_SET to value 0"]
impl crate::Resettable for En0SetSpec {
    const RESET_VALUE: u32 = 0;
}
