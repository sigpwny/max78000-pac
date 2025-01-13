#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Cache Bypassed. Instruction data is stored in the line fill buffer but is not written to main cache memory array."]
    Dis = 0,
    #[doc = "1: Cache Enabled."]
    En = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated."]
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
    #[doc = "Cache Bypassed. Instruction data is stored in the line fill buffer but is not written to main cache memory array."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En::Dis
    }
    #[doc = "Cache Enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En::En
    }
}
#[doc = "Field `EN` writer - Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cache Bypassed. Instruction data is stored in the line fill buffer but is not written to main cache memory array."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En::Dis)
    }
    #[doc = "Cache Enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En::En)
    }
}
#[doc = "Cache Ready flag. Cleared by hardware when at any time the cache as a whole is invalidated (including a system reset). When this bit is 0, the cache is effectively in bypass mode (instruction fetches will come from main memory or from the line fill buffer). Set by hardware when the invalidate operation is complete and the cache is ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdy {
    #[doc = "0: Not Ready."]
    NotReady = 0,
    #[doc = "1: Ready."]
    Ready = 1,
}
impl From<Rdy> for bool {
    #[inline(always)]
    fn from(variant: Rdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY` reader - Cache Ready flag. Cleared by hardware when at any time the cache as a whole is invalidated (including a system reset). When this bit is 0, the cache is effectively in bypass mode (instruction fetches will come from main memory or from the line fill buffer). Set by hardware when the invalidate operation is complete and the cache is ready."]
pub type RdyR = crate::BitReader<Rdy>;
impl RdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdy {
        match self.bits {
            false => Rdy::NotReady,
            true => Rdy::Ready,
        }
    }
    #[doc = "Not Ready."]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Rdy::NotReady
    }
    #[doc = "Ready."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Rdy::Ready
    }
}
impl R {
    #[doc = "Bit 0 - Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Cache Ready flag. Cleared by hardware when at any time the cache as a whole is invalidated (including a system reset). When this bit is 0, the cache is effectively in bypass mode (instruction fetches will come from main memory or from the line fill buffer). Set by hardware when the invalidate operation is complete and the cache is ready."]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache Enable. Controls whether the cache is bypassed or is in use. Changing the state of this bit will cause the instruction cache to be flushed and its contents invalidated."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CtrlSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "Cache Control and Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
