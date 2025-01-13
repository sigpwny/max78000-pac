#[doc = "Register `DUALEDGE` reader"]
pub type R = crate::R<DualedgeSpec>;
#[doc = "Register `DUALEDGE` writer"]
pub type W = crate::W<DualedgeSpec>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioDualedge {
    #[doc = "0: No Effect."]
    No = 0,
    #[doc = "1: Dual Edge mode is enabled. If edge-triggered interrupts are enabled on this GPIO pin, then both rising and falling edges will trigger interrupts regardless of the GPIO_INT_POL setting."]
    En = 1,
}
impl From<GpioDualedge> for u32 {
    #[inline(always)]
    fn from(variant: GpioDualedge) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioDualedge {
    type Ux = u32;
}
impl crate::IsEnum for GpioDualedge {}
#[doc = "Field `GPIO_DUALEDGE` reader - Mask of all of the pins on the port."]
pub type GpioDualedgeR = crate::FieldReader<GpioDualedge>;
impl GpioDualedgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioDualedge> {
        match self.bits {
            0 => Some(GpioDualedge::No),
            1 => Some(GpioDualedge::En),
            _ => None,
        }
    }
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == GpioDualedge::No
    }
    #[doc = "Dual Edge mode is enabled. If edge-triggered interrupts are enabled on this GPIO pin, then both rising and falling edges will trigger interrupts regardless of the GPIO_INT_POL setting."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GpioDualedge::En
    }
}
#[doc = "Field `GPIO_DUALEDGE` writer - Mask of all of the pins on the port."]
pub type GpioDualedgeW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioDualedge>;
impl<'a, REG> GpioDualedgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "No Effect."]
    #[inline(always)]
    pub fn no(self) -> &'a mut crate::W<REG> {
        self.variant(GpioDualedge::No)
    }
    #[doc = "Dual Edge mode is enabled. If edge-triggered interrupts are enabled on this GPIO pin, then both rising and falling edges will trigger interrupts regardless of the GPIO_INT_POL setting."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(GpioDualedge::En)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_dualedge(&self) -> GpioDualedgeR {
        GpioDualedgeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_dualedge(&mut self) -> GpioDualedgeW<DualedgeSpec> {
        GpioDualedgeW::new(self, 0)
    }
}
#[doc = "GPIO Interrupt Dual Edge Mode Register. Each bit in this register selects dual edge mode for the associated GPIO pin in this port.\n\nYou can [`read`](crate::Reg::read) this register and get [`dualedge::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dualedge::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DualedgeSpec;
impl crate::RegisterSpec for DualedgeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dualedge::R`](R) reader structure"]
impl crate::Readable for DualedgeSpec {}
#[doc = "`write(|w| ..)` method takes [`dualedge::W`](W) writer structure"]
impl crate::Writable for DualedgeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DUALEDGE to value 0"]
impl crate::Resettable for DualedgeSpec {
    const RESET_VALUE: u32 = 0;
}
