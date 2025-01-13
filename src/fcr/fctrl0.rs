#[doc = "Register `FCTRL0` reader"]
pub type R = crate::R<Fctrl0Spec>;
#[doc = "Register `FCTRL0` writer"]
pub type W = crate::W<Fctrl0Spec>;
#[doc = "I2C0 SDA Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c0dgen0 {
    #[doc = "0: Deglitcher disabled."]
    Dis = 0,
    #[doc = "1: Deglitcher enabled."]
    En = 1,
}
impl From<I2c0dgen0> for bool {
    #[inline(always)]
    fn from(variant: I2c0dgen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0DGEN0` reader - I2C0 SDA Pad Deglitcher enable."]
pub type I2c0dgen0R = crate::BitReader<I2c0dgen0>;
impl I2c0dgen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c0dgen0 {
        match self.bits {
            false => I2c0dgen0::Dis,
            true => I2c0dgen0::En,
        }
    }
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2c0dgen0::Dis
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2c0dgen0::En
    }
}
#[doc = "Field `I2C0DGEN0` writer - I2C0 SDA Pad Deglitcher enable."]
pub type I2c0dgen0W<'a, REG> = crate::BitWriter<'a, REG, I2c0dgen0>;
impl<'a, REG> I2c0dgen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0dgen0::Dis)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0dgen0::En)
    }
}
#[doc = "I2C0 SCL Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c0dgen1 {
    #[doc = "0: Deglitcher disabled."]
    Dis = 0,
    #[doc = "1: Deglitcher enabled."]
    En = 1,
}
impl From<I2c0dgen1> for bool {
    #[inline(always)]
    fn from(variant: I2c0dgen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0DGEN1` reader - I2C0 SCL Pad Deglitcher enable."]
pub type I2c0dgen1R = crate::BitReader<I2c0dgen1>;
impl I2c0dgen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c0dgen1 {
        match self.bits {
            false => I2c0dgen1::Dis,
            true => I2c0dgen1::En,
        }
    }
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2c0dgen1::Dis
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2c0dgen1::En
    }
}
#[doc = "Field `I2C0DGEN1` writer - I2C0 SCL Pad Deglitcher enable."]
pub type I2c0dgen1W<'a, REG> = crate::BitWriter<'a, REG, I2c0dgen1>;
impl<'a, REG> I2c0dgen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0dgen1::Dis)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0dgen1::En)
    }
}
#[doc = "I2C1 SDA Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c1dgen0 {
    #[doc = "0: Deglitcher disabled."]
    Dis = 0,
    #[doc = "1: Deglitcher enabled."]
    En = 1,
}
impl From<I2c1dgen0> for bool {
    #[inline(always)]
    fn from(variant: I2c1dgen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1DGEN0` reader - I2C1 SDA Pad Deglitcher enable."]
pub type I2c1dgen0R = crate::BitReader<I2c1dgen0>;
impl I2c1dgen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c1dgen0 {
        match self.bits {
            false => I2c1dgen0::Dis,
            true => I2c1dgen0::En,
        }
    }
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2c1dgen0::Dis
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2c1dgen0::En
    }
}
#[doc = "Field `I2C1DGEN0` writer - I2C1 SDA Pad Deglitcher enable."]
pub type I2c1dgen0W<'a, REG> = crate::BitWriter<'a, REG, I2c1dgen0>;
impl<'a, REG> I2c1dgen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1dgen0::Dis)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1dgen0::En)
    }
}
#[doc = "I2C1 SCL Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c1dgen1 {
    #[doc = "0: Deglitcher disabled."]
    Dis = 0,
    #[doc = "1: Deglitcher enabled."]
    En = 1,
}
impl From<I2c1dgen1> for bool {
    #[inline(always)]
    fn from(variant: I2c1dgen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1DGEN1` reader - I2C1 SCL Pad Deglitcher enable."]
pub type I2c1dgen1R = crate::BitReader<I2c1dgen1>;
impl I2c1dgen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c1dgen1 {
        match self.bits {
            false => I2c1dgen1::Dis,
            true => I2c1dgen1::En,
        }
    }
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2c1dgen1::Dis
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2c1dgen1::En
    }
}
#[doc = "Field `I2C1DGEN1` writer - I2C1 SCL Pad Deglitcher enable."]
pub type I2c1dgen1W<'a, REG> = crate::BitWriter<'a, REG, I2c1dgen1>;
impl<'a, REG> I2c1dgen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1dgen1::Dis)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1dgen1::En)
    }
}
#[doc = "I2C2 SDA Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c2dgen0 {
    #[doc = "0: Deglitcher disabled."]
    Dis = 0,
    #[doc = "1: Deglitcher enabled."]
    En = 1,
}
impl From<I2c2dgen0> for bool {
    #[inline(always)]
    fn from(variant: I2c2dgen0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2DGEN0` reader - I2C2 SDA Pad Deglitcher enable."]
pub type I2c2dgen0R = crate::BitReader<I2c2dgen0>;
impl I2c2dgen0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c2dgen0 {
        match self.bits {
            false => I2c2dgen0::Dis,
            true => I2c2dgen0::En,
        }
    }
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2c2dgen0::Dis
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2c2dgen0::En
    }
}
#[doc = "Field `I2C2DGEN0` writer - I2C2 SDA Pad Deglitcher enable."]
pub type I2c2dgen0W<'a, REG> = crate::BitWriter<'a, REG, I2c2dgen0>;
impl<'a, REG> I2c2dgen0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2dgen0::Dis)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2dgen0::En)
    }
}
#[doc = "I2C2 SCL Pad Deglitcher enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c2dgen1 {
    #[doc = "0: Deglitcher disabled."]
    Dis = 0,
    #[doc = "1: Deglitcher enabled."]
    En = 1,
}
impl From<I2c2dgen1> for bool {
    #[inline(always)]
    fn from(variant: I2c2dgen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2DGEN1` reader - I2C2 SCL Pad Deglitcher enable."]
pub type I2c2dgen1R = crate::BitReader<I2c2dgen1>;
impl I2c2dgen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c2dgen1 {
        match self.bits {
            false => I2c2dgen1::Dis,
            true => I2c2dgen1::En,
        }
    }
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == I2c2dgen1::Dis
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == I2c2dgen1::En
    }
}
#[doc = "Field `I2C2DGEN1` writer - I2C2 SCL Pad Deglitcher enable."]
pub type I2c2dgen1W<'a, REG> = crate::BitWriter<'a, REG, I2c2dgen1>;
impl<'a, REG> I2c2dgen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deglitcher disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2dgen1::Dis)
    }
    #[doc = "Deglitcher enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(I2c2dgen1::En)
    }
}
impl R {
    #[doc = "Bit 20 - I2C0 SDA Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c0dgen0(&self) -> I2c0dgen0R {
        I2c0dgen0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C0 SCL Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c0dgen1(&self) -> I2c0dgen1R {
        I2c0dgen1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 SDA Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c1dgen0(&self) -> I2c1dgen0R {
        I2c1dgen0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C1 SCL Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c1dgen1(&self) -> I2c1dgen1R {
        I2c1dgen1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I2C2 SDA Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c2dgen0(&self) -> I2c2dgen0R {
        I2c2dgen0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - I2C2 SCL Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c2dgen1(&self) -> I2c2dgen1R {
        I2c2dgen1R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - I2C0 SDA Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c0dgen0(&mut self) -> I2c0dgen0W<Fctrl0Spec> {
        I2c0dgen0W::new(self, 20)
    }
    #[doc = "Bit 21 - I2C0 SCL Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c0dgen1(&mut self) -> I2c0dgen1W<Fctrl0Spec> {
        I2c0dgen1W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C1 SDA Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c1dgen0(&mut self) -> I2c1dgen0W<Fctrl0Spec> {
        I2c1dgen0W::new(self, 22)
    }
    #[doc = "Bit 23 - I2C1 SCL Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c1dgen1(&mut self) -> I2c1dgen1W<Fctrl0Spec> {
        I2c1dgen1W::new(self, 23)
    }
    #[doc = "Bit 24 - I2C2 SDA Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c2dgen0(&mut self) -> I2c2dgen0W<Fctrl0Spec> {
        I2c2dgen0W::new(self, 24)
    }
    #[doc = "Bit 25 - I2C2 SCL Pad Deglitcher enable."]
    #[inline(always)]
    pub fn i2c2dgen1(&mut self) -> I2c2dgen1W<Fctrl0Spec> {
        I2c2dgen1W::new(self, 25)
    }
}
#[doc = "Function Control 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fctrl0Spec;
impl crate::RegisterSpec for Fctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctrl0::R`](R) reader structure"]
impl crate::Readable for Fctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`fctrl0::W`](W) writer structure"]
impl crate::Writable for Fctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCTRL0 to value 0"]
impl crate::Resettable for Fctrl0Spec {
    const RESET_VALUE: u32 = 0;
}
