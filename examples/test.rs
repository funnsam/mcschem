fn main() {
    let mut schem = mcschem::Schematic::new(mcschem::MC_1_18_2, 3, 3, 3);

    schem.set_block(1, 0, 0, mcschem::Block::from_str("minecraft:dirt").unwrap());
    schem.set_block(0, 1, 0, mcschem::Block::from_str("minecraft:stone").unwrap());
    schem.set_block(0, 0, 1, mcschem::Block::from_str("minecraft:grass_block").unwrap());

    let mut file = std::fs::File::create("schematic.schem").unwrap();
    schem.export(&mut file).unwrap();
}
