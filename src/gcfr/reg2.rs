#[doc = "Register `REG2` reader"]
pub type R = crate::R<Reg2Spec>;
#[doc = "Register `REG2` writer"]
pub type W = crate::W<Reg2Spec>;
#[doc = "Field `cnnx16_0_iso` reader - CNNx16_0 Power Domain Isolation"]
pub type Cnnx16_0IsoR = crate::BitReader;
#[doc = "Field `cnnx16_0_iso` writer - CNNx16_0 Power Domain Isolation"]
pub type Cnnx16_0IsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_1_iso` reader - CNNx16_1 Power Domain Isolation"]
pub type Cnnx16_1IsoR = crate::BitReader;
#[doc = "Field `cnnx16_1_iso` writer - CNNx16_1 Power Domain Isolation"]
pub type Cnnx16_1IsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_2_iso` reader - CNNx16_2 Power Domain Isolation"]
pub type Cnnx16_2IsoR = crate::BitReader;
#[doc = "Field `cnnx16_2_iso` writer - CNNx16_2 Power Domain Isolation"]
pub type Cnnx16_2IsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_3_iso` reader - CNNx16_3 Power Domain Isolation"]
pub type Cnnx16_3IsoR = crate::BitReader;
#[doc = "Field `cnnx16_3_iso` writer - CNNx16_3 Power Domain Isolation"]
pub type Cnnx16_3IsoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CNNx16_0 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_0_iso(&self) -> Cnnx16_0IsoR {
        Cnnx16_0IsoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CNNx16_1 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_1_iso(&self) -> Cnnx16_1IsoR {
        Cnnx16_1IsoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CNNx16_2 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_2_iso(&self) -> Cnnx16_2IsoR {
        Cnnx16_2IsoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CNNx16_3 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_3_iso(&self) -> Cnnx16_3IsoR {
        Cnnx16_3IsoR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CNNx16_0 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_0_iso(&mut self) -> Cnnx16_0IsoW<Reg2Spec> {
        Cnnx16_0IsoW::new(self, 0)
    }
    #[doc = "Bit 1 - CNNx16_1 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_1_iso(&mut self) -> Cnnx16_1IsoW<Reg2Spec> {
        Cnnx16_1IsoW::new(self, 1)
    }
    #[doc = "Bit 2 - CNNx16_2 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_2_iso(&mut self) -> Cnnx16_2IsoW<Reg2Spec> {
        Cnnx16_2IsoW::new(self, 2)
    }
    #[doc = "Bit 3 - CNNx16_3 Power Domain Isolation"]
    #[inline(always)]
    pub fn cnnx16_3_iso(&mut self) -> Cnnx16_3IsoW<Reg2Spec> {
        Cnnx16_3IsoW::new(self, 3)
    }
}
#[doc = "Register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg2Spec;
impl crate::RegisterSpec for Reg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg2::R`](R) reader structure"]
impl crate::Readable for Reg2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg2::W`](W) writer structure"]
impl crate::Writable for Reg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG2 to value 0"]
impl crate::Resettable for Reg2Spec {
    const RESET_VALUE: u32 = 0;
}
