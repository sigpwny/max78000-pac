#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Read Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ReadMode {
    #[doc = "0: Camera Interface Disabled."]
    Dis = 0,
    #[doc = "1: Single Image Capture."]
    SingleImg = 1,
    #[doc = "2: Continuous Image Capture."]
    Continuous = 2,
}
impl From<ReadMode> for u8 {
    #[inline(always)]
    fn from(variant: ReadMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ReadMode {
    type Ux = u8;
}
impl crate::IsEnum for ReadMode {}
#[doc = "Field `READ_MODE` reader - Read Mode."]
pub type ReadModeR = crate::FieldReader<ReadMode>;
impl ReadModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ReadMode> {
        match self.bits {
            0 => Some(ReadMode::Dis),
            1 => Some(ReadMode::SingleImg),
            2 => Some(ReadMode::Continuous),
            _ => None,
        }
    }
    #[doc = "Camera Interface Disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ReadMode::Dis
    }
    #[doc = "Single Image Capture."]
    #[inline(always)]
    pub fn is_single_img(&self) -> bool {
        *self == ReadMode::SingleImg
    }
    #[doc = "Continuous Image Capture."]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == ReadMode::Continuous
    }
}
#[doc = "Field `READ_MODE` writer - Read Mode."]
pub type ReadModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, ReadMode>;
impl<'a, REG> ReadModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Camera Interface Disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(ReadMode::Dis)
    }
    #[doc = "Single Image Capture."]
    #[inline(always)]
    pub fn single_img(self) -> &'a mut crate::W<REG> {
        self.variant(ReadMode::SingleImg)
    }
    #[doc = "Continuous Image Capture."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(ReadMode::Continuous)
    }
}
#[doc = "Data Width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DataWidth {
    #[doc = "0: 8 bit."]
    _8bit = 0,
    #[doc = "1: 10 bit."]
    _10bit = 1,
    #[doc = "2: 12 bit."]
    _12bit = 2,
}
impl From<DataWidth> for u8 {
    #[inline(always)]
    fn from(variant: DataWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DataWidth {
    type Ux = u8;
}
impl crate::IsEnum for DataWidth {}
#[doc = "Field `DATA_WIDTH` reader - Data Width."]
pub type DataWidthR = crate::FieldReader<DataWidth>;
impl DataWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DataWidth> {
        match self.bits {
            0 => Some(DataWidth::_8bit),
            1 => Some(DataWidth::_10bit),
            2 => Some(DataWidth::_12bit),
            _ => None,
        }
    }
    #[doc = "8 bit."]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == DataWidth::_8bit
    }
    #[doc = "10 bit."]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == DataWidth::_10bit
    }
    #[doc = "12 bit."]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == DataWidth::_12bit
    }
}
#[doc = "Field `DATA_WIDTH` writer - Data Width."]
pub type DataWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, DataWidth>;
impl<'a, REG> DataWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bit."]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(DataWidth::_8bit)
    }
    #[doc = "10 bit."]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(DataWidth::_10bit)
    }
    #[doc = "12 bit."]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(DataWidth::_12bit)
    }
}
#[doc = "DS Timing Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DsTimingEn {
    #[doc = "0: Timing from VSYNC and HSYNC."]
    Dis = 0,
    #[doc = "1: Timing embedded in data using SAV and EAV codes."]
    En = 1,
}
impl From<DsTimingEn> for bool {
    #[inline(always)]
    fn from(variant: DsTimingEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DS_TIMING_EN` reader - DS Timing Enable."]
pub type DsTimingEnR = crate::BitReader<DsTimingEn>;
impl DsTimingEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DsTimingEn {
        match self.bits {
            false => DsTimingEn::Dis,
            true => DsTimingEn::En,
        }
    }
    #[doc = "Timing from VSYNC and HSYNC."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DsTimingEn::Dis
    }
    #[doc = "Timing embedded in data using SAV and EAV codes."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DsTimingEn::En
    }
}
#[doc = "Field `DS_TIMING_EN` writer - DS Timing Enable."]
pub type DsTimingEnW<'a, REG> = crate::BitWriter<'a, REG, DsTimingEn>;
impl<'a, REG> DsTimingEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timing from VSYNC and HSYNC."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(DsTimingEn::Dis)
    }
    #[doc = "Timing embedded in data using SAV and EAV codes."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(DsTimingEn::En)
    }
}
#[doc = "Field `FIFO_THRSH` reader - Data FIFO Threshold."]
pub type FifoThrshR = crate::FieldReader;
#[doc = "Field `FIFO_THRSH` writer - Data FIFO Threshold."]
pub type FifoThrshW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "DMA Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxDma {
    #[doc = "0: DMA disabled."]
    Dis = 0,
    #[doc = "1: DMA enabled."]
    En = 1,
}
impl From<RxDma> for bool {
    #[inline(always)]
    fn from(variant: RxDma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_DMA` reader - DMA Enable."]
pub type RxDmaR = crate::BitReader<RxDma>;
impl RxDmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxDma {
        match self.bits {
            false => RxDma::Dis,
            true => RxDma::En,
        }
    }
    #[doc = "DMA disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == RxDma::Dis
    }
    #[doc = "DMA enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == RxDma::En
    }
}
#[doc = "Field `RX_DMA` writer - DMA Enable."]
pub type RxDmaW<'a, REG> = crate::BitWriter<'a, REG, RxDma>;
impl<'a, REG> RxDmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(RxDma::Dis)
    }
    #[doc = "DMA enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(RxDma::En)
    }
}
#[doc = "Field `RX_DMA_THRSH` reader - DMA Threshold."]
pub type RxDmaThrshR = crate::FieldReader;
#[doc = "Field `RX_DMA_THRSH` writer - DMA Threshold."]
pub type RxDmaThrshW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `THREE_CH_EN` reader - Three-channel mode enable."]
pub type ThreeChEnR = crate::BitReader;
#[doc = "Field `THREE_CH_EN` writer - Three-channel mode enable."]
pub type ThreeChEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PCIF Control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcifSys {
    #[doc = "0: PCIF disabled."]
    Dis = 0,
    #[doc = "1: PCIF enabled."]
    En = 1,
}
impl From<PcifSys> for bool {
    #[inline(always)]
    fn from(variant: PcifSys) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCIF_SYS` reader - PCIF Control."]
pub type PcifSysR = crate::BitReader<PcifSys>;
impl PcifSysR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PcifSys {
        match self.bits {
            false => PcifSys::Dis,
            true => PcifSys::En,
        }
    }
    #[doc = "PCIF disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PcifSys::Dis
    }
    #[doc = "PCIF enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PcifSys::En
    }
}
#[doc = "Field `PCIF_SYS` writer - PCIF Control."]
pub type PcifSysW<'a, REG> = crate::BitWriter<'a, REG, PcifSys>;
impl<'a, REG> PcifSysW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PCIF disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(PcifSys::Dis)
    }
    #[doc = "PCIF enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(PcifSys::En)
    }
}
impl R {
    #[doc = "Bits 0:1 - Read Mode."]
    #[inline(always)]
    pub fn read_mode(&self) -> ReadModeR {
        ReadModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Data Width."]
    #[inline(always)]
    pub fn data_width(&self) -> DataWidthR {
        DataWidthR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - DS Timing Enable."]
    #[inline(always)]
    pub fn ds_timing_en(&self) -> DsTimingEnR {
        DsTimingEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:9 - Data FIFO Threshold."]
    #[inline(always)]
    pub fn fifo_thrsh(&self) -> FifoThrshR {
        FifoThrshR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - DMA Enable."]
    #[inline(always)]
    pub fn rx_dma(&self) -> RxDmaR {
        RxDmaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - DMA Threshold."]
    #[inline(always)]
    pub fn rx_dma_thrsh(&self) -> RxDmaThrshR {
        RxDmaThrshR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Three-channel mode enable."]
    #[inline(always)]
    pub fn three_ch_en(&self) -> ThreeChEnR {
        ThreeChEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PCIF Control."]
    #[inline(always)]
    pub fn pcif_sys(&self) -> PcifSysR {
        PcifSysR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Read Mode."]
    #[inline(always)]
    pub fn read_mode(&mut self) -> ReadModeW<CtrlSpec> {
        ReadModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Data Width."]
    #[inline(always)]
    pub fn data_width(&mut self) -> DataWidthW<CtrlSpec> {
        DataWidthW::new(self, 2)
    }
    #[doc = "Bit 4 - DS Timing Enable."]
    #[inline(always)]
    pub fn ds_timing_en(&mut self) -> DsTimingEnW<CtrlSpec> {
        DsTimingEnW::new(self, 4)
    }
    #[doc = "Bits 5:9 - Data FIFO Threshold."]
    #[inline(always)]
    pub fn fifo_thrsh(&mut self) -> FifoThrshW<CtrlSpec> {
        FifoThrshW::new(self, 5)
    }
    #[doc = "Bit 16 - DMA Enable."]
    #[inline(always)]
    pub fn rx_dma(&mut self) -> RxDmaW<CtrlSpec> {
        RxDmaW::new(self, 16)
    }
    #[doc = "Bits 17:20 - DMA Threshold."]
    #[inline(always)]
    pub fn rx_dma_thrsh(&mut self) -> RxDmaThrshW<CtrlSpec> {
        RxDmaThrshW::new(self, 17)
    }
    #[doc = "Bit 30 - Three-channel mode enable."]
    #[inline(always)]
    pub fn three_ch_en(&mut self) -> ThreeChEnW<CtrlSpec> {
        ThreeChEnW::new(self, 30)
    }
    #[doc = "Bit 31 - PCIF Control."]
    #[inline(always)]
    pub fn pcif_sys(&mut self) -> PcifSysW<CtrlSpec> {
        PcifSysW::new(self, 31)
    }
}
#[doc = "Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
