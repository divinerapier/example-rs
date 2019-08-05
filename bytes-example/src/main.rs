use bytes::{BigEndian, BufMut, BytesMut};

fn main() {
    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"hello world"[..]);
    buf.put_u16::<BigEndian>(1234);

    let a = buf.take();
    assert_eq!(a, b"hello world\x04\xD2"[..]);

    buf.put(&b"goodbye world"[..]);

    let b = buf.take();
    assert_eq!(b, b"goodbye world"[..]);
    assert_eq!(buf.capacity(), 998);

}
