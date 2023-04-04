use std::borrow::Cow;

struct LazyBuffer<'a> {
    data: Cow<'a, [u8]>,
}

impl<'a> LazyBuffer<'a> {

    pub fn new(data: &'a[u8]) -> Self {
        Self {
            data: Cow::Borrowed(data),
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn append(&mut self, data: &[u8]) {
        self.data.to_mut().extend(data)
    }
}


fn main() {
    let data = vec![0u8; 10];

    // No memory copied yet
    let mut buffer = LazyBuffer::new(&data);
    println!("{:?}", buffer.data());

    // The data is cloned
    buffer.append(&[1, 2, 3]);
    println!("{:?}", buffer.data());

    // The data is not cloned on further attempts
    buffer.append(&[4, 5, 6]);
    println!("{:?}", buffer.data());    
}
