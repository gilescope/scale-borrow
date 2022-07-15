use parity_scale_codec::Decode;

pub trait BorrowDecode<'scale> {
    fn borrow_decode(data: &'scale [u8]) -> Self;
}

impl<'scale> BorrowDecode<'scale> for bool {
    fn borrow_decode(mut data: &'scale [u8]) -> Self {
        let d = &mut data;
        <bool>::decode(d).unwrap()
    }
}

impl<'scale> BorrowDecode<'scale> for &'scale str {
    fn borrow_decode(data: &'scale [u8]) -> Self {
        std::str::from_utf8(data).unwrap()
    }
}

impl<'scale> BorrowDecode<'scale> for &'scale [u8] {
    fn borrow_decode(data: &'scale [u8]) -> Self {
        data
    }
}
