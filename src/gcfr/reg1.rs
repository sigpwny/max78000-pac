#[doc = "Register `REG1` reader"]
pub type R = crate::R<Reg1Spec>;
#[doc = "Register `REG1` writer"]
pub type W = crate::W<Reg1Spec>;
#[doc = "Field `cnnx16_0_ram_en` reader - CNNx16_0 RAM Power Enable"]
pub type Cnnx16_0RamEnR = crate::BitReader;
#[doc = "Field `cnnx16_0_ram_en` writer - CNNx16_0 RAM Power Enable"]
pub type Cnnx16_0RamEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_1_ram_en` reader - CNNx16_1 RAM Power Enable"]
pub type Cnnx16_1RamEnR = crate::BitReader;
#[doc = "Field `cnnx16_1_ram_en` writer - CNNx16_1 RAM Power Enable"]
pub type Cnnx16_1RamEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_2_ram_en` reader - CNNx16_2 RAM Power Enable"]
pub type Cnnx16_2RamEnR = crate::BitReader;
#[doc = "Field `cnnx16_2_ram_en` writer - CNNx16_2 RAM Power Enable"]
pub type Cnnx16_2RamEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cnnx16_3_ram_en` reader - CNNx16_3 RAM Power Enable"]
pub type Cnnx16_3RamEnR = crate::BitReader;
#[doc = "Field `cnnx16_3_ram_en` writer - CNNx16_3 RAM Power Enable"]
pub type Cnnx16_3RamEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CNNx16_0 RAM Power Enable"]
    #[inline(always)]
    pub fn cnnx16_0_ram_en(&self) -> Cnnx16_0RamEnR {
        Cnnx16_0RamEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CNNx16_1 RAM Power Enable"]
    #[inline(always)]
    pub fn cnnx16_1_ram_en(&self) -> Cnnx16_1RamEnR {
        Cnnx16_1RamEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CNNx16_2 RAM Power Enable"]
    #[inline(always)]
    pub fn cnnx16_2_ram_en(&self) -> Cnnx16_2RamEnR {
        Cnnx16_2RamEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CNNx16_3 RAM Power Enable"]
    #[inline(always)]
    pub fn cnnx16_3_ram_en(&self) -> Cnnx16_3RamEnR {
        Cnnx16_3RamEnR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CNNx16_0 RAM Power Enable"]
    #[inline(always)]
    pub fn cnnx16_0_ram_en(&mut self) -> Cnnx16_0RamEnW<Reg1Spec> {
        Cnnx16_0RamEnW::new(self, 0)
    }
    #[doc = "Bit 1 - CNNx16_1 RAM Power Enable"]
    #[inline(always)]
    pub fn cnnx16_1_ram_en(&mut self) -> Cnnx16_1RamEnW<Reg1Spec> {
        Cnnx16_1RamEnW::new(self, 1)
    }
    #[doc = "Bit 2 - CNNx16_2 RAM Power Enable"]
    #[inline(always)]
    pub fn cnnx16_2_ram_en(&mut self) -> Cnnx16_2RamEnW<Reg1Spec> {
        Cnnx16_2RamEnW::new(self, 2)
    }
    #[doc = "Bit 3 - CNNx16_3 RAM Power Enable"]
    #[inline(always)]
    pub fn cnnx16_3_ram_en(&mut self) -> Cnnx16_3RamEnW<Reg1Spec> {
        Cnnx16_3RamEnW::new(self, 3)
    }
}
#[doc = "Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg1Spec;
impl crate::RegisterSpec for Reg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg1::R`](R) reader structure"]
impl crate::Readable for Reg1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg1::W`](W) writer structure"]
impl crate::Writable for Reg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG1 to value 0"]
impl crate::Resettable for Reg1Spec {
    const RESET_VALUE: u32 = 0;
}
