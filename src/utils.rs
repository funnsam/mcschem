use super::*;

pub fn barrel_ss(ss: usize) -> Vec<ItemSlot> {
    let n = ((ss * 27).div_ceil(14) - 2).max(ss);
    let mut items = Vec::with_capacity(n);

    for i in 0..n {
        items.push(ItemSlot {
            id: "minecraft:redstone_block".to_string(),
            extra: nbt::compound! {},
            count: 64,
            slot: i as i8
        })
    }

    items
}
