#[doc = "Register `CTRL0` reader"]
pub type R = crate::R<Ctrl0Spec>;
#[doc = "Register `CTRL0` writer"]
pub type W = crate::W<Ctrl0Spec>;
#[doc = "SPI Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: SPI is disabled."]
    Dis = 0,
    #[doc = "1: SPI is enabled."]
    En = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - SPI Enable."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Dis,
            true => En::En,
        }
    }
    #[doc = "SPI is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En::Dis
    }
    #[doc = "SPI is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En::En
    }
}
#[doc = "Field `EN` writer - SPI Enable."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En::Dis)
    }
    #[doc = "SPI is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En::En)
    }
}
#[doc = "Master Mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MstMode {
    #[doc = "0: SPI is Slave mode."]
    Dis = 0,
    #[doc = "1: SPI is Master mode."]
    En = 1,
}
impl From<MstMode> for bool {
    #[inline(always)]
    fn from(variant: MstMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MST_MODE` reader - Master Mode Enable."]
pub type MstModeR = crate::BitReader<MstMode>;
impl MstModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MstMode {
        match self.bits {
            false => MstMode::Dis,
            true => MstMode::En,
        }
    }
    #[doc = "SPI is Slave mode."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MstMode::Dis
    }
    #[doc = "SPI is Master mode."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MstMode::En
    }
}
#[doc = "Field `MST_MODE` writer - Master Mode Enable."]
pub type MstModeW<'a, REG> = crate::BitWriter<'a, REG, MstMode>;
impl<'a, REG> MstModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI is Slave mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(MstMode::Dis)
    }
    #[doc = "SPI is Master mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(MstMode::En)
    }
}
#[doc = "Slave Select 0, IO direction, to support Multi-Master mode,Slave Select 0 can be input in Master mode. This bit has no effect in slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsIo {
    #[doc = "0: Slave select 0 is output."]
    Output = 0,
    #[doc = "1: Slave Select 0 is input, only valid if MMEN=1."]
    Input = 1,
}
impl From<SsIo> for bool {
    #[inline(always)]
    fn from(variant: SsIo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SS_IO` reader - Slave Select 0, IO direction, to support Multi-Master mode,Slave Select 0 can be input in Master mode. This bit has no effect in slave mode."]
pub type SsIoR = crate::BitReader<SsIo>;
impl SsIoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsIo {
        match self.bits {
            false => SsIo::Output,
            true => SsIo::Input,
        }
    }
    #[doc = "Slave select 0 is output."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == SsIo::Output
    }
    #[doc = "Slave Select 0 is input, only valid if MMEN=1."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == SsIo::Input
    }
}
#[doc = "Field `SS_IO` writer - Slave Select 0, IO direction, to support Multi-Master mode,Slave Select 0 can be input in Master mode. This bit has no effect in slave mode."]
pub type SsIoW<'a, REG> = crate::BitWriter<'a, REG, SsIo>;
impl<'a, REG> SsIoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave select 0 is output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(SsIo::Output)
    }
    #[doc = "Slave Select 0 is input, only valid if MMEN=1."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(SsIo::Input)
    }
}
#[doc = "Start Transmit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "1: Master Initiates a transaction, this bit is self clearing when transactions are done. If a transaction cimpletes, and the TX FIFO is empty, the Master halts, if a transaction completes, and the TX FIFO is not empty, the Master initiates another transaction."]
    Start = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start Transmit."]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Start> {
        match self.bits {
            true => Some(Start::Start),
            _ => None,
        }
    }
    #[doc = "Master Initiates a transaction, this bit is self clearing when transactions are done. If a transaction cimpletes, and the TX FIFO is empty, the Master halts, if a transaction completes, and the TX FIFO is not empty, the Master initiates another transaction."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Start::Start
    }
}
#[doc = "Field `START` writer - Start Transmit."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master Initiates a transaction, this bit is self clearing when transactions are done. If a transaction cimpletes, and the TX FIFO is empty, the Master halts, if a transaction completes, and the TX FIFO is not empty, the Master initiates another transaction."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Start)
    }
}
#[doc = "Start Select Control. Used in Master mode to control the behavior of the Slave Select signal at the end of a transaction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsCtrl {
    #[doc = "0: SPI De-asserts Slave Select at the end of a transaction."]
    Deassert = 0,
    #[doc = "1: SPI leaves Slave Select asserted at the end of a transaction."]
    Assert = 1,
}
impl From<SsCtrl> for bool {
    #[inline(always)]
    fn from(variant: SsCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SS_CTRL` reader - Start Select Control. Used in Master mode to control the behavior of the Slave Select signal at the end of a transaction."]
pub type SsCtrlR = crate::BitReader<SsCtrl>;
impl SsCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsCtrl {
        match self.bits {
            false => SsCtrl::Deassert,
            true => SsCtrl::Assert,
        }
    }
    #[doc = "SPI De-asserts Slave Select at the end of a transaction."]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == SsCtrl::Deassert
    }
    #[doc = "SPI leaves Slave Select asserted at the end of a transaction."]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == SsCtrl::Assert
    }
}
#[doc = "Field `SS_CTRL` writer - Start Select Control. Used in Master mode to control the behavior of the Slave Select signal at the end of a transaction."]
pub type SsCtrlW<'a, REG> = crate::BitWriter<'a, REG, SsCtrl>;
impl<'a, REG> SsCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI De-asserts Slave Select at the end of a transaction."]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(SsCtrl::Deassert)
    }
    #[doc = "SPI leaves Slave Select asserted at the end of a transaction."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(SsCtrl::Assert)
    }
}
#[doc = "Slave Select, when in Master mode selects which Slave devices are selected. More than one Slave device can be selected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SsActive {
    #[doc = "1: SS0 is selected."]
    Ss0 = 1,
    #[doc = "2: SS1 is selected."]
    Ss1 = 2,
    #[doc = "4: SS2 is selected."]
    Ss2 = 4,
    #[doc = "8: SS3 is selected."]
    Ss3 = 8,
}
impl From<SsActive> for u8 {
    #[inline(always)]
    fn from(variant: SsActive) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SsActive {
    type Ux = u8;
}
impl crate::IsEnum for SsActive {}
#[doc = "Field `SS_ACTIVE` reader - Slave Select, when in Master mode selects which Slave devices are selected. More than one Slave device can be selected."]
pub type SsActiveR = crate::FieldReader<SsActive>;
impl SsActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SsActive> {
        match self.bits {
            1 => Some(SsActive::Ss0),
            2 => Some(SsActive::Ss1),
            4 => Some(SsActive::Ss2),
            8 => Some(SsActive::Ss3),
            _ => None,
        }
    }
    #[doc = "SS0 is selected."]
    #[inline(always)]
    pub fn is_ss0(&self) -> bool {
        *self == SsActive::Ss0
    }
    #[doc = "SS1 is selected."]
    #[inline(always)]
    pub fn is_ss1(&self) -> bool {
        *self == SsActive::Ss1
    }
    #[doc = "SS2 is selected."]
    #[inline(always)]
    pub fn is_ss2(&self) -> bool {
        *self == SsActive::Ss2
    }
    #[doc = "SS3 is selected."]
    #[inline(always)]
    pub fn is_ss3(&self) -> bool {
        *self == SsActive::Ss3
    }
}
#[doc = "Field `SS_ACTIVE` writer - Slave Select, when in Master mode selects which Slave devices are selected. More than one Slave device can be selected."]
pub type SsActiveW<'a, REG> = crate::FieldWriter<'a, REG, 4, SsActive>;
impl<'a, REG> SsActiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SS0 is selected."]
    #[inline(always)]
    pub fn ss0(self) -> &'a mut crate::W<REG> {
        self.variant(SsActive::Ss0)
    }
    #[doc = "SS1 is selected."]
    #[inline(always)]
    pub fn ss1(self) -> &'a mut crate::W<REG> {
        self.variant(SsActive::Ss1)
    }
    #[doc = "SS2 is selected."]
    #[inline(always)]
    pub fn ss2(self) -> &'a mut crate::W<REG> {
        self.variant(SsActive::Ss2)
    }
    #[doc = "SS3 is selected."]
    #[inline(always)]
    pub fn ss3(self) -> &'a mut crate::W<REG> {
        self.variant(SsActive::Ss3)
    }
}
impl R {
    #[doc = "Bit 0 - SPI Enable."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Mode Enable."]
    #[inline(always)]
    pub fn mst_mode(&self) -> MstModeR {
        MstModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Select 0, IO direction, to support Multi-Master mode,Slave Select 0 can be input in Master mode. This bit has no effect in slave mode."]
    #[inline(always)]
    pub fn ss_io(&self) -> SsIoR {
        SsIoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Start Transmit."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Start Select Control. Used in Master mode to control the behavior of the Slave Select signal at the end of a transaction."]
    #[inline(always)]
    pub fn ss_ctrl(&self) -> SsCtrlR {
        SsCtrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Slave Select, when in Master mode selects which Slave devices are selected. More than one Slave device can be selected."]
    #[inline(always)]
    pub fn ss_active(&self) -> SsActiveR {
        SsActiveR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Enable."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<Ctrl0Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Master Mode Enable."]
    #[inline(always)]
    pub fn mst_mode(&mut self) -> MstModeW<Ctrl0Spec> {
        MstModeW::new(self, 1)
    }
    #[doc = "Bit 4 - Slave Select 0, IO direction, to support Multi-Master mode,Slave Select 0 can be input in Master mode. This bit has no effect in slave mode."]
    #[inline(always)]
    pub fn ss_io(&mut self) -> SsIoW<Ctrl0Spec> {
        SsIoW::new(self, 4)
    }
    #[doc = "Bit 5 - Start Transmit."]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<Ctrl0Spec> {
        StartW::new(self, 5)
    }
    #[doc = "Bit 8 - Start Select Control. Used in Master mode to control the behavior of the Slave Select signal at the end of a transaction."]
    #[inline(always)]
    pub fn ss_ctrl(&mut self) -> SsCtrlW<Ctrl0Spec> {
        SsCtrlW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Slave Select, when in Master mode selects which Slave devices are selected. More than one Slave device can be selected."]
    #[inline(always)]
    pub fn ss_active(&mut self) -> SsActiveW<Ctrl0Spec> {
        SsActiveW::new(self, 16)
    }
}
#[doc = "Register for controlling SPI peripheral.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl0Spec;
impl crate::RegisterSpec for Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl0::R`](R) reader structure"]
impl crate::Readable for Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl0::W`](W) writer structure"]
impl crate::Writable for Ctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for Ctrl0Spec {
    const RESET_VALUE: u32 = 0;
}
