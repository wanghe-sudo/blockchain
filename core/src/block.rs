// 区块头
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String,
}

// 区块数据结构
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}


// impl   for Block {
//
// }