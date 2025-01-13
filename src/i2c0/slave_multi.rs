#[doc = "Register `SLAVE_MULTI[%s]` reader"]
pub type R = crate::R<SlaveMultiSpec>;
#[doc = "Register `SLAVE_MULTI[%s]` writer"]
pub type W = crate::W<SlaveMultiSpec>;
#[doc = "Field `ADDR` reader - Slave Address."]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Slave Address."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DIS` reader - Slave Disable."]
pub type DisR = crate::BitReader;
#[doc = "Field `DIS` writer - Slave Disable."]
pub type DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Extended Address Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtAddrEn {
    #[doc = "0: 7-bit address."]
    _7BitsAddress = 0,
    #[doc = "1: 10-bit address."]
    _10BitsAddress = 1,
}
impl From<ExtAddrEn> for bool {
    #[inline(always)]
    fn from(variant: ExtAddrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXT_ADDR_EN` reader - Extended Address Select."]
pub type ExtAddrEnR = crate::BitReader<ExtAddrEn>;
impl ExtAddrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ExtAddrEn {
        match self.bits {
            false => ExtAddrEn::_7BitsAddress,
            true => ExtAddrEn::_10BitsAddress,
        }
    }
    #[doc = "7-bit address."]
    #[inline(always)]
    pub fn is_7_bits_address(&self) -> bool {
        *self == ExtAddrEn::_7BitsAddress
    }
    #[doc = "10-bit address."]
    #[inline(always)]
    pub fn is_10_bits_address(&self) -> bool {
        *self == ExtAddrEn::_10BitsAddress
    }
}
#[doc = "Field `EXT_ADDR_EN` writer - Extended Address Select."]
pub type ExtAddrEnW<'a, REG> = crate::BitWriter<'a, REG, ExtAddrEn>;
impl<'a, REG> ExtAddrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "7-bit address."]
    #[inline(always)]
    pub fn _7_bits_address(self) -> &'a mut crate::W<REG> {
        self.variant(ExtAddrEn::_7BitsAddress)
    }
    #[doc = "10-bit address."]
    #[inline(always)]
    pub fn _10_bits_address(self) -> &'a mut crate::W<REG> {
        self.variant(ExtAddrEn::_10BitsAddress)
    }
}
impl R {
    #[doc = "Bits 0:9 - Slave Address."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Slave Disable."]
    #[inline(always)]
    pub fn dis(&self) -> DisR {
        DisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Extended Address Select."]
    #[inline(always)]
    pub fn ext_addr_en(&self) -> ExtAddrEnR {
        ExtAddrEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave Address."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<SlaveMultiSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bit 10 - Slave Disable."]
    #[inline(always)]
    pub fn dis(&mut self) -> DisW<SlaveMultiSpec> {
        DisW::new(self, 10)
    }
    #[doc = "Bit 15 - Extended Address Select."]
    #[inline(always)]
    pub fn ext_addr_en(&mut self) -> ExtAddrEnW<SlaveMultiSpec> {
        ExtAddrEnW::new(self, 15)
    }
}
#[doc = "Slave Address Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slave_multi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave_multi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlaveMultiSpec;
impl crate::RegisterSpec for SlaveMultiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave_multi::R`](R) reader structure"]
impl crate::Readable for SlaveMultiSpec {}
#[doc = "`write(|w| ..)` method takes [`slave_multi::W`](W) writer structure"]
impl crate::Writable for SlaveMultiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLAVE_MULTI[%s]
to value 0"]
impl crate::Resettable for SlaveMultiSpec {
    const RESET_VALUE: u32 = 0;
}
