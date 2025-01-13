#[doc = "Register `LPCN` reader"]
pub type R = crate::R<LpcnSpec>;
#[doc = "Register `LPCN` writer"]
pub type W = crate::W<LpcnSpec>;
#[doc = "System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramret0 {
    #[doc = "0: Disable Ram Retention."]
    Dis = 0,
    #[doc = "1: Enable System RAM 0 retention."]
    En = 1,
}
impl From<Ramret0> for bool {
    #[inline(always)]
    fn from(variant: Ramret0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMRET0` reader - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret0R = crate::BitReader<Ramret0>;
impl Ramret0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramret0 {
        match self.bits {
            false => Ramret0::Dis,
            true => Ramret0::En,
        }
    }
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ramret0::Dis
    }
    #[doc = "Enable System RAM 0 retention."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ramret0::En
    }
}
#[doc = "Field `RAMRET0` writer - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret0W<'a, REG> = crate::BitWriter<'a, REG, Ramret0>;
impl<'a, REG> Ramret0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret0::Dis)
    }
    #[doc = "Enable System RAM 0 retention."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret0::En)
    }
}
#[doc = "System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramret1 {
    #[doc = "0: Disable Ram Retention."]
    Dis = 0,
    #[doc = "1: Enable System RAM 1 retention."]
    En = 1,
}
impl From<Ramret1> for bool {
    #[inline(always)]
    fn from(variant: Ramret1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMRET1` reader - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret1R = crate::BitReader<Ramret1>;
impl Ramret1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramret1 {
        match self.bits {
            false => Ramret1::Dis,
            true => Ramret1::En,
        }
    }
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ramret1::Dis
    }
    #[doc = "Enable System RAM 1 retention."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ramret1::En
    }
}
#[doc = "Field `RAMRET1` writer - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret1W<'a, REG> = crate::BitWriter<'a, REG, Ramret1>;
impl<'a, REG> Ramret1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret1::Dis)
    }
    #[doc = "Enable System RAM 1 retention."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret1::En)
    }
}
#[doc = "System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramret2 {
    #[doc = "0: Disable Ram Retention."]
    Dis = 0,
    #[doc = "1: Enable System RAM 2 retention."]
    En = 1,
}
impl From<Ramret2> for bool {
    #[inline(always)]
    fn from(variant: Ramret2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMRET2` reader - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret2R = crate::BitReader<Ramret2>;
impl Ramret2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramret2 {
        match self.bits {
            false => Ramret2::Dis,
            true => Ramret2::En,
        }
    }
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ramret2::Dis
    }
    #[doc = "Enable System RAM 2 retention."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ramret2::En
    }
}
#[doc = "Field `RAMRET2` writer - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret2W<'a, REG> = crate::BitWriter<'a, REG, Ramret2>;
impl<'a, REG> Ramret2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret2::Dis)
    }
    #[doc = "Enable System RAM 2 retention."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret2::En)
    }
}
#[doc = "System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramret3 {
    #[doc = "0: Disable Ram Retention."]
    Dis = 0,
    #[doc = "1: Enable System RAM 3 retention."]
    En = 1,
}
impl From<Ramret3> for bool {
    #[inline(always)]
    fn from(variant: Ramret3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMRET3` reader - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret3R = crate::BitReader<Ramret3>;
impl Ramret3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramret3 {
        match self.bits {
            false => Ramret3::Dis,
            true => Ramret3::En,
        }
    }
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ramret3::Dis
    }
    #[doc = "Enable System RAM 3 retention."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ramret3::En
    }
}
#[doc = "Field `RAMRET3` writer - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
pub type Ramret3W<'a, REG> = crate::BitWriter<'a, REG, Ramret3>;
impl<'a, REG> Ramret3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Ram Retention."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret3::Dis)
    }
    #[doc = "Enable System RAM 3 retention."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ramret3::En)
    }
}
#[doc = "Field `LPMCLKSEL` reader - Low Power Mode APB Clock Select."]
pub type LpmclkselR = crate::BitReader;
#[doc = "Field `LPMCLKSEL` writer - Low Power Mode APB Clock Select."]
pub type LpmclkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMFAST` reader - Low Power Mode Clock Select."]
pub type LpmfastR = crate::BitReader;
#[doc = "Field `LPMFAST` writer - Low Power Mode Clock Select."]
pub type LpmfastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Bandgap OFF. This controls the System Bandgap in DeepSleep mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BgDis {
    #[doc = "0: Bandgap is always ON."]
    On = 0,
    #[doc = "1: Bandgap is OFF in DeepSleep mode (default)."]
    Off = 1,
}
impl From<BgDis> for bool {
    #[inline(always)]
    fn from(variant: BgDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BG_DIS` reader - Bandgap OFF. This controls the System Bandgap in DeepSleep mode."]
