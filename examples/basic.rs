extern crate flatbuffers_retained;
extern crate flatbuffers;

use flatbuffers_retained::FlatbufferRetained;
mod monster_generated;

pub use monster_generated::my_game::sample::{root_as_monster,
                                             Color, Equipment,
                                             Monster, MonsterArgs,
                                             Vec3,
                                             Weapon, WeaponArgs};

struct SerializedMonster<'a> {
    monster: flatbuffers_retained::FlatbufferRetained<'a, Monster<'a>>,
}

impl<'a> SerializedMonster<'a> {
    pub fn new(data: Vec<u8>) -> Result<Self, flatbuffers::InvalidFlatbuffer> {
        Ok(SerializedMonster {
	    monster: FlatbufferRetained::new(data)?
	})
    }
    pub fn get_hp(&self) -> i16 {
        self.monster.get().hp()
    }
    pub fn get(&'a self) -> <Monster<'a> as flatbuffers::Follow<'a>>::Inner {
        self.monster.get()
    }
}

fn main() -> Result<(), flatbuffers::InvalidFlatbuffer> {
    // Build up a serialized buffer algorithmically.
    // Initialize it with a capacity of 1024 bytes.
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    let orc = Monster::create(&mut builder, &MonsterArgs{
        pos: Some(&Vec3::new(1.0f32, 2.0f32, 3.0f32)),
        mana: 150,
        hp: 80,
        color: Color::Red,
       ..Default::default()
    });
    builder.finish(orc, None);
    let buf = builder.finished_data().to_vec();
    let monster = SerializedMonster::new(buf)?;
    assert_eq!(monster.get_hp(), 80);
    assert_eq!(monster.get().mana(), 150);
    assert_eq!(monster.get().pos().unwrap().y(), 2.0f32);

    // We cannot pass a non prefixed buffer into a SizePrefixedFlatbufferRetained struct.
    flatbuffers_retained::SizePrefixedFlatbufferRetained::<Monster>::new(builder.finished_data().to_vec()).map(|_|false).unwrap_err();

    builder.reset();
    let orc2 = Monster::create(&mut builder, &MonsterArgs{
        pos: Some(&Vec3::new(1.0f32, 2.0f32, 3.0f32)),
        mana: 220,
        hp: 81,
        color: Color::Blue,
       ..Default::default()
    });
    builder.finish_size_prefixed(orc2, None);
    let size_prefixed_buf = builder.finished_data().to_vec();
    let monster2 = flatbuffers_retained::SizePrefixedFlatbufferRetained::<Monster>::new(size_prefixed_buf)?;

    let monster2_ptr = monster2.get();
    assert_eq!(monster2_ptr.hp(), 81);

    // We cannot pass a size prefixed buffer into a FlatbufferRetained struct.
    flatbuffers_retained::FlatbufferRetained::<Monster>::new(builder.finished_data().to_vec()).map(|_|false).unwrap_err();
    Ok(())
}