#[doc = "Register `REG0` reader"]
pub type R = crate::R<Reg0Spec>;
#[doc = "Register `REG0` writer"]
pub type W = crate::W<Reg0Spec>;
#[doc = "Field `cnnx16_0_pwr_en` reader - CNNx16_0 Power Domain Enable"]
pub type Cnnx16_0PwrEnR = crate::BitReader;
#[doc = "Field `cnnx16_0_pwr_en` writer - CNNx16_0 Power Domain Enable"]
pub type Cnnx16_0PwrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_1_pwr_en` reader - CNNx16_1 Power Domain Enable"]
pub type Cnnx16_1PwrEnR = crate::BitReader;
#[doc = "Field `cnnx16_1_pwr_en` writer - CNNx16_1 Power Domain Enable"]
pub type Cnnx16_1PwrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_2_pwr_en` reader - CNNx16_2 Power Domain Enable"]
pub type Cnnx16_2PwrEnR = crate::BitReader;
#[doc = "Field `cnnx16_2_pwr_en` writer - CNNx16_2 Power Domain Enable"]
pub type Cnnx16_2PwrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_3_pwr_en` reader - CNNx16_3 Power Domain Enable"]
pub type Cnnx16_3PwrEnR = crate::BitReader;
#[doc = "Field `cnnx16_3_pwr_en` writer - CNNx16_3 Power Domain Enable"]
pub type Cnnx16_3PwrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CNNx16_0 Power Domain Enable"]
    #[inline(always)]
    pub fn cnnx16_0_pwr_en(&self) -> Cnnx16_0PwrEnR {
        Cnnx16_0PwrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CNNx16_1 Power Domain Enable"]
    #[inline(always)]
    pub fn cnnx16_1_pwr_en(&self) -> Cnnx16_1PwrEnR {
        Cnnx16_1PwrEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CNNx16_2 Power Domain Enable"]
    #[inline(always)]
    pub fn cnnx16_2_pwr_en(&self) -> Cnnx16_2PwrEnR {
        Cnnx16_2PwrEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CNNx16_3 Power Domain Enable"]
    #[inline(always)]
    pub fn cnnx16_3_pwr_en(&self) -> Cnnx16_3PwrEnR {
        Cnnx16_3PwrEnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CNNx16_0 Power Domain Enable"]
    #[inline(always)]
    pub fn cnnx16_0_pwr_en(&mut self) -> Cnnx16_0PwrEnW<Reg0Spec> {
        Cnnx16_0PwrEnW::new(self, 0)
    }
    #[doc = "Bit 1 - CNNx16_1 Power Domain Enable"]
    #[inline(always)]
    pub fn cnnx16_1_pwr_en(&mut self) -> Cnnx16_1PwrEnW<Reg0Spec> {
        Cnnx16_1PwrEnW::new(self, 1)
    }
    #[doc = "Bit 2 - CNNx16_2 Power Domain Enable"]
    #[inline(always)]
    pub fn cnnx16_2_pwr_en(&mut self) -> Cnnx16_2PwrEnW<Reg0Spec> {
        Cnnx16_2PwrEnW::new(self, 2)
    }
    #[doc = "Bit 3 - CNNx16_3 Power Domain Enable"]
    #[inline(always)]
    pub fn cnnx16_3_pwr_en(&mut self) -> Cnnx16_3PwrEnW<Reg0Spec> {
        Cnnx16_3PwrEnW::new(self, 3)
    }
}
#[doc = "Register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg0Spec;
impl crate::RegisterSpec for Reg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg0::R`](R) reader structure"]
impl crate::Readable for Reg0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg0::W`](W) writer structure"]
impl crate::Writable for Reg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG0 to value 0"]
impl crate::Resettable for Reg0Spec {
    const RESET_VALUE: u32 = 0;
}
