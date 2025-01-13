#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Channel Status. This bit is used to indicate to the programmer when it is safe to change the configuration, address, and count registers for the channel. Whenever this bit is cleared by hardware, the DMA_CFG.CHEN bit is also cleared (if not cleared already).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: Disable."]
    Dis = 0,
    #[doc = "1: Enable."]
    En = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - Channel Status. This bit is used to indicate to the programmer when it is safe to change the configuration, address, and count registers for the channel. Whenever this bit is cleared by hardware, the DMA_CFG.CHEN bit is also cleared (if not cleared already)."]
pub type StatusR = crate::BitReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            false => Status::Dis,
            true => Status::En,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Status::Dis
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Status::En
    }
}
#[doc = "Channel Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ipend {
    #[doc = "0: No interrupt is pending."]
    Inactive = 0,
    #[doc = "1: An interrupt is pending."]
    Pending = 1,
}
impl From<Ipend> for bool {
    #[inline(always)]
    fn from(variant: Ipend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPEND` reader - Channel Interrupt."]
pub type IpendR = crate::BitReader<Ipend>;
impl IpendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ipend {
        match self.bits {
            false => Ipend::Inactive,
            true => Ipend::Pending,
        }
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Ipend::Inactive
    }
    #[doc = "An interrupt is pending."]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == Ipend::Pending
    }
}
#[doc = "Field `CTZ_IF` reader - Count-to-Zero (CTZ) Interrupt Flag"]
pub type CtzIfR = crate::BitReader;
#[doc = "Field `CTZ_IF` writer - Count-to-Zero (CTZ) Interrupt Flag"]
pub type CtzIfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RLD_IF` reader - Reload Event Interrupt Flag."]
pub type RldIfR = crate::BitReader;
#[doc = "Field `RLD_IF` writer - Reload Event Interrupt Flag."]
pub type RldIfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BUS_ERR` reader - Bus Error. Indicates that an AHB abort was received and the channel has been disabled."]
pub type BusErrR = crate::BitReader;
#[doc = "Field `BUS_ERR` writer - Bus Error. Indicates that an AHB abort was received and the channel has been disabled."]
pub type BusErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TO_IF` reader - Time-Out Event Interrupt Flag."]
pub type ToIfR = crate::BitReader;
#[doc = "Field `TO_IF` writer - Time-Out Event Interrupt Flag."]
pub type ToIfW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel Status. This bit is used to indicate to the programmer when it is safe to change the configuration, address, and count registers for the channel. Whenever this bit is cleared by hardware, the DMA_CFG.CHEN bit is also cleared (if not cleared already)."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Interrupt."]
    #[inline(always)]
    pub fn ipend(&self) -> IpendR {
        IpendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Count-to-Zero (CTZ) Interrupt Flag"]
    #[inline(always)]
    pub fn ctz_if(&self) -> CtzIfR {
        CtzIfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reload Event Interrupt Flag."]
    #[inline(always)]
    pub fn rld_if(&self) -> RldIfR {
        RldIfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus Error. Indicates that an AHB abort was received and the channel has been disabled."]
    #[inline(always)]
    pub fn bus_err(&self) -> BusErrR {
        BusErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Time-Out Event Interrupt Flag."]
    #[inline(always)]
    pub fn to_if(&self) -> ToIfR {
        ToIfR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Count-to-Zero (CTZ) Interrupt Flag"]
    #[inline(always)]
    pub fn ctz_if(&mut self) -> CtzIfW<StatusSpec> {
        CtzIfW::new(self, 2)
    }
    #[doc = "Bit 3 - Reload Event Interrupt Flag."]
    #[inline(always)]
    pub fn rld_if(&mut self) -> RldIfW<StatusSpec> {
        RldIfW::new(self, 3)
    }
    #[doc = "Bit 4 - Bus Error. Indicates that an AHB abort was received and the channel has been disabled."]
    #[inline(always)]
    pub fn bus_err(&mut self) -> BusErrW<StatusSpec> {
        BusErrW::new(self, 4)
    }
    #[doc = "Bit 6 - Time-Out Event Interrupt Flag."]
    #[inline(always)]
    pub fn to_if(&mut self) -> ToIfW<StatusSpec> {
        ToIfW::new(self, 6)
    }
}
#[doc = "DMA Channel Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x5c;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
