pub trait Pkcs7 {
    fn apply_pkcs7(self, block_size: usize) -> Self;
}

impl Pkcs7 for Vec<u8> {
    fn apply_pkcs7(mut self, block_size: usize) -> Self {
        let pkcs7_num = match block_size - self.len() % block_size {
            0 => block_size,
            x => x,
        };

        self.extend(vec![pkcs7_num as u8; pkcs7_num].iter().cloned());
        self
    }
}

#[test]
fn pkcs7_test() {
    let different_than_block_sized = vec![0u8; 3];
    assert_eq!(
        different_than_block_sized.apply_pkcs7(6),
        vec![0, 0, 0, 3, 3, 3]
    );

    let block_sized = vec![0u8; 5];
    assert_eq!(
        block_sized.apply_pkcs7(5),
        vec![0, 0, 0, 0, 0, 5, 5, 5, 5, 5]
    );
}
