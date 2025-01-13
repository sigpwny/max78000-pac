#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `VDDA_LIMITLO` reader - VDDA Low Trim Limit."]
pub type VddaLimitloR = crate::FieldReader;
#[doc = "Field `VDDA_LIMITLO` writer - VDDA Low Trim Limit."]
pub type VddaLimitloW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `VDDA_LIMITHI` reader - VDDA High Trim Limit."]
pub type VddaLimithiR = crate::FieldReader;
#[doc = "Field `VDDA_LIMITHI` writer - VDDA High Trim Limit."]
pub type VddaLimithiW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `IPO_LIMITHI` reader - IPO High Trim Limit."]
pub type IpoLimithiR = crate::FieldReader<u16>;
#[doc = "Field `IPO_LIMITHI` writer - IPO High Trim Limit."]
pub type IpoLimithiW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "INRO Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InroSel {
    #[doc = "0: `0`"]
    _8khz = 0,
    #[doc = "1: `1`"]
    _16khz = 1,
    #[doc = "2: `10`"]
    _30khz = 2,
}
impl From<InroSel> for u8 {
    #[inline(always)]
    fn from(variant: InroSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for InroSel {
    type Ux = u8;
}
impl crate::IsEnum for InroSel {}
#[doc = "Field `INRO_SEL` reader - INRO Clock Select."]
pub type InroSelR = crate::FieldReader<InroSel>;
impl InroSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<InroSel> {
        match self.bits {
            0 => Some(InroSel::_8khz),
            1 => Some(InroSel::_16khz),
            2 => Some(InroSel::_30khz),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_8khz(&self) -> bool {
        *self == InroSel::_8khz
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_16khz(&self) -> bool {
        *self == InroSel::_16khz
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_30khz(&self) -> bool {
        *self == InroSel::_30khz
    }
}
#[doc = "Field `INRO_SEL` writer - INRO Clock Select."]
pub type InroSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, InroSel>;
impl<'a, REG> InroSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _8khz(self) -> &'a mut crate::W<REG> {
        self.variant(InroSel::_8khz)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _16khz(self) -> &'a mut crate::W<REG> {
        self.variant(InroSel::_16khz)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn _30khz(self) -> &'a mut crate::W<REG> {
        self.variant(InroSel::_30khz)
    }
}
#[doc = "Field `INRO_TRIM` reader - INRO Clock Trim."]
pub type InroTrimR = crate::FieldReader;
#[doc = "Field `INRO_TRIM` writer - INRO Clock Trim."]
pub type InroTrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:6 - VDDA Low Trim Limit."]
    #[inline(always)]
    pub fn vdda_limitlo(&self) -> VddaLimitloR {
        VddaLimitloR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - VDDA High Trim Limit."]
    #[inline(always)]
    pub fn vdda_limithi(&self) -> VddaLimithiR {
        VddaLimithiR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:23 - IPO High Trim Limit."]
    #[inline(always)]
    pub fn ipo_limithi(&self) -> IpoLimithiR {
        IpoLimithiR::new(((self.bits >> 15) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:25 - INRO Clock Select."]
    #[inline(always)]
    pub fn inro_sel(&self) -> InroSelR {
        InroSelR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 29:31 - INRO Clock Trim."]
    #[inline(always)]
    pub fn inro_trim(&self) -> InroTrimR {
        InroTrimR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - VDDA Low Trim Limit."]
    #[inline(always)]
    pub fn vdda_limitlo(&mut self) -> VddaLimitloW<CtrlSpec> {
        VddaLimitloW::new(self, 0)
    }
    #[doc = "Bits 8:14 - VDDA High Trim Limit."]
    #[inline(always)]
    pub fn vdda_limithi(&mut self) -> VddaLimithiW<CtrlSpec> {
        VddaLimithiW::new(self, 8)
    }
    #[doc = "Bits 15:23 - IPO High Trim Limit."]
    #[inline(always)]
    pub fn ipo_limithi(&mut self) -> IpoLimithiW<CtrlSpec> {
        IpoLimithiW::new(self, 15)
    }
    #[doc = "Bits 24:25 - INRO Clock Select."]
    #[inline(always)]
    pub fn inro_sel(&mut self) -> InroSelW<CtrlSpec> {
        InroSelW::new(self, 24)
    }
    #[doc = "Bits 29:31 - INRO Clock Trim."]
    #[inline(always)]
    pub fn inro_trim(&mut self) -> InroTrimW<CtrlSpec> {
        InroTrimW::new(self, 29)
    }
}
#[doc = "Control Trim System Initialization Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
