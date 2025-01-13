#[doc = "Register `SSTIME` reader"]
pub type R = crate::R<SstimeSpec>;
#[doc = "Register `SSTIME` writer"]
pub type W = crate::W<SstimeSpec>;
#[doc = "Slave Select Pre delay 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pre {
    #[doc = "0: 256 system clocks between SS active and first serial clock edge."]
    _256 = 0,
}
impl From<Pre> for u8 {
    #[inline(always)]
    fn from(variant: Pre) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pre {
    type Ux = u8;
}
impl crate::IsEnum for Pre {}
#[doc = "Field `PRE` reader - Slave Select Pre delay 1."]
pub type PreR = crate::FieldReader<Pre>;
impl PreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pre> {
        match self.bits {
            0 => Some(Pre::_256),
            _ => None,
        }
    }
    #[doc = "256 system clocks between SS active and first serial clock edge."]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Pre::_256
    }
}
#[doc = "Field `PRE` writer - Slave Select Pre delay 1."]
pub type PreW<'a, REG> = crate::FieldWriter<'a, REG, 8, Pre>;
impl<'a, REG> PreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 system clocks between SS active and first serial clock edge."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Pre::_256)
    }
}
#[doc = "Slave Select Post delay 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Post {
    #[doc = "0: 256 system clocks between last serial clock edge and SS inactive."]
    _256 = 0,
}
impl From<Post> for u8 {
    #[inline(always)]
    fn from(variant: Post) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Post {
    type Ux = u8;
}
impl crate::IsEnum for Post {}
#[doc = "Field `POST` reader - Slave Select Post delay 2."]
pub type PostR = crate::FieldReader<Post>;
impl PostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Post> {
        match self.bits {
            0 => Some(Post::_256),
            _ => None,
        }
    }
    #[doc = "256 system clocks between last serial clock edge and SS inactive."]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Post::_256
    }
}
#[doc = "Field `POST` writer - Slave Select Post delay 2."]
pub type PostW<'a, REG> = crate::FieldWriter<'a, REG, 8, Post>;
impl<'a, REG> PostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 system clocks between last serial clock edge and SS inactive."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Post::_256)
    }
}
#[doc = "Slave Select Inactive delay.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Inact {
    #[doc = "0: 256 system clocks between transactions."]
    _256 = 0,
}
impl From<Inact> for u8 {
    #[inline(always)]
    fn from(variant: Inact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Inact {
    type Ux = u8;
}
impl crate::IsEnum for Inact {}
#[doc = "Field `INACT` reader - Slave Select Inactive delay."]
pub type InactR = crate::FieldReader<Inact>;
impl InactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Inact> {
        match self.bits {
            0 => Some(Inact::_256),
            _ => None,
        }
    }
    #[doc = "256 system clocks between transactions."]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Inact::_256
    }
}
#[doc = "Field `INACT` writer - Slave Select Inactive delay."]
pub type InactW<'a, REG> = crate::FieldWriter<'a, REG, 8, Inact>;
impl<'a, REG> InactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "256 system clocks between transactions."]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Inact::_256)
    }
}
impl R {
    #[doc = "Bits 0:7 - Slave Select Pre delay 1."]
    #[inline(always)]
    pub fn pre(&self) -> PreR {
        PreR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Slave Select Post delay 2."]
    #[inline(always)]
    pub fn post(&self) -> PostR {
        PostR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Slave Select Inactive delay."]
    #[inline(always)]
    pub fn inact(&self) -> InactR {
        InactR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave Select Pre delay 1."]
    #[inline(always)]
    pub fn pre(&mut self) -> PreW<SstimeSpec> {
        PreW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Slave Select Post delay 2."]
    #[inline(always)]
    pub fn post(&mut self) -> PostW<SstimeSpec> {
        PostW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Slave Select Inactive delay."]
    #[inline(always)]
    pub fn inact(&mut self) -> InactW<SstimeSpec> {
        InactW::new(self, 16)
    }
}
#[doc = "Register for controlling SPI peripheral/Slave Select Timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`sstime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sstime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SstimeSpec;
impl crate::RegisterSpec for SstimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sstime::R`](R) reader structure"]
impl crate::Readable for SstimeSpec {}
#[doc = "`write(|w| ..)` method takes [`sstime::W`](W) writer structure"]
impl crate::Writable for SstimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSTIME to value 0"]
impl crate::Resettable for SstimeSpec {
    const RESET_VALUE: u32 = 0;
}
