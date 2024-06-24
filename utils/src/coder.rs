use bincode;
use serde::{Deserialize, Serialize, Serializer};

pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    let serialized = bincode::serialize(value).unwrap();
    println!("序列化后结果：{:?}", serialized);
    serialized
}

pub fn my_deserialize<'a, T>(bytes: &'a [u8]) -> T
where
    T: Deserialize<'a>,
{
    let deserialized = bincode::deserialize(bytes).unwrap();

    deserialized
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use crate::coder::{my_deserialize, my_serialize};
    use crate::coder::Point;

    #[test]
    fn coder_works() {
        let point = Point {
            x: 1,
            y: 1,
        };
        let de = my_serialize(&point);
        let deserialize: Point = my_deserialize(&de);
        assert_eq!(point.x, deserialize.x);
    }
}