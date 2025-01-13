#[doc = "Register `SYSCTRL` reader"]
pub type R = crate::R<SysctrlSpec>;
#[doc = "Register `SYSCTRL` writer"]
pub type W = crate::W<SysctrlSpec>;
#[doc = "Field `BSTAPEN` reader - Boundary Scan TAP enable. When enabled, the JTAG port is conneted to the Boundary Scan TAP instead of the ARM ICE."]
pub type BstapenR = crate::BitReader;
#[doc = "Field `BSTAPEN` writer - Boundary Scan TAP enable. When enabled, the JTAG port is conneted to the Boundary Scan TAP instead of the ARM ICE."]
pub type BstapenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlashPageFlip {
    #[doc = "0: Physical layout matches logical layout."]
    Normal = 0,
    #[doc = "1: Bottom half mapped to logical top half and vice versa."]
    Swapped = 1,
}
impl From<FlashPageFlip> for bool {
    #[inline(always)]
    fn from(variant: FlashPageFlip) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH_PAGE_FLIP` reader - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
pub type FlashPageFlipR = crate::BitReader<FlashPageFlip>;
impl FlashPageFlipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FlashPageFlip {
        match self.bits {
            false => FlashPageFlip::Normal,
            true => FlashPageFlip::Swapped,
        }
    }
    #[doc = "Physical layout matches logical layout."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FlashPageFlip::Normal
    }
    #[doc = "Bottom half mapped to logical top half and vice versa."]
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == FlashPageFlip::Swapped
    }
}
#[doc = "Field `FLASH_PAGE_FLIP` writer - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
pub type FlashPageFlipW<'a, REG> = crate::BitWriter<'a, REG, FlashPageFlip>;
impl<'a, REG> FlashPageFlipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Physical layout matches logical layout."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(FlashPageFlip::Normal)
    }
    #[doc = "Bottom half mapped to logical top half and vice versa."]
    #[inline(always)]
    pub fn swapped(self) -> &'a mut crate::W<REG> {
        self.variant(FlashPageFlip::Swapped)
    }
}
#[doc = "Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icc0Flush {
    #[doc = "0: Normal Code Cache Operation"]
    Normal = 0,
    #[doc = "1: Code Caches and CPU instruction buffer are flushed"]
    Flush = 1,
}
impl From<Icc0Flush> for bool {
    #[inline(always)]
    fn from(variant: Icc0Flush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICC0_FLUSH` reader - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
pub type Icc0FlushR = crate::BitReader<Icc0Flush>;
impl Icc0FlushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icc0Flush {
        match self.bits {
            false => Icc0Flush::Normal,
            true => Icc0Flush::Flush,
        }
    }
    #[doc = "Normal Code Cache Operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Icc0Flush::Normal
    }
    #[doc = "Code Caches and CPU instruction buffer are flushed"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == Icc0Flush::Flush
    }
}
#[doc = "Field `ICC0_FLUSH` writer - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
pub type Icc0FlushW<'a, REG> = crate::BitWriter<'a, REG, Icc0Flush>;
impl<'a, REG> Icc0FlushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Code Cache Operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Icc0Flush::Normal)
    }
    #[doc = "Code Caches and CPU instruction buffer are flushed"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(Icc0Flush::Flush)
    }
}
#[doc = "Field `ROMDONE` reader - ROM_DONE status. Used to disable SWD interface during system initialization procedure"]
pub type RomdoneR = crate::BitReader;
#[doc = "Field `ROMDONE` writer - ROM_DONE status. Used to disable SWD interface during system initialization procedure"]
pub type RomdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cchk {
    #[doc = "0: No operation/complete."]
    Complete = 0,
    #[doc = "1: Start operation."]
    Start = 1,
}
impl From<Cchk> for bool {
    #[inline(always)]
    fn from(variant: Cchk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCHK` reader - Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed."]
pub type CchkR = crate::BitReader<Cchk>;
impl CchkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cchk {
        match self.bits {
            false => Cchk::Complete,
            true => Cchk::Start,
        }
    }
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == Cchk::Complete
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Cchk::Start
    }
}
#[doc = "Field `CCHK` writer - Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed."]
pub type CchkW<'a, REG> = crate::BitWriter<'a, REG, Cchk>;
impl<'a, REG> CchkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No operation/complete."]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(Cchk::Complete)
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Cchk::Start)
    }
}
#[doc = "Field `SWD_DIS` reader - Serial Wire Debug Disable. This bit is used to disable the serial wire debug interface This bit is only writeable if (FMV lock word is not programmed) or if (ICE lock word is not programmed and the ROM_DONE bit is not set)."]
pub type SwdDisR = crate::BitReader;
#[doc = "Field `SWD_DIS` writer - Serial Wire Debug Disable. This bit is used to disable the serial wire debug interface This bit is only writeable if (FMV lock word is not programmed) or if (ICE lock word is not programmed and the ROM_DONE bit is not set)."]
pub type SwdDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ROM Checksum Result. This bit is only valid when CHKRD=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chkres {
    #[doc = "0: ROM Checksum Correct."]
    Pass = 0,
    #[doc = "1: ROM Checksum Fail."]
    Fail = 1,
}
impl From<Chkres> for bool {
    #[inline(always)]
    fn from(variant: Chkres) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHKRES` reader - ROM Checksum Result. This bit is only valid when CHKRD=1."]
pub type ChkresR = crate::BitReader<Chkres>;
impl ChkresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chkres {
        match self.bits {
            false => Chkres::Pass,
            true => Chkres::Fail,
        }
    }
    #[doc = "ROM Checksum Correct."]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == Chkres::Pass
    }
    #[doc = "ROM Checksum Fail."]
    #[inline(always)]
    pub fn is_fail(&self) -> bool {
        *self == Chkres::Fail
    }
}
#[doc = "Field `CHKRES` writer - ROM Checksum Result. This bit is only valid when CHKRD=1."]
pub type ChkresW<'a, REG> = crate::BitWriter<'a, REG, Chkres>;
impl<'a, REG> ChkresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ROM Checksum Correct."]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(Chkres::Pass)
    }
    #[doc = "ROM Checksum Fail."]
    #[inline(always)]
    pub fn fail(self) -> &'a mut crate::W<REG> {
        self.variant(Chkres::Fail)
    }
}
#[doc = "Field `OVR` reader - Operating Voltage Range."]
pub type OvrR = crate::FieldReader;
#[doc = "Field `OVR` writer - Operating Voltage Range."]
pub type OvrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - Boundary Scan TAP enable. When enabled, the JTAG port is conneted to the Boundary Scan TAP instead of the ARM ICE."]
    #[inline(always)]
    pub fn bstapen(&self) -> BstapenR {
        BstapenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
    #[inline(always)]
    pub fn flash_page_flip(&self) -> FlashPageFlipR {
        FlashPageFlipR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
    #[inline(always)]
    pub fn icc0_flush(&self) -> Icc0FlushR {
        Icc0FlushR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - ROM_DONE status. Used to disable SWD interface during system initialization procedure"]
    #[inline(always)]
    pub fn romdone(&self) -> RomdoneR {
        RomdoneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed."]
    #[inline(always)]
    pub fn cchk(&self) -> CchkR {
        CchkR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Serial Wire Debug Disable. This bit is used to disable the serial wire debug interface This bit is only writeable if (FMV lock word is not programmed) or if (ICE lock word is not programmed and the ROM_DONE bit is not set)."]
    #[inline(always)]
    pub fn swd_dis(&self) -> SwdDisR {
        SwdDisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ROM Checksum Result. This bit is only valid when CHKRD=1."]
    #[inline(always)]
    pub fn chkres(&self) -> ChkresR {
        ChkresR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Operating Voltage Range."]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Boundary Scan TAP enable. When enabled, the JTAG port is conneted to the Boundary Scan TAP instead of the ARM ICE."]
    #[inline(always)]
    pub fn bstapen(&mut self) -> BstapenW<SysctrlSpec> {
        BstapenW::new(self, 1)
    }
    #[doc = "Bit 4 - Flips the Flash bottom and top halves. (Depending on the total flash size, each half is either 256K or 512K). Initiating a flash page flip will cause a flush of both the data buffer on the DCODE bus and the internal instruction buffer."]
    #[inline(always)]
    pub fn flash_page_flip(&mut self) -> FlashPageFlipW<SysctrlSpec> {
        FlashPageFlipW::new(self, 4)
    }
    #[doc = "Bit 6 - Code Cache Flush. This bit is used to flush the code caches and the instruction buffer of the Cortex-M4."]
    #[inline(always)]
    pub fn icc0_flush(&mut self) -> Icc0FlushW<SysctrlSpec> {
        Icc0FlushW::new(self, 6)
    }
    #[doc = "Bit 12 - ROM_DONE status. Used to disable SWD interface during system initialization procedure"]
    #[inline(always)]
    pub fn romdone(&mut self) -> RomdoneW<SysctrlSpec> {
        RomdoneW::new(self, 12)
    }
    #[doc = "Bit 13 - Compute ROM Checksum. This bit is self-cleared when calculation is completed. Once set, software clearing this bit is ignored and the bit will remain set until the operation is completed."]
    #[inline(always)]
    pub fn cchk(&mut self) -> CchkW<SysctrlSpec> {
        CchkW::new(self, 13)
    }
    #[doc = "Bit 14 - Serial Wire Debug Disable. This bit is used to disable the serial wire debug interface This bit is only writeable if (FMV lock word is not programmed) or if (ICE lock word is not programmed and the ROM_DONE bit is not set)."]
    #[inline(always)]
    pub fn swd_dis(&mut self) -> SwdDisW<SysctrlSpec> {
        SwdDisW::new(self, 14)
    }
    #[doc = "Bit 15 - ROM Checksum Result. This bit is only valid when CHKRD=1."]
    #[inline(always)]
    pub fn chkres(&mut self) -> ChkresW<SysctrlSpec> {
        ChkresW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Operating Voltage Range."]
    #[inline(always)]
    pub fn ovr(&mut self) -> OvrW<SysctrlSpec> {
        OvrW::new(self, 16)
    }
}
#[doc = "System Control.\n\nYou can [`read`](crate::Reg::read) this register and get [`sysctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysctrlSpec;
impl crate::RegisterSpec for SysctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysctrl::R`](R) reader structure"]
impl crate::Readable for SysctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sysctrl::W`](W) writer structure"]
impl crate::Writable for SysctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCTRL to value 0"]
impl crate::Resettable for SysctrlSpec {
    const RESET_VALUE: u32 = 0;
}
