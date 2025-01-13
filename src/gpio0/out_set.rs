#[doc = "Register `OUT_SET` writer"]
pub type W = crate::W<OutSetSpec>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioOutSet {
    #[doc = "0: No Effect."]
    No = 0,
    #[doc = "1: Set GPIO_OUT bit in this position to '1'"]
    Set = 1,
}
impl From<GpioOutSet> for u32 {
    #[inline(always)]
    fn from(variant: GpioOutSet) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioOutSet {
    type Ux = u32;
}
impl crate::IsEnum for GpioOutSet {}
#[doc = "Field `GPIO_OUT_SET` writer - Mask of all of the pins on the port."]
pub type GpioOutSetW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioOutSet>;
impl<'a, REG> GpioOutSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(GpioOutSet::No)
    }
    #[doc = "Set GPIO_OUT bit in this position to '1'"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(GpioOutSet::Set)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_out_set(&mut self) -> GpioOutSetW<OutSetSpec> {
        GpioOutSetW::new(self, 0)
    }
}
#[doc = "GPIO Output Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT to 1, without affecting other bits in that register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutSetSpec;
impl crate::RegisterSpec for OutSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out_set::W`](W) writer structure"]
impl crate::Writable for OutSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_SET to value 0"]
impl crate::Resettable for OutSetSpec {
    const RESET_VALUE: u32 = 0;
}
