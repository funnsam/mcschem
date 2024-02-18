use std::collections::{BTreeMap, HashMap};
use std::io;
use std::fmt;

pub const MC_1_18_2: i32 = 2975;

/// A struct holding infomation about a schematic
#[derive(Debug, Clone)]
pub struct Schematic {
    data_version: i32,

    blocks: Vec<Vec<Vec<Block>>>,
    block_entities: HashMap<[u16; 3], BlockEntity>,
    size_x: u16,
    size_y: u16,
    size_z: u16,
}

/// A block with ID and properties
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    id: String,
    properties: BTreeMap<String, String>,
}

/// A block entity
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum BlockEntity {
    Barrel {
        items: Vec<ItemSlot>,
    },
}

/// An item slot in a container
#[derive(Debug, Clone)]
pub struct ItemSlot {
}

impl Block {
    /// Parse a block from a string
    ///
    /// # Example
    /// ```rs
    /// assert!(Block::from_str("minecraft:oak_log[axis=y]").is_ok());
    /// ```
    pub fn from_str(block: &str) -> Result<Self, ()> {
        let (id, properties) = block
            .split_once('[')
            .map_or_else(
                || (block, None),
                |(a, b)| (a, Some(b))
            );

        let mut prop = BTreeMap::new();
        if let Some(properties) = properties {
            if !matches!(properties.chars().last(), Some(']')) {
                return Err(());
            }

            let properties = &properties[..properties.len()-1];

            for property in properties.split(',') {
                let (k, v) = property.split_once('=').ok_or(())?;
                prop.insert(k.to_string(), v.to_string());
            }
        }

        Ok(Self {
            id: id.to_string(), properties: prop
        })
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.id.fmt(f)?;

        if self.properties.len() != 0 {
            write!(
                f,
                "[{}]",
                self.properties
                    .iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect::<Vec<String>>()
                    .join(",")
            )?;
        }

        Ok(())
    }
}

impl Schematic {
    /// Initialize a new schematic filled with `minecraft:air`
    pub fn new(data_version: i32, size_x: u16, size_y: u16, size_z: u16) -> Self {
        Self {
            data_version,

            blocks: vec![
                vec![
                    vec![
                        Block::from_str("minecraft:air").unwrap(); size_x as usize
                    ]; size_z as usize
                ]; size_y as usize
            ],
            block_entities: HashMap::new(),
            size_x, size_y, size_z
        }
    }

    /// Sets a block in the schematic
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) {
        self.blocks[y][z][x] = block
    }

    /// Export the schematic to a writer
    pub fn export<W: io::Write>(&self, writer: &mut W) -> Result<(), quartz_nbt::io::NbtIoError> {
        use quartz_nbt as nbt;

        let mut palette = Vec::new();
        let mut block_data = Vec::new();
        for y in self.blocks.iter() {
            for z in y.iter() {
                for block in z.iter() {
                    if !palette.contains(block) {
                        palette.push(block.clone());
                    }

                    let mut id = palette.iter().position(|v| v == block).unwrap();

                    while id & 0x80 != 0 {
                        block_data.push(id as u8 & 0x7F | 0x80);
                        id >>= 7;
                    }
                    block_data.push(id as u8);
                }
            }
        }

        let mut palette_nbt = nbt::NbtCompound::new();
        for (bi, b) in palette.iter().enumerate() {
            palette_nbt.insert(format!("{b}"), nbt::NbtTag::Int(bi as i32));
        }

        let mut block_entities = vec![];
        for (p, e) in self.block_entities.iter() {
            block_entities.push(nbt::compound! {
                "Pos": [I; p[0] as i32, p[1] as i32, p[2] as i32],
                // TODO:
            });
        }

        let schem = nbt::compound! {
            "Version": 2_i32,
            "DataVersion": self.data_version,
            "Metadata": nbt::compound! {},
            "Width": self.size_x as i16,
            "Height": self.size_y as i16,
            "Length": self.size_z as i16,
            "PaletteMax": (palette.len() - 1) as i32,
            "Palette": palette_nbt,
            "BlockData": nbt::NbtList::from(block_data),
            "BlockEntities": nbt::NbtList::from(block_entities),
        };

        println!("{schem:#?}");

        nbt::io::write_nbt(writer, Some("Schematic"), &schem, nbt::io::Flavor::GzCompressed)
    }
}
