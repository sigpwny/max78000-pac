#[doc = "Register `INTEN_SET` reader"]
pub type R = crate::R<IntenSetSpec>;
#[doc = "Register `INTEN_SET` writer"]
pub type W = crate::W<IntenSetSpec>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioIntenSet {
    #[doc = "0: No effect."]
    No = 0,
    #[doc = "1: Set GPIO_INT_EN bit in this position to '1'"]
    Set = 1,
}
impl From<GpioIntenSet> for u32 {
    #[inline(always)]
    fn from(variant: GpioIntenSet) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioIntenSet {
    type Ux = u32;
}
impl crate::IsEnum for GpioIntenSet {}
#[doc = "Field `GPIO_INTEN_SET` reader - Mask of all of the pins on the port."]
pub type GpioIntenSetR = crate::FieldReader<GpioIntenSet>;
impl GpioIntenSetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioIntenSet> {
        match self.bits {
            0 => Some(GpioIntenSet::No),
            1 => Some(GpioIntenSet::Set),
            _ => None,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == GpioIntenSet::No
    }
    #[doc = "Set GPIO_INT_EN bit in this position to '1'"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == GpioIntenSet::Set
    }
}
#[doc = "Field `GPIO_INTEN_SET` writer - Mask of all of the pins on the port."]
pub type GpioIntenSetW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioIntenSet>;
impl<'a, REG> GpioIntenSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIntenSet::No)
    }
    #[doc = "Set GPIO_INT_EN bit in this position to '1'"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIntenSet::Set)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_inten_set(&self) -> GpioIntenSetR {
        GpioIntenSetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_inten_set(&mut self) -> GpioIntenSetW<IntenSetSpec> {
        GpioIntenSetW::new(self, 0)
    }
}
#[doc = "GPIO Interrupt Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_INT_EN to 1, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSetSpec;
impl crate::RegisterSpec for IntenSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten_set::R`](R) reader structure"]
impl crate::Readable for IntenSetSpec {}
#[doc = "`write(|w| ..)` method takes [`inten_set::W`](W) writer structure"]
impl crate::Writable for IntenSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN_SET to value 0"]
impl crate::Resettable for IntenSetSpec {
    const RESET_VALUE: u32 = 0;
}
