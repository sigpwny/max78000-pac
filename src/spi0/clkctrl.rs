#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<ClkctrlSpec>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<ClkctrlSpec>;
#[doc = "Low duty cycle control. In timer mode, reload\\[7:0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lo {
    #[doc = "0: Duty cycle control of serial clock generation is disabled."]
    Dis = 0,
}
impl From<Lo> for u8 {
    #[inline(always)]
    fn from(variant: Lo) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lo {
    type Ux = u8;
}
impl crate::IsEnum for Lo {}
#[doc = "Field `LO` reader - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
pub type LoR = crate::FieldReader<Lo>;
impl LoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lo> {
        match self.bits {
            0 => Some(Lo::Dis),
            _ => None,
        }
    }
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Lo::Dis
    }
}
#[doc = "Field `LO` writer - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
pub type LoW<'a, REG> = crate::FieldWriter<'a, REG, 8, Lo>;
impl<'a, REG> LoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Lo::Dis)
    }
}
#[doc = "High duty cycle control. In timer mode, reload\\[15:8\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hi {
    #[doc = "0: Duty cycle control of serial clock generation is disabled."]
    Dis = 0,
}
impl From<Hi> for u8 {
    #[inline(always)]
    fn from(variant: Hi) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hi {
    type Ux = u8;
}
impl crate::IsEnum for Hi {}
#[doc = "Field `HI` reader - High duty cycle control. In timer mode, reload\\[15:8\\]."]
pub type HiR = crate::FieldReader<Hi>;
impl HiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hi> {
        match self.bits {
            0 => Some(Hi::Dis),
            _ => None,
        }
    }
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Hi::Dis
    }
}
#[doc = "Field `HI` writer - High duty cycle control. In timer mode, reload\\[15:8\\]."]
pub type HiW<'a, REG> = crate::FieldWriter<'a, REG, 8, Hi>;
impl<'a, REG> HiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Duty cycle control of serial clock generation is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Hi::Dis)
    }
}
#[doc = "Field `CLKDIV` reader - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
    #[inline(always)]
    pub fn lo(&self) -> LoR {
        LoR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - High duty cycle control. In timer mode, reload\\[15:8\\]."]
    #[inline(always)]
    pub fn hi(&self) -> HiR {
        HiR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low duty cycle control. In timer mode, reload\\[7:0\\]."]
    #[inline(always)]
    pub fn lo(&mut self) -> LoW<ClkctrlSpec> {
        LoW::new(self, 0)
    }
    #[doc = "Bits 8:15 - High duty cycle control. In timer mode, reload\\[15:8\\]."]
    #[inline(always)]
    pub fn hi(&mut self) -> HiW<ClkctrlSpec> {
        HiW::new(self, 8)
    }
    #[doc = "Bits 16:19 - System Clock scale factor. Scales the AMBA clock by 2^SCALE before generating serial clock."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<ClkctrlSpec> {
        ClkdivW::new(self, 16)
    }
}
#[doc = "Register for controlling SPI clock rate.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctrlSpec;
impl crate::RegisterSpec for ClkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for ClkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for ClkctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for ClkctrlSpec {
    const RESET_VALUE: u32 = 0;
}
