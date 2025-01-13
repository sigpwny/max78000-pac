#[doc = "Register `MSTCTRL` reader"]
pub type R = crate::R<MstctrlSpec>;
#[doc = "Register `MSTCTRL` writer"]
pub type W = crate::W<MstctrlSpec>;
#[doc = "Field `START` reader - Setting this bit to 1 will start a master transfer."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Setting this bit to 1 will start a master transfer."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTART` reader - Setting this bit to 1 will generate a repeated START."]
pub type RestartR = crate::BitReader;
#[doc = "Field `RESTART` writer - Setting this bit to 1 will generate a repeated START."]
pub type RestartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - Setting this bit to 1 will generate a STOP condition."]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Setting this bit to 1 will generate a STOP condition."]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Slave Extend Address Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExAddrEn {
    #[doc = "0: 7-bit address."]
    _7BitsAddress = 0,
    #[doc = "1: 10-bit address."]
    _10BitsAddress = 1,
}
impl From<ExAddrEn> for bool {
    #[inline(always)]
    fn from(variant: ExAddrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EX_ADDR_EN` reader - Slave Extend Address Select."]
pub type ExAddrEnR = crate::BitReader<ExAddrEn>;
impl ExAddrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ExAddrEn {
        match self.bits {
            false => ExAddrEn::_7BitsAddress,
            true => ExAddrEn::_10BitsAddress,
        }
    }
    #[doc = "7-bit address."]
    #[inline(always)]
    pub fn is_7_bits_address(&self) -> bool {
        *self == ExAddrEn::_7BitsAddress
    }
    #[doc = "10-bit address."]
    #[inline(always)]
    pub fn is_10_bits_address(&self) -> bool {
        *self == ExAddrEn::_10BitsAddress
    }
}
#[doc = "Field `EX_ADDR_EN` writer - Slave Extend Address Select."]
pub type ExAddrEnW<'a, REG> = crate::BitWriter<'a, REG, ExAddrEn>;
impl<'a, REG> ExAddrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "7-bit address."]
    #[inline(always)]
    pub fn _7_bits_address(self) -> &'a mut crate::W<REG> {
        self.variant(ExAddrEn::_7BitsAddress)
    }
    #[doc = "10-bit address."]
    #[inline(always)]
    pub fn _10_bits_address(self) -> &'a mut crate::W<REG> {
        self.variant(ExAddrEn::_10BitsAddress)
    }
}
impl R {
    #[doc = "Bit 0 - Setting this bit to 1 will start a master transfer."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting this bit to 1 will generate a repeated START."]
    #[inline(always)]
    pub fn restart(&self) -> RestartR {
        RestartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Setting this bit to 1 will generate a STOP condition."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Slave Extend Address Select."]
    #[inline(always)]
    pub fn ex_addr_en(&self) -> ExAddrEnR {
        ExAddrEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting this bit to 1 will start a master transfer."]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<MstctrlSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Setting this bit to 1 will generate a repeated START."]
    #[inline(always)]
    pub fn restart(&mut self) -> RestartW<MstctrlSpec> {
        RestartW::new(self, 1)
    }
    #[doc = "Bit 2 - Setting this bit to 1 will generate a STOP condition."]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<MstctrlSpec> {
        StopW::new(self, 2)
    }
    #[doc = "Bit 7 - Slave Extend Address Select."]
    #[inline(always)]
    pub fn ex_addr_en(&mut self) -> ExAddrEnW<MstctrlSpec> {
        ExAddrEnW::new(self, 7)
    }
}
#[doc = "Master Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mstctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstctrlSpec;
impl crate::RegisterSpec for MstctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstctrl::R`](R) reader structure"]
impl crate::Readable for MstctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mstctrl::W`](W) writer structure"]
impl crate::Writable for MstctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTCTRL to value 0"]
impl crate::Resettable for MstctrlSpec {
    const RESET_VALUE: u32 = 0;
}
