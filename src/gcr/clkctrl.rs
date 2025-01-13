#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<ClkctrlSpec>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<ClkctrlSpec>;
#[doc = "Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysclkDiv {
    #[doc = "0: Divide by 1."]
    Div1 = 0,
    #[doc = "1: Divide by 2."]
    Div2 = 1,
    #[doc = "2: Divide by 4."]
    Div4 = 2,
    #[doc = "3: Divide by 8."]
    Div8 = 3,
    #[doc = "4: Divide by 16."]
    Div16 = 4,
    #[doc = "5: Divide by 32."]
    Div32 = 5,
    #[doc = "6: Divide by 64."]
    Div64 = 6,
    #[doc = "7: Divide by 128."]
    Div128 = 7,
}
impl From<SysclkDiv> for u8 {
    #[inline(always)]
    fn from(variant: SysclkDiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysclkDiv {
    type Ux = u8;
}
impl crate::IsEnum for SysclkDiv {}
#[doc = "Field `SYSCLK_DIV` reader - Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0."]
pub type SysclkDivR = crate::FieldReader<SysclkDiv>;
impl SysclkDivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysclkDiv {
        match self.bits {
            0 => SysclkDiv::Div1,
            1 => SysclkDiv::Div2,
            2 => SysclkDiv::Div4,
            3 => SysclkDiv::Div8,
            4 => SysclkDiv::Div16,
            5 => SysclkDiv::Div32,
            6 => SysclkDiv::Div64,
            7 => SysclkDiv::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == SysclkDiv::Div1
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == SysclkDiv::Div2
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == SysclkDiv::Div4
    }
    #[doc = "Divide by 8."]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == SysclkDiv::Div8
    }
    #[doc = "Divide by 16."]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == SysclkDiv::Div16
    }
    #[doc = "Divide by 32."]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == SysclkDiv::Div32
    }
    #[doc = "Divide by 64."]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == SysclkDiv::Div64
    }
    #[doc = "Divide by 128."]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == SysclkDiv::Div128
    }
}
#[doc = "Field `SYSCLK_DIV` writer - Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0."]
pub type SysclkDivW<'a, REG> = crate::FieldWriter<'a, REG, 3, SysclkDiv, crate::Safe>;
impl<'a, REG> SysclkDivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkDiv::Div1)
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkDiv::Div2)
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkDiv::Div4)
    }
    #[doc = "Divide by 8."]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkDiv::Div8)
    }
    #[doc = "Divide by 16."]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkDiv::Div16)
    }
    #[doc = "Divide by 32."]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkDiv::Div32)
    }
    #[doc = "Divide by 64."]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkDiv::Div64)
    }
    #[doc = "Divide by 128."]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkDiv::Div128)
    }
}
#[doc = "Clock Source Select. This 3 bit field selects the source for the system clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysclkSel {
    #[doc = "0: The internal 60 MHz oscillator is used for the system clock."]
    Iso = 0,
    #[doc = "3: 8 kHz LIRC is used for the system clock."]
    Inro = 3,
    #[doc = "4: The internal 100 MHz oscillator is used for the system clock."]
    Ipo = 4,
    #[doc = "5: The internal 7.3725 MHz oscillator is used for the system clock."]
    Ibro = 5,
    #[doc = "6: 32 kHz is used for the system clock."]
    Ertco = 6,
    #[doc = "7: External clock on GPIO0.30."]
    Extclk = 7,
}
impl From<SysclkSel> for u8 {
    #[inline(always)]
    fn from(variant: SysclkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysclkSel {
    type Ux = u8;
}
impl crate::IsEnum for SysclkSel {}
#[doc = "Field `SYSCLK_SEL` reader - Clock Source Select. This 3 bit field selects the source for the system clock."]
pub type SysclkSelR = crate::FieldReader<SysclkSel>;
impl SysclkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SysclkSel> {
        match self.bits {
            0 => Some(SysclkSel::Iso),
            3 => Some(SysclkSel::Inro),
            4 => Some(SysclkSel::Ipo),
            5 => Some(SysclkSel::Ibro),
            6 => Some(SysclkSel::Ertco),
            7 => Some(SysclkSel::Extclk),
            _ => None,
        }
    }
    #[doc = "The internal 60 MHz oscillator is used for the system clock."]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == SysclkSel::Iso
    }
    #[doc = "8 kHz LIRC is used for the system clock."]
    #[inline(always)]
    pub fn is_inro(&self) -> bool {
        *self == SysclkSel::Inro
    }
    #[doc = "The internal 100 MHz oscillator is used for the system clock."]
    #[inline(always)]
    pub fn is_ipo(&self) -> bool {
        *self == SysclkSel::Ipo
    }
    #[doc = "The internal 7.3725 MHz oscillator is used for the system clock."]
    #[inline(always)]
    pub fn is_ibro(&self) -> bool {
        *self == SysclkSel::Ibro
    }
    #[doc = "32 kHz is used for the system clock."]
    #[inline(always)]
    pub fn is_ertco(&self) -> bool {
        *self == SysclkSel::Ertco
    }
    #[doc = "External clock on GPIO0.30."]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == SysclkSel::Extclk
    }
}
#[doc = "Field `SYSCLK_SEL` writer - Clock Source Select. This 3 bit field selects the source for the system clock."]
pub type SysclkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, SysclkSel>;
impl<'a, REG> SysclkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The internal 60 MHz oscillator is used for the system clock."]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkSel::Iso)
    }
    #[doc = "8 kHz LIRC is used for the system clock."]
    #[inline(always)]
    pub fn inro(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkSel::Inro)
    }
    #[doc = "The internal 100 MHz oscillator is used for the system clock."]
    #[inline(always)]
    pub fn ipo(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkSel::Ipo)
    }
    #[doc = "The internal 7.3725 MHz oscillator is used for the system clock."]
    #[inline(always)]
    pub fn ibro(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkSel::Ibro)
    }
    #[doc = "32 kHz is used for the system clock."]
    #[inline(always)]
    pub fn ertco(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkSel::Ertco)
    }
    #[doc = "External clock on GPIO0.30."]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut crate::W<REG> {
        self.variant(SysclkSel::Extclk)
    }
}
#[doc = "Clock Ready. This read only bit reflects whether the currently selected system clock source is running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysclkRdy {
    #[doc = "0: Switchover to the new clock source (as selected by CLKSEL) has not yet occurred."]
    Busy = 0,
    #[doc = "1: System clock running from CLKSEL clock source."]
    Ready = 1,
}
impl From<SysclkRdy> for bool {
    #[inline(always)]
    fn from(variant: SysclkRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSCLK_RDY` reader - Clock Ready. This read only bit reflects whether the currently selected system clock source is running."]
pub type SysclkRdyR = crate::BitReader<SysclkRdy>;
impl SysclkRdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysclkRdy {
        match self.bits {
            false => SysclkRdy::Busy,
            true => SysclkRdy::Ready,
        }
    }
    #[doc = "Switchover to the new clock source (as selected by CLKSEL) has not yet occurred."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SysclkRdy::Busy
    }
    #[doc = "System clock running from CLKSEL clock source."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == SysclkRdy::Ready
    }
}
#[doc = "32 kHz Crystal Oscillator Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErtcoEn {
    #[doc = "0: Is Disabled."]
    Dis = 0,
    #[doc = "1: Is Enabled."]
    En = 1,
}
impl From<ErtcoEn> for bool {
    #[inline(always)]
    fn from(variant: ErtcoEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERTCO_EN` reader - 32 kHz Crystal Oscillator Enable."]
pub type ErtcoEnR = crate::BitReader<ErtcoEn>;
impl ErtcoEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ErtcoEn {
        match self.bits {
            false => ErtcoEn::Dis,
            true => ErtcoEn::En,
        }
    }
    #[doc = "Is Disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ErtcoEn::Dis
    }
    #[doc = "Is Enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ErtcoEn::En
    }
}
#[doc = "Field `ERTCO_EN` writer - 32 kHz Crystal Oscillator Enable."]
pub type ErtcoEnW<'a, REG> = crate::BitWriter<'a, REG, ErtcoEn>;
impl<'a, REG> ErtcoEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Is Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(ErtcoEn::Dis)
    }
    #[doc = "Is Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(ErtcoEn::En)
    }
}
#[doc = "60 MHz High Frequency Internal Reference Clock Enable."]
pub use ErtcoEn as IsoEn;
#[doc = "100 MHz High Frequency Internal Reference Clock Enable."]
pub use ErtcoEn as IpoEn;
#[doc = "7.3725 MHz High Frequency Internal Reference Clock Enable."]
pub use ErtcoEn as IbroEn;
#[doc = "Field `ISO_EN` reader - 60 MHz High Frequency Internal Reference Clock Enable."]
pub use ErtcoEnR as IsoEnR;
#[doc = "Field `IPO_EN` reader - 100 MHz High Frequency Internal Reference Clock Enable."]
pub use ErtcoEnR as IpoEnR;
#[doc = "Field `IBRO_EN` reader - 7.3725 MHz High Frequency Internal Reference Clock Enable."]
pub use ErtcoEnR as IbroEnR;
#[doc = "Field `ISO_EN` writer - 60 MHz High Frequency Internal Reference Clock Enable."]
pub use ErtcoEnW as IsoEnW;
#[doc = "Field `IPO_EN` writer - 100 MHz High Frequency Internal Reference Clock Enable."]
pub use ErtcoEnW as IpoEnW;
#[doc = "Field `IBRO_EN` writer - 7.3725 MHz High Frequency Internal Reference Clock Enable."]
pub use ErtcoEnW as IbroEnW;
#[doc = "7.3725 MHz High Frequency Internal Reference Clock Voltage Select. This register bit is used to select the power supply to the IBRO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IbroVs {
    #[doc = "0: VCore Supply"]
    Vcor = 0,
    #[doc = "1: Dedicated 1V regulated supply."]
    _1v = 1,
}
impl From<IbroVs> for bool {
    #[inline(always)]
    fn from(variant: IbroVs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBRO_VS` reader - 7.3725 MHz High Frequency Internal Reference Clock Voltage Select. This register bit is used to select the power supply to the IBRO."]
pub type IbroVsR = crate::BitReader<IbroVs>;
impl IbroVsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IbroVs {
        match self.bits {
            false => IbroVs::Vcor,
            true => IbroVs::_1v,
        }
    }
    #[doc = "VCore Supply"]
    #[inline(always)]
    pub fn is_vcor(&self) -> bool {
        *self == IbroVs::Vcor
    }
    #[doc = "Dedicated 1V regulated supply."]
    #[inline(always)]
    pub fn is_1v(&self) -> bool {
        *self == IbroVs::_1v
    }
}
#[doc = "Field `IBRO_VS` writer - 7.3725 MHz High Frequency Internal Reference Clock Voltage Select. This register bit is used to select the power supply to the IBRO."]
pub type IbroVsW<'a, REG> = crate::BitWriter<'a, REG, IbroVs>;
impl<'a, REG> IbroVsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VCore Supply"]
    #[inline(always)]
    pub fn vcor(self) -> &'a mut crate::W<REG> {
        self.variant(IbroVs::Vcor)
    }
    #[doc = "Dedicated 1V regulated supply."]
    #[inline(always)]
    pub fn _1v(self) -> &'a mut crate::W<REG> {
        self.variant(IbroVs::_1v)
    }
}
#[doc = "32 kHz Crystal Oscillator Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErtcoRdy {
    #[doc = "0: Is not Ready."]
    Not = 0,
    #[doc = "1: Is Ready."]
    Ready = 1,
}
impl From<ErtcoRdy> for bool {
    #[inline(always)]
    fn from(variant: ErtcoRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERTCO_RDY` reader - 32 kHz Crystal Oscillator Ready"]
pub type ErtcoRdyR = crate::BitReader<ErtcoRdy>;
impl ErtcoRdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ErtcoRdy {
        match self.bits {
            false => ErtcoRdy::Not,
            true => ErtcoRdy::Ready,
        }
    }
    #[doc = "Is not Ready."]
    #[inline(always)]
    pub fn is_not(&self) -> bool {
        *self == ErtcoRdy::Not
    }
    #[doc = "Is Ready."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ErtcoRdy::Ready
    }
}
#[doc = "60 MHz HIRC Ready."]
pub use ErtcoRdy as IsoRdy;
#[doc = "100 MHz HIRC Ready."]
pub use ErtcoRdy as IpoRdy;
#[doc = "7.3725 MHz HIRC Ready."]
pub use ErtcoRdy as IbroRdy;
#[doc = "8 kHz Low Frequency Reference Clock Ready."]
pub use ErtcoRdy as InroRdy;
#[doc = "Field `ISO_RDY` reader - 60 MHz HIRC Ready."]
pub use ErtcoRdyR as IsoRdyR;
#[doc = "Field `IPO_RDY` reader - 100 MHz HIRC Ready."]
pub use ErtcoRdyR as IpoRdyR;
#[doc = "Field `IBRO_RDY` reader - 7.3725 MHz HIRC Ready."]
pub use ErtcoRdyR as IbroRdyR;
#[doc = "Field `INRO_RDY` reader - 8 kHz Low Frequency Reference Clock Ready."]
pub use ErtcoRdyR as InroRdyR;
impl R {
    #[doc = "Bits 6:8 - Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0."]
    #[inline(always)]
    pub fn sysclk_div(&self) -> SysclkDivR {
        SysclkDivR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Clock Source Select. This 3 bit field selects the source for the system clock."]
    #[inline(always)]
    pub fn sysclk_sel(&self) -> SysclkSelR {
        SysclkSelR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 13 - Clock Ready. This read only bit reflects whether the currently selected system clock source is running."]
    #[inline(always)]
    pub fn sysclk_rdy(&self) -> SysclkRdyR {
        SysclkRdyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - 32 kHz Crystal Oscillator Enable."]
    #[inline(always)]
    pub fn ertco_en(&self) -> ErtcoEnR {
        ErtcoEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 60 MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn iso_en(&self) -> IsoEnR {
        IsoEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 100 MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn ipo_en(&self) -> IpoEnR {
        IpoEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 7.3725 MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn ibro_en(&self) -> IbroEnR {
        IbroEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 7.3725 MHz High Frequency Internal Reference Clock Voltage Select. This register bit is used to select the power supply to the IBRO."]
    #[inline(always)]
    pub fn ibro_vs(&self) -> IbroVsR {
        IbroVsR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - 32 kHz Crystal Oscillator Ready"]
    #[inline(always)]
    pub fn ertco_rdy(&self) -> ErtcoRdyR {
        ErtcoRdyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 60 MHz HIRC Ready."]
    #[inline(always)]
    pub fn iso_rdy(&self) -> IsoRdyR {
        IsoRdyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 100 MHz HIRC Ready."]
    #[inline(always)]
    pub fn ipo_rdy(&self) -> IpoRdyR {
        IpoRdyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 7.3725 MHz HIRC Ready."]
    #[inline(always)]
    pub fn ibro_rdy(&self) -> IbroRdyR {
        IbroRdyR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 8 kHz Low Frequency Reference Clock Ready."]
    #[inline(always)]
    pub fn inro_rdy(&self) -> InroRdyR {
        InroRdyR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 6:8 - Prescaler Select. This 3 bit field sets the system operating frequency by controlling the prescaler that divides the output of the PLL0."]
    #[inline(always)]
    pub fn sysclk_div(&mut self) -> SysclkDivW<ClkctrlSpec> {
        SysclkDivW::new(self, 6)
    }
    #[doc = "Bits 9:11 - Clock Source Select. This 3 bit field selects the source for the system clock."]
    #[inline(always)]
    pub fn sysclk_sel(&mut self) -> SysclkSelW<ClkctrlSpec> {
        SysclkSelW::new(self, 9)
    }
    #[doc = "Bit 17 - 32 kHz Crystal Oscillator Enable."]
    #[inline(always)]
    pub fn ertco_en(&mut self) -> ErtcoEnW<ClkctrlSpec> {
        ErtcoEnW::new(self, 17)
    }
    #[doc = "Bit 18 - 60 MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn iso_en(&mut self) -> IsoEnW<ClkctrlSpec> {
        IsoEnW::new(self, 18)
    }
    #[doc = "Bit 19 - 100 MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn ipo_en(&mut self) -> IpoEnW<ClkctrlSpec> {
        IpoEnW::new(self, 19)
    }
    #[doc = "Bit 20 - 7.3725 MHz High Frequency Internal Reference Clock Enable."]
    #[inline(always)]
    pub fn ibro_en(&mut self) -> IbroEnW<ClkctrlSpec> {
        IbroEnW::new(self, 20)
    }
    #[doc = "Bit 21 - 7.3725 MHz High Frequency Internal Reference Clock Voltage Select. This register bit is used to select the power supply to the IBRO."]
    #[inline(always)]
    pub fn ibro_vs(&mut self) -> IbroVsW<ClkctrlSpec> {
        IbroVsW::new(self, 21)
    }
}
#[doc = "Clock Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CLKCTRL to value 0x08"]
impl crate::Resettable for ClkctrlSpec {
    const RESET_VALUE: u32 = 0x08;
}
