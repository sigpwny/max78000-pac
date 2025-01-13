#[doc = "Register `SRSEL` reader"]
pub type R = crate::R<SrselSpec>;
#[doc = "Register `SRSEL` writer"]
pub type W = crate::W<SrselSpec>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioSrsel {
    #[doc = "0: Fast Slew Rate selected."]
    Fast = 0,
    #[doc = "1: Slow Slew Rate selected."]
    Slow = 1,
}
impl From<GpioSrsel> for u32 {
    #[inline(always)]
    fn from(variant: GpioSrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GpioSrsel {
    type Ux = u32;
}
impl crate::IsEnum for GpioSrsel {}
#[doc = "Field `GPIO_SRSEL` reader - Mask of all of the pins on the port."]
pub type GpioSrselR = crate::FieldReader<GpioSrsel>;
impl GpioSrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GpioSrsel> {
        match self.bits {
            0 => Some(GpioSrsel::Fast),
            1 => Some(GpioSrsel::Slow),
            _ => None,
        }
    }
    #[doc = "Fast Slew Rate selected."]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == GpioSrsel::Fast
    }
    #[doc = "Slow Slew Rate selected."]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == GpioSrsel::Slow
    }
}
#[doc = "Field `GPIO_SRSEL` writer - Mask of all of the pins on the port."]
pub type GpioSrselW<'a, REG> = crate::FieldWriter<'a, REG, 32, GpioSrsel>;
impl<'a, REG> GpioSrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Fast Slew Rate selected."]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(GpioSrsel::Fast)
    }
    #[doc = "Slow Slew Rate selected."]
    #[inline(always)]
    pub fn slow(self) -> &'a mut crate::W<REG> {
        self.variant(GpioSrsel::Slow)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_srsel(&self) -> GpioSrselR {
        GpioSrselR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn gpio_srsel(&mut self) -> GpioSrselW<SrselSpec> {
        GpioSrselW::new(self, 0)
    }
}
#[doc = "GPIO Slew Rate Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`srsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrselSpec;
impl crate::RegisterSpec for SrselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srsel::R`](R) reader structure"]
impl crate::Readable for SrselSpec {}
#[doc = "`write(|w| ..)` method takes [`srsel::W`](W) writer structure"]
impl crate::Writable for SrselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSEL to value 0"]
impl crate::Resettable for SrselSpec {
    const RESET_VALUE: u32 = 0;
}
