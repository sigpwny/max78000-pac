#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `start` reader - Start ADC Conversion"]
pub type StartR = crate::BitReader;
#[doc = "Field `start` writer - Start ADC Conversion"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pwr` reader - ADC Power Up"]
pub type PwrR = crate::BitReader;
#[doc = "Field `pwr` writer - ADC Power Up"]
pub type PwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `refbuf_pwr` reader - ADC Reference Buffer Power Up"]
pub type RefbufPwrR = crate::BitReader;
#[doc = "Field `refbuf_pwr` writer - ADC Reference Buffer Power Up"]
pub type RefbufPwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ref_sel` reader - ADC Reference Select"]
pub type RefSelR = crate::BitReader;
#[doc = "Field `ref_sel` writer - ADC Reference Select"]
pub type RefSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ref_scale` reader - ADC Reference Scale"]
pub type RefScaleR = crate::BitReader;
#[doc = "Field `ref_scale` writer - ADC Reference Scale"]
pub type RefScaleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scale` reader - ADC Scale"]
pub type ScaleR = crate::BitReader;
#[doc = "Field `scale` writer - ADC Scale"]
pub type ScaleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_en` reader - ADC Clock Enable"]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `clk_en` writer - ADC Clock Enable"]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ADC Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ChSel {
    #[doc = "0: `0`"]
    Ain0 = 0,
    #[doc = "1: `1`"]
    Ain1 = 1,
    #[doc = "2: `10`"]
    Ain2 = 2,
    #[doc = "3: `11`"]
    Ain3 = 3,
    #[doc = "4: `100`"]
    Ain4 = 4,
    #[doc = "5: `101`"]
    Ain5 = 5,
    #[doc = "6: `110`"]
    Ain6 = 6,
    #[doc = "7: `111`"]
    Ain7 = 7,
    #[doc = "8: `1000`"]
    VcoreA = 8,
    #[doc = "9: `1001`"]
    VcoreB = 9,
    #[doc = "10: `1010`"]
    Vrxout = 10,
    #[doc = "11: `1011`"]
    Vtxout = 11,
    #[doc = "12: `1100`"]
    VddA = 12,
    #[doc = "13: VddB/4"]
    VddB = 13,
    #[doc = "14: Vddio/4"]
    Vddio = 14,
    #[doc = "15: Vddioh/4"]
    Vddioh = 15,
    #[doc = "16: VregI/4"]
    VregI = 16,
}
impl From<ChSel> for u8 {
    #[inline(always)]
    fn from(variant: ChSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ChSel {
    type Ux = u8;
}
impl crate::IsEnum for ChSel {}
#[doc = "Field `ch_sel` reader - ADC Channel Select"]
pub type ChSelR = crate::FieldReader<ChSel>;
impl ChSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ChSel> {
        match self.bits {
            0 => Some(ChSel::Ain0),
            1 => Some(ChSel::Ain1),
            2 => Some(ChSel::Ain2),
            3 => Some(ChSel::Ain3),
            4 => Some(ChSel::Ain4),
            5 => Some(ChSel::Ain5),
            6 => Some(ChSel::Ain6),
            7 => Some(ChSel::Ain7),
            8 => Some(ChSel::VcoreA),
            9 => Some(ChSel::VcoreB),
            10 => Some(ChSel::Vrxout),
            11 => Some(ChSel::Vtxout),
            12 => Some(ChSel::VddA),
            13 => Some(ChSel::VddB),
            14 => Some(ChSel::Vddio),
            15 => Some(ChSel::Vddioh),
            16 => Some(ChSel::VregI),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_ain0(&self) -> bool {
        *self == ChSel::Ain0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == ChSel::Ain1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == ChSel::Ain2
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == ChSel::Ain3
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == ChSel::Ain4
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == ChSel::Ain5
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_ain6(&self) -> bool {
        *self == ChSel::Ain6
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_ain7(&self) -> bool {
        *self == ChSel::Ain7
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_vcore_a(&self) -> bool {
        *self == ChSel::VcoreA
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_vcore_b(&self) -> bool {
        *self == ChSel::VcoreB
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_vrxout(&self) -> bool {
        *self == ChSel::Vrxout
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_vtxout(&self) -> bool {
        *self == ChSel::Vtxout
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_vdd_a(&self) -> bool {
        *self == ChSel::VddA
    }
    #[doc = "VddB/4"]
    #[inline(always)]
    pub fn is_vdd_b(&self) -> bool {
        *self == ChSel::VddB
    }
    #[doc = "Vddio/4"]
    #[inline(always)]
    pub fn is_vddio(&self) -> bool {
        *self == ChSel::Vddio
    }
    #[doc = "Vddioh/4"]
    #[inline(always)]
    pub fn is_vddioh(&self) -> bool {
        *self == ChSel::Vddioh
    }
    #[doc = "VregI/4"]
    #[inline(always)]
    pub fn is_vreg_i(&self) -> bool {
        *self == ChSel::VregI
    }
}
#[doc = "Field `ch_sel` writer - ADC Channel Select"]
pub type ChSelW<'a, REG> = crate::FieldWriter<'a, REG, 5, ChSel>;
impl<'a, REG> ChSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ain0(self) -> &'a mut crate::W<REG> {
        self.variant(ChSel::Ain0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut crate::W<REG> {
        self.variant(ChSel::Ain1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut crate::W<REG> {
        self.variant(ChSel::Ain2)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut crate::W<REG> {
        self.variant(ChSel::Ain3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut crate::W<REG> {
        self.variant(ChSel::Ain4)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut crate::W<REG> {
        self.variant(ChSel::Ain5)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn ain6(self) -> &'a mut crate::W<REG> {
        self.variant(ChSel::Ain6)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn ain7(self) -> &'a mut crate::W<REG> {
        self.variant(ChSel::Ain7)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn vcore_a(self) -> &'a mut crate::W<REG> {
        self.variant(ChSel::VcoreA)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn vcore_b(self) -> &'a mut crate::W<REG> {
        self.variant(ChSel::VcoreB)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn vrxout(self) -> &'a mut crate::W<REG> {
        self.variant(ChSel::Vrxout)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn vtxout(self) -> &'a mut crate::W<REG> {
        self.variant(ChSel::Vtxout)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn vdd_a(self) -> &'a mut crate::W<REG> {
        self.variant(ChSel::VddA)
    }
    #[doc = "VddB/4"]
    #[inline(always)]
    pub fn vdd_b(self) -> &'a mut crate::W<REG> {
        self.variant(ChSel::VddB)
    }
    #[doc = "Vddio/4"]
    #[inline(always)]
    pub fn vddio(self) -> &'a mut crate::W<REG> {
        self.variant(ChSel::Vddio)
    }
    #[doc = "Vddioh/4"]
    #[inline(always)]
    pub fn vddioh(self) -> &'a mut crate::W<REG> {
        self.variant(ChSel::Vddioh)
    }
    #[doc = "VregI/4"]
    #[inline(always)]
    pub fn vreg_i(self) -> &'a mut crate::W<REG> {
        self.variant(ChSel::VregI)
    }
}
#[doc = "Scales the external inputs, all inputs are scaled the same\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcDivsel {
    #[doc = "0: `0`"]
    Div1 = 0,
    #[doc = "1: `1`"]
    Div2 = 1,
    #[doc = "2: `10`"]
    Div3 = 2,
    #[doc = "3: `11`"]
    Div4 = 3,
}
impl From<AdcDivsel> for u8 {
    #[inline(always)]
    fn from(variant: AdcDivsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AdcDivsel {
    type Ux = u8;
}
impl crate::IsEnum for AdcDivsel {}
#[doc = "Field `adc_divsel` reader - Scales the external inputs, all inputs are scaled the same"]
pub type AdcDivselR = crate::FieldReader<AdcDivsel>;
impl AdcDivselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcDivsel {
        match self.bits {
            0 => AdcDivsel::Div1,
            1 => AdcDivsel::Div2,
            2 => AdcDivsel::Div3,
            3 => AdcDivsel::Div4,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == AdcDivsel::Div1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == AdcDivsel::Div2
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == AdcDivsel::Div3
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == AdcDivsel::Div4
    }
}
#[doc = "Field `adc_divsel` writer - Scales the external inputs, all inputs are scaled the same"]
pub type AdcDivselW<'a, REG> = crate::FieldWriter<'a, REG, 2, AdcDivsel, crate::Safe>;
impl<'a, REG> AdcDivselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(AdcDivsel::Div1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(AdcDivsel::Div2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(AdcDivsel::Div3)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(AdcDivsel::Div4)
    }
}
#[doc = "Field `data_align` reader - ADC Data Alignment Select"]
pub type DataAlignR = crate::BitReader;
#[doc = "Field `data_align` writer - ADC Data Alignment Select"]
pub type DataAlignW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start ADC Conversion"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Power Up"]
    #[inline(always)]
    pub fn pwr(&self) -> PwrR {
        PwrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Reference Buffer Power Up"]
    #[inline(always)]
    pub fn refbuf_pwr(&self) -> RefbufPwrR {
        RefbufPwrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC Reference Select"]
    #[inline(always)]
    pub fn ref_sel(&self) -> RefSelR {
        RefSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC Reference Scale"]
    #[inline(always)]
    pub fn ref_scale(&self) -> RefScaleR {
        RefScaleR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC Scale"]
    #[inline(always)]
    pub fn scale(&self) -> ScaleR {
        ScaleR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC Clock Enable"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:16 - ADC Channel Select"]
    #[inline(always)]
    pub fn ch_sel(&self) -> ChSelR {
        ChSelR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:18 - Scales the external inputs, all inputs are scaled the same"]
    #[inline(always)]
    pub fn adc_divsel(&self) -> AdcDivselR {
        AdcDivselR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 20 - ADC Data Alignment Select"]
    #[inline(always)]
    pub fn data_align(&self) -> DataAlignR {
        DataAlignR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start ADC Conversion"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<CtrlSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC Power Up"]
    #[inline(always)]
    pub fn pwr(&mut self) -> PwrW<CtrlSpec> {
        PwrW::new(self, 1)
    }
    #[doc = "Bit 3 - ADC Reference Buffer Power Up"]
    #[inline(always)]
    pub fn refbuf_pwr(&mut self) -> RefbufPwrW<CtrlSpec> {
        RefbufPwrW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC Reference Select"]
    #[inline(always)]
    pub fn ref_sel(&mut self) -> RefSelW<CtrlSpec> {
        RefSelW::new(self, 4)
    }
    #[doc = "Bit 8 - ADC Reference Scale"]
    #[inline(always)]
    pub fn ref_scale(&mut self) -> RefScaleW<CtrlSpec> {
        RefScaleW::new(self, 8)
    }
    #[doc = "Bit 9 - ADC Scale"]
    #[inline(always)]
    pub fn scale(&mut self) -> ScaleW<CtrlSpec> {
        ScaleW::new(self, 9)
    }
    #[doc = "Bit 11 - ADC Clock Enable"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<CtrlSpec> {
        ClkEnW::new(self, 11)
    }
    #[doc = "Bits 12:16 - ADC Channel Select"]
    #[inline(always)]
    pub fn ch_sel(&mut self) -> ChSelW<CtrlSpec> {
        ChSelW::new(self, 12)
    }
    #[doc = "Bits 17:18 - Scales the external inputs, all inputs are scaled the same"]
    #[inline(always)]
    pub fn adc_divsel(&mut self) -> AdcDivselW<CtrlSpec> {
        AdcDivselW::new(self, 17)
    }
    #[doc = "Bit 20 - ADC Data Alignment Select"]
    #[inline(always)]
    pub fn data_align(&mut self) -> DataAlignW<CtrlSpec> {
        DataAlignW::new(self, 20)
    }
}
#[doc = "ADC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
