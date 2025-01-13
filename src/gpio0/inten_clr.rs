#[doc = "Register `INTEN_CLR` reader"]
pub type R = crate::R<IntenClrSpec>;
#[doc = "Register `INTEN_CLR` writer"]
pub type W = crate::W<IntenClrSpec>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioIntenClr {
    #[doc = "0: No Effect."]
    No = 0,
    #[doc = "1: Clear GPIO_INT_EN bit in this position to '0'"]
    Clear = 1,
}
impl From<GpioIntenClr> for u32 {
    #[inline(always)]
    fn from(variant: GpioIntenClr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioIntenClr {
    type Ux = u32;
}
impl crate::IsEnum for GpioIntenClr {}
#[doc = "Field `GPIO_INTEN_CLR` reader - Mask of all of the pins on the port."]
pub type GpioIntenClrR = crate::FieldReader<GpioIntenClr>;
impl GpioIntenClrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioIntenClr> {
        match self.bits {
            0 => Some(GpioIntenClr::No),
            1 => Some(GpioIntenClr::Clear),
            _ => None,
        }
    }
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == GpioIntenClr::No
    }
    #[doc = "Clear GPIO_INT_EN bit in this position to '0'"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == GpioIntenClr::Clear
    }
}
#[doc = "Field `GPIO_INTEN_CLR` writer - Mask of all of the pins on the port."]
pub type GpioIntenClrW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioIntenClr>;
impl<'a, REG> GpioIntenClrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIntenClr::No)
    }
    #[doc = "Clear GPIO_INT_EN bit in this position to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIntenClr::Clear)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_inten_clr(&self) -> GpioIntenClrR {
        GpioIntenClrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_inten_clr(&mut self) -> GpioIntenClrW<IntenClrSpec> {
        GpioIntenClrW::new(self, 0)
    }
}
#[doc = "GPIO Interrupt Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_EN to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenClrSpec;
impl crate::RegisterSpec for IntenClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten_clr::R`](R) reader structure"]
impl crate::Readable for IntenClrSpec {}
#[doc = "`write(|w| ..)` method takes [`inten_clr::W`](W) writer structure"]
impl crate::Writable for IntenClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN_CLR to value 0"]
impl crate::Resettable for IntenClrSpec {
    const RESET_VALUE: u32 = 0;
}
