#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "I2C Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disable I2C."]
    Dis = 0,
    #[doc = "1: enable I2C."]
    En = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - I2C Enable."]
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
    #[doc = "Disable I2C."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En::Dis
    }
    #[doc = "enable I2C."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En::En
    }
}
#[doc = "Field `EN` writer - I2C Enable."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable I2C."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En::Dis)
    }
    #[doc = "enable I2C."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En::En)
    }
}
#[doc = "Master Mode Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MstMode {
    #[doc = "0: Slave Mode."]
    SlaveMode = 0,
    #[doc = "1: Master Mode."]
    MasterMode = 1,
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
            false => MstMode::SlaveMode,
            true => MstMode::MasterMode,
        }
    }
    #[doc = "Slave Mode."]
    #[inline(always)]
    pub fn is_slave_mode(&self) -> bool {
        *self == MstMode::SlaveMode
    }
    #[doc = "Master Mode."]
    #[inline(always)]
    pub fn is_master_mode(&self) -> bool {
        *self == MstMode::MasterMode
    }
}
#[doc = "Field `MST_MODE` writer - Master Mode Enable."]
pub type MstModeW<'a, REG> = crate::BitWriter<'a, REG, MstMode>;
impl<'a, REG> MstModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave Mode."]
    #[inline(always)]
    pub fn slave_mode(self) -> &'a mut crate::W<REG> {
        self.variant(MstMode::SlaveMode)
    }
    #[doc = "Master Mode."]
    #[inline(always)]
    pub fn master_mode(self) -> &'a mut crate::W<REG> {
        self.variant(MstMode::MasterMode)
    }
}
#[doc = "General Call Address Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GcAddrEn {
    #[doc = "0: Ignore Gneral Call Address."]
    Dis = 0,
    #[doc = "1: Acknowledge general call address."]
    En = 1,
}
impl From<GcAddrEn> for bool {
    #[inline(always)]
    fn from(variant: GcAddrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GC_ADDR_EN` reader - General Call Address Enable."]
pub type GcAddrEnR = crate::BitReader<GcAddrEn>;
impl GcAddrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GcAddrEn {
        match self.bits {
            false => GcAddrEn::Dis,
            true => GcAddrEn::En,
        }
    }
    #[doc = "Ignore Gneral Call Address."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == GcAddrEn::Dis
    }
    #[doc = "Acknowledge general call address."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == GcAddrEn::En
    }
}
#[doc = "Field `GC_ADDR_EN` writer - General Call Address Enable."]
pub type GcAddrEnW<'a, REG> = crate::BitWriter<'a, REG, GcAddrEn>;
impl<'a, REG> GcAddrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignore Gneral Call Address."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(GcAddrEn::Dis)
    }
    #[doc = "Acknowledge general call address."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(GcAddrEn::En)
    }
}
#[doc = "Interactive Receive Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrxmEn {
    #[doc = "0: Disable Interactive Receive Mode."]
    Dis = 0,
    #[doc = "1: Enable Interactive Receive Mode."]
    En = 1,
}
impl From<IrxmEn> for bool {
    #[inline(always)]
    fn from(variant: IrxmEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRXM_EN` reader - Interactive Receive Mode."]
pub type IrxmEnR = crate::BitReader<IrxmEn>;
impl IrxmEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrxmEn {
        match self.bits {
            false => IrxmEn::Dis,
            true => IrxmEn::En,
        }
    }
    #[doc = "Disable Interactive Receive Mode."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == IrxmEn::Dis
    }
    #[doc = "Enable Interactive Receive Mode."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == IrxmEn::En
    }
}
#[doc = "Field `IRXM_EN` writer - Interactive Receive Mode."]
pub type IrxmEnW<'a, REG> = crate::BitWriter<'a, REG, IrxmEn>;
impl<'a, REG> IrxmEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Interactive Receive Mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(IrxmEn::Dis)
    }
    #[doc = "Enable Interactive Receive Mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(IrxmEn::En)
    }
}
#[doc = "Data Acknowledge. This bit defines the acknowledge bit returned by the I2C receiver while IRXM = 1 HW forces ACK to 0 when IRXM = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrxmAck {
    #[doc = "0: return ACK (pulling SDA LOW)."]
    Ack = 0,
    #[doc = "1: return NACK (leaving SDA HIGH)."]
    Nack = 1,
}
impl From<IrxmAck> for bool {
    #[inline(always)]
    fn from(variant: IrxmAck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRXM_ACK` reader - Data Acknowledge. This bit defines the acknowledge bit returned by the I2C receiver while IRXM = 1 HW forces ACK to 0 when IRXM = 0."]
pub type IrxmAckR = crate::BitReader<IrxmAck>;
impl IrxmAckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrxmAck {
        match self.bits {
            false => IrxmAck::Ack,
            true => IrxmAck::Nack,
        }
    }
    #[doc = "return ACK (pulling SDA LOW)."]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == IrxmAck::Ack
    }
    #[doc = "return NACK (leaving SDA HIGH)."]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == IrxmAck::Nack
    }
}
#[doc = "Field `IRXM_ACK` writer - Data Acknowledge. This bit defines the acknowledge bit returned by the I2C receiver while IRXM = 1 HW forces ACK to 0 when IRXM = 0."]
pub type IrxmAckW<'a, REG> = crate::BitWriter<'a, REG, IrxmAck>;
impl<'a, REG> IrxmAckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "return ACK (pulling SDA LOW)."]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(IrxmAck::Ack)
    }
    #[doc = "return NACK (leaving SDA HIGH)."]
    #[inline(always)]
    pub fn nack(self) -> &'a mut crate::W<REG> {
        self.variant(IrxmAck::Nack)
    }
}
#[doc = "SCL Output. This bits control SCL output when SWOE =1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SclOut {
    #[doc = "0: Drive SCL low."]
    DriveSclLow = 0,
    #[doc = "1: Release SCL."]
    ReleaseScl = 1,
}
impl From<SclOut> for bool {
    #[inline(always)]
    fn from(variant: SclOut) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCL_OUT` reader - SCL Output. This bits control SCL output when SWOE =1."]
pub type SclOutR = crate::BitReader<SclOut>;
impl SclOutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SclOut {
        match self.bits {
            false => SclOut::DriveSclLow,
            true => SclOut::ReleaseScl,
        }
    }
    #[doc = "Drive SCL low."]
    #[inline(always)]
    pub fn is_drive_scl_low(&self) -> bool {
        *self == SclOut::DriveSclLow
    }
    #[doc = "Release SCL."]
    #[inline(always)]
    pub fn is_release_scl(&self) -> bool {
        *self == SclOut::ReleaseScl
    }
}
#[doc = "Field `SCL_OUT` writer - SCL Output. This bits control SCL output when SWOE =1."]
pub type SclOutW<'a, REG> = crate::BitWriter<'a, REG, SclOut>;
impl<'a, REG> SclOutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Drive SCL low."]
    #[inline(always)]
    pub fn drive_scl_low(self) -> &'a mut crate::W<REG> {
        self.variant(SclOut::DriveSclLow)
    }
    #[doc = "Release SCL."]
    #[inline(always)]
    pub fn release_scl(self) -> &'a mut crate::W<REG> {
        self.variant(SclOut::ReleaseScl)
    }
}
#[doc = "SDA Output. This bits control SDA output when SWOE = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdaOut {
    #[doc = "0: Drive SDA low."]
    DriveSdaLow = 0,
    #[doc = "1: Release SDA."]
    ReleaseSda = 1,
}
impl From<SdaOut> for bool {
    #[inline(always)]
    fn from(variant: SdaOut) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDA_OUT` reader - SDA Output. This bits control SDA output when SWOE = 1."]
pub type SdaOutR = crate::BitReader<SdaOut>;
impl SdaOutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SdaOut {
        match self.bits {
            false => SdaOut::DriveSdaLow,
            true => SdaOut::ReleaseSda,
        }
    }
    #[doc = "Drive SDA low."]
    #[inline(always)]
    pub fn is_drive_sda_low(&self) -> bool {
        *self == SdaOut::DriveSdaLow
    }
    #[doc = "Release SDA."]
    #[inline(always)]
    pub fn is_release_sda(&self) -> bool {
        *self == SdaOut::ReleaseSda
    }
}
#[doc = "Field `SDA_OUT` writer - SDA Output. This bits control SDA output when SWOE = 1."]
pub type SdaOutW<'a, REG> = crate::BitWriter<'a, REG, SdaOut>;
impl<'a, REG> SdaOutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Drive SDA low."]
    #[inline(always)]
    pub fn drive_sda_low(self) -> &'a mut crate::W<REG> {
        self.variant(SdaOut::DriveSdaLow)
    }
    #[doc = "Release SDA."]
    #[inline(always)]
    pub fn release_sda(self) -> &'a mut crate::W<REG> {
        self.variant(SdaOut::ReleaseSda)
    }
}
#[doc = "Field `SCL` reader - SCL status. This bit reflects the logic gate of SCL signal."]
pub type SclR = crate::BitReader;
#[doc = "Field `SDA` reader - SDA status. THis bit reflects the logic gate of SDA signal."]
pub type SdaR = crate::BitReader;
#[doc = "Software Output Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BbMode {
    #[doc = "0: I2C Outputs SCLO and SDAO disabled."]
    OutputsDisable = 0,
    #[doc = "1: I2C Outputs SCLO and SDAO enabled."]
    OutputsEnable = 1,
}
impl From<BbMode> for bool {
    #[inline(always)]
    fn from(variant: BbMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BB_MODE` reader - Software Output Enable."]
pub type BbModeR = crate::BitReader<BbMode>;
impl BbModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BbMode {
        match self.bits {
            false => BbMode::OutputsDisable,
            true => BbMode::OutputsEnable,
        }
    }
    #[doc = "I2C Outputs SCLO and SDAO disabled."]
    #[inline(always)]
    pub fn is_outputs_disable(&self) -> bool {
        *self == BbMode::OutputsDisable
    }
    #[doc = "I2C Outputs SCLO and SDAO enabled."]
    #[inline(always)]
    pub fn is_outputs_enable(&self) -> bool {
        *self == BbMode::OutputsEnable
    }
}
#[doc = "Field `BB_MODE` writer - Software Output Enable."]
pub type BbModeW<'a, REG> = crate::BitWriter<'a, REG, BbMode>;
impl<'a, REG> BbModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C Outputs SCLO and SDAO disabled."]
    #[inline(always)]
    pub fn outputs_disable(self) -> &'a mut crate::W<REG> {
        self.variant(BbMode::OutputsDisable)
    }
    #[doc = "I2C Outputs SCLO and SDAO enabled."]
    #[inline(always)]
    pub fn outputs_enable(self) -> &'a mut crate::W<REG> {
        self.variant(BbMode::OutputsEnable)
    }
}
#[doc = "Read. This bit reflects the R/W bit of an address match (AMI = 1) or general call match (GCI = 1). This bit is valid 3 cycles after the relevant interrupt bit is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Read {
    #[doc = "0: Write."]
    Write = 0,
    #[doc = "1: Read."]
    Read = 1,
}
impl From<Read> for bool {
    #[inline(always)]
    fn from(variant: Read) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ` reader - Read. This bit reflects the R/W bit of an address match (AMI = 1) or general call match (GCI = 1). This bit is valid 3 cycles after the relevant interrupt bit is set."]
pub type ReadR = crate::BitReader<Read>;
impl ReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Read {
        match self.bits {
            false => Read::Write,
            true => Read::Read,
        }
    }
    #[doc = "Write."]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == Read::Write
    }
    #[doc = "Read."]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Read::Read
    }
}
#[doc = "This bit will disable slave clock stretching when set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkstrDis {
    #[doc = "0: Slave clock stretching enabled."]
    En = 0,
    #[doc = "1: Slave clock stretching disabled."]
    Dis = 1,
}
impl From<ClkstrDis> for bool {
    #[inline(always)]
    fn from(variant: ClkstrDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSTR_DIS` reader - This bit will disable slave clock stretching when set."]
pub type ClkstrDisR = crate::BitReader<ClkstrDis>;
impl ClkstrDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkstrDis {
        match self.bits {
            false => ClkstrDis::En,
            true => ClkstrDis::Dis,
        }
    }
    #[doc = "Slave clock stretching enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ClkstrDis::En
    }
    #[doc = "Slave clock stretching disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ClkstrDis::Dis
    }
}
#[doc = "Field `CLKSTR_DIS` writer - This bit will disable slave clock stretching when set."]
pub type ClkstrDisW<'a, REG> = crate::BitWriter<'a, REG, ClkstrDis>;
impl<'a, REG> ClkstrDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave clock stretching enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(ClkstrDis::En)
    }
    #[doc = "Slave clock stretching disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(ClkstrDis::Dis)
    }
}
#[doc = "SCL Push-Pull Mode. This bit controls whether SCL is operated in a the I2C standard open-drain mode, or in a non-standard push-pull mode where the Hi-Z output isreplaced with Drive-1. The non-standard mode should only be used when operating as a master and communicating with slaves that are guaranteed to never drive SCL low.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OneMstMode {
    #[doc = "0: Standard open-drain operation: drive low for 0, Hi-Z for 1"]
    Dis = 0,
    #[doc = "1: Non-standard push-pull operation: drive low for 0, drive high for 1"]
    En = 1,
}
impl From<OneMstMode> for bool {
    #[inline(always)]
    fn from(variant: OneMstMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONE_MST_MODE` reader - SCL Push-Pull Mode. This bit controls whether SCL is operated in a the I2C standard open-drain mode, or in a non-standard push-pull mode where the Hi-Z output isreplaced with Drive-1. The non-standard mode should only be used when operating as a master and communicating with slaves that are guaranteed to never drive SCL low."]
pub type OneMstModeR = crate::BitReader<OneMstMode>;
impl OneMstModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OneMstMode {
        match self.bits {
            false => OneMstMode::Dis,
            true => OneMstMode::En,
        }
    }
    #[doc = "Standard open-drain operation: drive low for 0, Hi-Z for 1"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == OneMstMode::Dis
    }
    #[doc = "Non-standard push-pull operation: drive low for 0, drive high for 1"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == OneMstMode::En
    }
}
#[doc = "Field `ONE_MST_MODE` writer - SCL Push-Pull Mode. This bit controls whether SCL is operated in a the I2C standard open-drain mode, or in a non-standard push-pull mode where the Hi-Z output isreplaced with Drive-1. The non-standard mode should only be used when operating as a master and communicating with slaves that are guaranteed to never drive SCL low."]
pub type OneMstModeW<'a, REG> = crate::BitWriter<'a, REG, OneMstMode>;
impl<'a, REG> OneMstModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard open-drain operation: drive low for 0, Hi-Z for 1"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(OneMstMode::Dis)
    }
    #[doc = "Non-standard push-pull operation: drive low for 0, drive high for 1"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(OneMstMode::En)
    }
}
#[doc = "Field `HS_EN` reader - High speed mode enable"]
pub type HsEnR = crate::BitReader;
#[doc = "Field `HS_EN` writer - High speed mode enable"]
pub type HsEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C Enable."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Mode Enable."]
    #[inline(always)]
    pub fn mst_mode(&self) -> MstModeR {
        MstModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - General Call Address Enable."]
    #[inline(always)]
    pub fn gc_addr_en(&self) -> GcAddrEnR {
        GcAddrEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interactive Receive Mode."]
    #[inline(always)]
    pub fn irxm_en(&self) -> IrxmEnR {
        IrxmEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Acknowledge. This bit defines the acknowledge bit returned by the I2C receiver while IRXM = 1 HW forces ACK to 0 when IRXM = 0."]
    #[inline(always)]
    pub fn irxm_ack(&self) -> IrxmAckR {
        IrxmAckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL Output. This bits control SCL output when SWOE =1."]
    #[inline(always)]
    pub fn scl_out(&self) -> SclOutR {
        SclOutR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SDA Output. This bits control SDA output when SWOE = 1."]
    #[inline(always)]
    pub fn sda_out(&self) -> SdaOutR {
        SdaOutR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCL status. This bit reflects the logic gate of SCL signal."]
    #[inline(always)]
    pub fn scl(&self) -> SclR {
        SclR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SDA status. THis bit reflects the logic gate of SDA signal."]
    #[inline(always)]
    pub fn sda(&self) -> SdaR {
        SdaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software Output Enable."]
    #[inline(always)]
    pub fn bb_mode(&self) -> BbModeR {
        BbModeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Read. This bit reflects the R/W bit of an address match (AMI = 1) or general call match (GCI = 1). This bit is valid 3 cycles after the relevant interrupt bit is set."]
    #[inline(always)]
    pub fn read(&self) -> ReadR {
        ReadR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit will disable slave clock stretching when set."]
    #[inline(always)]
    pub fn clkstr_dis(&self) -> ClkstrDisR {
        ClkstrDisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCL Push-Pull Mode. This bit controls whether SCL is operated in a the I2C standard open-drain mode, or in a non-standard push-pull mode where the Hi-Z output isreplaced with Drive-1. The non-standard mode should only be used when operating as a master and communicating with slaves that are guaranteed to never drive SCL low."]
    #[inline(always)]
    pub fn one_mst_mode(&self) -> OneMstModeR {
        OneMstModeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - High speed mode enable"]
    #[inline(always)]
    pub fn hs_en(&self) -> HsEnR {
        HsEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Enable."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Master Mode Enable."]
    #[inline(always)]
    pub fn mst_mode(&mut self) -> MstModeW<CtrlSpec> {
        MstModeW::new(self, 1)
    }
    #[doc = "Bit 2 - General Call Address Enable."]
    #[inline(always)]
    pub fn gc_addr_en(&mut self) -> GcAddrEnW<CtrlSpec> {
        GcAddrEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Interactive Receive Mode."]
    #[inline(always)]
    pub fn irxm_en(&mut self) -> IrxmEnW<CtrlSpec> {
        IrxmEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Data Acknowledge. This bit defines the acknowledge bit returned by the I2C receiver while IRXM = 1 HW forces ACK to 0 when IRXM = 0."]
    #[inline(always)]
    pub fn irxm_ack(&mut self) -> IrxmAckW<CtrlSpec> {
        IrxmAckW::new(self, 4)
    }
    #[doc = "Bit 6 - SCL Output. This bits control SCL output when SWOE =1."]
    #[inline(always)]
    pub fn scl_out(&mut self) -> SclOutW<CtrlSpec> {
        SclOutW::new(self, 6)
    }
    #[doc = "Bit 7 - SDA Output. This bits control SDA output when SWOE = 1."]
    #[inline(always)]
    pub fn sda_out(&mut self) -> SdaOutW<CtrlSpec> {
        SdaOutW::new(self, 7)
    }
    #[doc = "Bit 10 - Software Output Enable."]
    #[inline(always)]
    pub fn bb_mode(&mut self) -> BbModeW<CtrlSpec> {
        BbModeW::new(self, 10)
    }
    #[doc = "Bit 12 - This bit will disable slave clock stretching when set."]
    #[inline(always)]
    pub fn clkstr_dis(&mut self) -> ClkstrDisW<CtrlSpec> {
        ClkstrDisW::new(self, 12)
    }
    #[doc = "Bit 13 - SCL Push-Pull Mode. This bit controls whether SCL is operated in a the I2C standard open-drain mode, or in a non-standard push-pull mode where the Hi-Z output isreplaced with Drive-1. The non-standard mode should only be used when operating as a master and communicating with slaves that are guaranteed to never drive SCL low."]
    #[inline(always)]
    pub fn one_mst_mode(&mut self) -> OneMstModeW<CtrlSpec> {
        OneMstModeW::new(self, 13)
    }
    #[doc = "Bit 15 - High speed mode enable"]
    #[inline(always)]
    pub fn hs_en(&mut self) -> HsEnW<CtrlSpec> {
        HsEnW::new(self, 15)
    }
}
#[doc = "Control Register0.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
