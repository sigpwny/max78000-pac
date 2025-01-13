#[doc = "Register `REG3` reader"]
pub type R = crate::R<Reg3Spec>;
#[doc = "Register `REG3` writer"]
pub type W = crate::W<Reg3Spec>;
#[doc = "Field `cnnx16_0_rst` reader - CNNx16_0 Power Domain Reset"]
pub type Cnnx16_0RstR = crate::BitReader;
#[doc = "Field `cnnx16_0_rst` writer - CNNx16_0 Power Domain Reset"]
pub type Cnnx16_0RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_1_rst` reader - CNNx16_1 Power Domain Reset"]
pub type Cnnx16_1RstR = crate::BitReader;
#[doc = "Field `cnnx16_1_rst` writer - CNNx16_1 Power Domain Reset"]
pub type Cnnx16_1RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_2_rst` reader - CNNx16_2 Power Domain Reset"]
pub type Cnnx16_2RstR = crate::BitReader;
#[doc = "Field `cnnx16_2_rst` writer - CNNx16_2 Power Domain Reset"]
pub type Cnnx16_2RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_3_rst` reader - CNNx16_3 Power Domain Reset"]
pub type Cnnx16_3RstR = crate::BitReader;
#[doc = "Field `cnnx16_3_rst` writer - CNNx16_3 Power Domain Reset"]
pub type Cnnx16_3RstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CNNx16_0 Power Domain Reset"]
    #[inline(always)]
    pub fn cnnx16_0_rst(&self) -> Cnnx16_0RstR {
        Cnnx16_0RstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CNNx16_1 Power Domain Reset"]
    #[inline(always)]
    pub fn cnnx16_1_rst(&self) -> Cnnx16_1RstR {
        Cnnx16_1RstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CNNx16_2 Power Domain Reset"]
    #[inline(always)]
    pub fn cnnx16_2_rst(&self) -> Cnnx16_2RstR {
        Cnnx16_2RstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CNNx16_3 Power Domain Reset"]
    #[inline(always)]
    pub fn cnnx16_3_rst(&self) -> Cnnx16_3RstR {
        Cnnx16_3RstR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CNNx16_0 Power Domain Reset"]
    #[inline(always)]
    pub fn cnnx16_0_rst(&mut self) -> Cnnx16_0RstW<Reg3Spec> {
        Cnnx16_0RstW::new(self, 0)
    }
    #[doc = "Bit 1 - CNNx16_1 Power Domain Reset"]
    #[inline(always)]
    pub fn cnnx16_1_rst(&mut self) -> Cnnx16_1RstW<Reg3Spec> {
        Cnnx16_1RstW::new(self, 1)
    }
    #[doc = "Bit 2 - CNNx16_2 Power Domain Reset"]
    #[inline(always)]
    pub fn cnnx16_2_rst(&mut self) -> Cnnx16_2RstW<Reg3Spec> {
        Cnnx16_2RstW::new(self, 2)
    }
    #[doc = "Bit 3 - CNNx16_3 Power Domain Reset"]
    #[inline(always)]
    pub fn cnnx16_3_rst(&mut self) -> Cnnx16_3RstW<Reg3Spec> {
        Cnnx16_3RstW::new(self, 3)
    }
}
#[doc = "Register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg3Spec;
impl crate::RegisterSpec for Reg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg3::R`](R) reader structure"]
impl crate::Readable for Reg3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg3::W`](W) writer structure"]
impl crate::Writable for Reg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG3 to value 0"]
impl crate::Resettable for Reg3Spec {
    const RESET_VALUE: u32 = 0;
}
