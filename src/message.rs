pub trait Pod: 'static + Copy + Sized + Send + Sync {}

impl<T: 'static + Copy + Sized + Send + Sync> Pod for T {}

pub trait MessageKind: std::fmt::Display + Pod {}

/// T represents an Enum which tells both sides what kind of message is being
/// passed in the body of the message
pub struct MessageHeader<T: MessageKind> {
    /// The kind of invariant in the message body, used as an identifier
    pub id: T,
    /// the length of the message in bytes
    pub size: usize,
}

pub struct Message<T: MessageKind> {
    pub header: MessageHeader<T>,
    pub body: Vec<u8>,
}

impl<T: MessageKind> Message<T> {
    pub fn new(id: T) -> Self {
        let header = MessageHeader {
            id,
            size: std::mem::size_of::<MessageHeader<T>>(),
        };

        Self {
            header,
            body: vec![],
        }
    }

    pub fn size(&self) -> usize {
        std::mem::size_of::<MessageHeader<T>>() + self.body.len()
    }

    pub fn push<V: Pod>(&mut self, data: V) {
        let i = self.body.len();

        self.body
            .resize(self.body.len() + std::mem::size_of::<V>(), 0);

        unsafe {
            let data_ptr: *const V = &data;
            let byte_ptr: *const u8 = data_ptr as *const _;
            let byte_slice: &[u8] = std::slice::from_raw_parts(byte_ptr, std::mem::size_of::<V>());

            std::ptr::copy(
                &byte_slice[0],
                self.body.as_mut_ptr().offset(i as isize),
                std::mem::size_of::<V>(),
            );
        }

        self.header.size = self.size();
    }

    pub fn pull<V: Pod>(&mut self, bytes: usize) -> V {
        let i = self.body.len() - bytes;

        let out = unsafe {
            let data_ptr = self.body.as_ptr().offset(i as isize);
            let byte_slice: &[u8] = std::slice::from_raw_parts(data_ptr, bytes);

            std::mem::transmute_copy(&byte_slice[0])
        };

        self.body.resize(i, 0);

        self.header.size = self.size();

        out
    }
}

impl<T: MessageKind> std::fmt::Display for Message<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID:{} Size:{}", self.header.id, self.header.size)
    }
}
