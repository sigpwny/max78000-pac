#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EN` reader - AES Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - AES Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RX_EN` reader - DMA Request To Read Data Output FIFO"]
pub type DmaRxEnR = crate::BitReader;
#[doc = "Field `DMA_RX_EN` writer - DMA Request To Read Data Output FIFO"]
pub type DmaRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_TX_EN` reader - DMA Request To Write Data Input FIFO"]
pub type DmaTxEnR = crate::BitReader;
#[doc = "Field `DMA_TX_EN` writer - DMA Request To Write Data Input FIFO"]
pub type DmaTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Start AES Calculation"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start AES Calculation"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_FLUSH` reader - Flush the data input FIFO"]
pub type InputFlushR = crate::BitReader;
#[doc = "Field `INPUT_FLUSH` writer - Flush the data input FIFO"]
pub type InputFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPUT_FLUSH` reader - Flush the data output FIFO"]
pub type OutputFlushR = crate::BitReader;
#[doc = "Field `OUTPUT_FLUSH` writer - Flush the data output FIFO"]
pub type OutputFlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Encryption Key Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KeySize {
    #[doc = "0: 128 Bits."]
    Aes128 = 0,
    #[doc = "1: 192 Bits."]
    Aes192 = 1,
    #[doc = "2: 256 Bits."]
    Aes256 = 2,
}
impl From<KeySize> for u8 {
    #[inline(always)]
    fn from(variant: KeySize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KeySize {
    type Ux = u8;
}
impl crate::IsEnum for KeySize {}
#[doc = "Field `KEY_SIZE` reader - Encryption Key Size"]
pub type KeySizeR = crate::FieldReader<KeySize>;
impl KeySizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<KeySize> {
        match self.bits {
            0 => Some(KeySize::Aes128),
            1 => Some(KeySize::Aes192),
            2 => Some(KeySize::Aes256),
            _ => None,
        }
    }
    #[doc = "128 Bits."]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == KeySize::Aes128
    }
    #[doc = "192 Bits."]
    #[inline(always)]
    pub fn is_aes192(&self) -> bool {
        *self == KeySize::Aes192
    }
    #[doc = "256 Bits."]
    #[inline(always)]
    pub fn is_aes256(&self) -> bool {
        *self == KeySize::Aes256
    }
}
#[doc = "Field `KEY_SIZE` writer - Encryption Key Size"]
pub type KeySizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, KeySize>;
impl<'a, REG> KeySizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128 Bits."]
    #[inline(always)]
    pub fn aes128(self) -> &'a mut crate::W<REG> {
        self.variant(KeySize::Aes128)
    }
    #[doc = "192 Bits."]
    #[inline(always)]
    pub fn aes192(self) -> &'a mut crate::W<REG> {
        self.variant(KeySize::Aes192)
    }
    #[doc = "256 Bits."]
    #[inline(always)]
    pub fn aes256(self) -> &'a mut crate::W<REG> {
        self.variant(KeySize::Aes256)
    }
}
#[doc = "Encryption Type Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Type {
    #[doc = "0: Encryption using the external AES key."]
    EncExt = 0,
    #[doc = "1: Decryption using the external AES key."]
    DecExt = 1,
    #[doc = "2: Decryption using the locally generated decryption key."]
    DecInt = 2,
}
impl From<Type> for u8 {
    #[inline(always)]
    fn from(variant: Type) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Type {
    type Ux = u8;
}
impl crate::IsEnum for Type {}
#[doc = "Field `TYPE` reader - Encryption Type Selection"]
pub type TypeR = crate::FieldReader<Type>;
impl TypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Type> {
        match self.bits {
            0 => Some(Type::EncExt),
            1 => Some(Type::DecExt),
            2 => Some(Type::DecInt),
            _ => None,
        }
    }
    #[doc = "Encryption using the external AES key."]
    #[inline(always)]
    pub fn is_enc_ext(&self) -> bool {
        *self == Type::EncExt
    }
    #[doc = "Decryption using the external AES key."]
    #[inline(always)]
    pub fn is_dec_ext(&self) -> bool {
        *self == Type::DecExt
    }
    #[doc = "Decryption using the locally generated decryption key."]
    #[inline(always)]
    pub fn is_dec_int(&self) -> bool {
        *self == Type::DecInt
    }
}
#[doc = "Field `TYPE` writer - Encryption Type Selection"]
pub type TypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Type>;
impl<'a, REG> TypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Encryption using the external AES key."]
    #[inline(always)]
    pub fn enc_ext(self) -> &'a mut crate::W<REG> {
        self.variant(Type::EncExt)
    }
    #[doc = "Decryption using the external AES key."]
    #[inline(always)]
    pub fn dec_ext(self) -> &'a mut crate::W<REG> {
        self.variant(Type::DecExt)
    }
    #[doc = "Decryption using the locally generated decryption key."]
    #[inline(always)]
    pub fn dec_int(self) -> &'a mut crate::W<REG> {
        self.variant(Type::DecInt)
    }
}
impl R {
    #[doc = "Bit 0 - AES Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Request To Read Data Output FIFO"]
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DmaRxEnR {
        DmaRxEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Request To Write Data Input FIFO"]
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DmaTxEnR {
        DmaTxEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start AES Calculation"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flush the data input FIFO"]
    #[inline(always)]
    pub fn input_flush(&self) -> InputFlushR {
        InputFlushR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flush the data output FIFO"]
    #[inline(always)]
    pub fn output_flush(&self) -> OutputFlushR {
        OutputFlushR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Encryption Key Size"]
    #[inline(always)]
    pub fn key_size(&self) -> KeySizeR {
        KeySizeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Encryption Type Selection"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AES Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Request To Read Data Output FIFO"]
    #[inline(always)]
    pub fn dma_rx_en(&mut self) -> DmaRxEnW<CtrlSpec> {
        DmaRxEnW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Request To Write Data Input FIFO"]
    #[inline(always)]
    pub fn dma_tx_en(&mut self) -> DmaTxEnW<CtrlSpec> {
        DmaTxEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Start AES Calculation"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<CtrlSpec> {
        StartW::new(self, 3)
    }
    #[doc = "Bit 4 - Flush the data input FIFO"]
    #[inline(always)]
    pub fn input_flush(&mut self) -> InputFlushW<CtrlSpec> {
        InputFlushW::new(self, 4)
    }
    #[doc = "Bit 5 - Flush the data output FIFO"]
    #[inline(always)]
    pub fn output_flush(&mut self) -> OutputFlushW<CtrlSpec> {
        OutputFlushW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Encryption Key Size"]
    #[inline(always)]
    pub fn key_size(&mut self) -> KeySizeW<CtrlSpec> {
        KeySizeW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Encryption Type Selection"]
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<CtrlSpec> {
        TypeW::new(self, 8)
    }
}
#[doc = "AES Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