pub type BgDisR = crate::BitReader<BgDis>;
impl BgDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BgDis {
        match self.bits {
            false => BgDis::On,
            true => BgDis::Off,
        }
    }
    #[doc = "Bandgap is always ON."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == BgDis::On
    }
    #[doc = "Bandgap is OFF in DeepSleep mode (default)."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == BgDis::Off
    }
}
#[doc = "Field `BG_DIS` writer - Bandgap OFF. This controls the System Bandgap in DeepSleep mode."]
pub type BgDisW<'a, REG> = crate::BitWriter<'a, REG, BgDis>;
impl<'a, REG> BgDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bandgap is always ON."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(BgDis::On)
    }
    #[doc = "Bandgap is OFF in DeepSleep mode (default)."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(BgDis::Off)
    }
}
#[doc = "Field `LPWKST_CLR` reader - Low Power Wakeup Status Register Clear"]
pub type LpwkstClrR = crate::BitReader;
#[doc = "Field `LPWKST_CLR` writer - Low Power Wakeup Status Register Clear"]
pub type LpwkstClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret0(&self) -> Ramret0R {
        Ramret0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret1(&self) -> Ramret1R {
        Ramret1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret2(&self) -> Ramret2R {
        Ramret2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret3(&self) -> Ramret3R {
        Ramret3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Low Power Mode APB Clock Select."]
    #[inline(always)]
    pub fn lpmclksel(&self) -> LpmclkselR {
        LpmclkselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Low Power Mode Clock Select."]
    #[inline(always)]
    pub fn lpmfast(&self) -> LpmfastR {
        LpmfastR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Bandgap OFF. This controls the System Bandgap in DeepSleep mode."]
    #[inline(always)]
    pub fn bg_dis(&self) -> BgDisR {
        BgDisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 31 - Low Power Wakeup Status Register Clear"]
    #[inline(always)]
    pub fn lpwkst_clr(&self) -> LpwkstClrR {
        LpwkstClrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret0(&mut self) -> Ramret0W<LpcnSpec> {
        Ramret0W::new(self, 0)
    }
    #[doc = "Bit 1 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret1(&mut self) -> Ramret1W<LpcnSpec> {
        Ramret1W::new(self, 1)
    }
    #[doc = "Bit 2 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret2(&mut self) -> Ramret2W<LpcnSpec> {
        Ramret2W::new(self, 2)
    }
    #[doc = "Bit 3 - System RAM retention in BACKUP mode. These two bits are used in conjuction with RREGEN bit."]
    #[inline(always)]
    pub fn ramret3(&mut self) -> Ramret3W<LpcnSpec> {
        Ramret3W::new(self, 3)
    }
    #[doc = "Bit 8 - Low Power Mode APB Clock Select."]
    #[inline(always)]
    pub fn lpmclksel(&mut self) -> LpmclkselW<LpcnSpec> {
        LpmclkselW::new(self, 8)
    }
    #[doc = "Bit 9 - Low Power Mode Clock Select."]
    #[inline(always)]
    pub fn lpmfast(&mut self) -> LpmfastW<LpcnSpec> {
        LpmfastW::new(self, 9)
    }
    #[doc = "Bit 11 - Bandgap OFF. This controls the System Bandgap in DeepSleep mode."]
    #[inline(always)]
    pub fn bg_dis(&mut self) -> BgDisW<LpcnSpec> {
        BgDisW::new(self, 11)
    }
    #[doc = "Bit 31 - Low Power Wakeup Status Register Clear"]
    #[inline(always)]
    pub fn lpwkst_clr(&mut self) -> LpwkstClrW<LpcnSpec> {
        LpwkstClrW::new(self, 31)
    }
}
#[doc = "Low Power Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpcnSpec;
impl crate::RegisterSpec for LpcnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpcn::R`](R) reader structure"]
impl crate::Readable for LpcnSpec {}
#[doc = "`write(|w| ..)` method takes [`lpcn::W`](W) writer structure"]
impl crate::Writable for LpcnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPCN to value 0"]
impl crate::Resettable for LpcnSpec {
    const RESET_VALUE: u32 = 0;
}
