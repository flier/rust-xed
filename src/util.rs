use crate::ffi;

pub trait SignExtend {
    type Signed;

    /// arbitrary sign extension from a qty of "bits" length
    fn arbitrary(self, bits: u32) -> Self::Signed;
}

impl SignExtend for u64 {
    type Signed = i64;

    fn arbitrary(self, bits: u32) -> Self::Signed {
        unsafe { ffi::xed_sign_extend_arbitrary_to_64(self, bits) }
    }
}

impl SignExtend for u32 {
    type Signed = i32;

    fn arbitrary(self, bits: u32) -> Self::Signed {
        unsafe { ffi::xed_sign_extend_arbitrary_to_32(self, bits) }
    }
}
