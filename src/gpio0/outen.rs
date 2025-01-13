#[doc = "Register `OUTEN` reader"]
pub type R = crate::R<OutenSpec>;
#[doc = "Register `OUTEN` writer"]
pub type W = crate::W<OutenSpec>;
#[doc = "Mask of all of the pins on the port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum En {
    #[doc = "0: GPIO Output Disable"]
    Dis = 0,
    #[doc = "1: GPIO Output Enable"]
    En = 1,
}
impl From<En> for u32 {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for En {
    type Ux = u32;
}
impl crate::IsEnum for En {}
#[doc = "Field `EN` reader - Mask of all of the pins on the port."]
pub type EnR = crate::FieldReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<En> {
        match self.bits {
            0 => Some(En::Dis),
            1 => Some(En::En),
            _ => None,
        }
    }
    #[doc = "GPIO Output Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En::Dis
    }
    #[doc = "GPIO Output Enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En::En
    }
}
#[doc = "Field `EN` writer - Mask of all of the pins on the port."]
pub type EnW<'a, REG> = crate::FieldWriter<'a, REG, 32, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "GPIO Output Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En::Dis)
    }
    #[doc = "GPIO Output Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En::En)
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask of all of the pins on the port."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<OutenSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "GPIO Output Enable Register. Each bit controls the GPIO_OUT_EN setting for one GPIO pin in the associated port.\n\nYou can [`read`](crate::Reg::read) this register and get [`outen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutenSpec;
impl crate::RegisterSpec for OutenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outen::R`](R) reader structure"]
impl crate::Readable for OutenSpec {}
#[doc = "`write(|w| ..)` method takes [`outen::W`](W) writer structure"]
impl crate::Writable for OutenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTEN to value 0"]
impl crate::Resettable for OutenSpec {
    const RESET_VALUE: u32 = 0;
}
