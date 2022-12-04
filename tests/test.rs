extern crate flatbuffers;
extern crate flatbuffers_retained;

use flatbuffers_retained::FlatbufferRetained;

mod monster_generated;

pub use monster_generated::my_game::sample::{
    root_as_monster, Color, Equipment, Monster, MonsterArgs, Vec3, Weapon, WeaponArgs,
};

fn do_some_checks<'a>(monster: &SerializedMonster<'a>) {
    assert_eq!(monster.get_hp(), 80);
    assert_eq!(monster.get().mana(), 150);
    assert_eq!(monster.get().pos().unwrap().y(), 2.0f32);
}

#[derive(Default)]
struct MonstersHolder<'a> {
    monsters: std::collections::HashMap<i16, SerializedMonster<'a>>,
}
impl<'a> MonstersHolder<'a> {
    fn do_some_checks(&self) {
        for v in self.monsters.values() {
            do_some_checks(v);
        }
    }
}

struct SerializedMonster<'a> {
    monster: flatbuffers_retained::FlatbufferRetained<'a, Monster<'a>>,
}

impl<'a> SerializedMonster<'a> {
    pub fn new(data: Vec<u8>) -> Result<Self, flatbuffers::InvalidFlatbuffer> {
        Ok(SerializedMonster {
            monster: FlatbufferRetained::new(data)?,
        })
    }
    pub fn get_hp(&self) -> i16 {
        self.monster.get().hp()
    }
    pub fn get(&'a self) -> <Monster<'a> as flatbuffers::Follow<'a>>::Inner {
        self.monster.get()
    }
}

#[test]
fn test_stored_in_map() {
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    let orc = Monster::create(
        &mut builder,
        &MonsterArgs {
            pos: Some(&Vec3::new(1.0f32, 2.0f32, 3.0f32)),
            mana: 150,
            hp: 80,
            ..Default::default()
        },
    );
    builder.finish(orc, None);

    let buf = builder.finished_data().to_vec();
    let mut monsters = MonstersHolder::default();
    monsters
        .monsters
        .insert(1, SerializedMonster::new(buf).unwrap());
    monsters.do_some_checks();
    monsters.monsters.insert(
        2,
        SerializedMonster::new(builder.finished_data().to_vec()).unwrap(),
    );
    monsters.do_some_checks();
}

#[test]
fn test_unprefixed() {
    // Build up a serialized buffer algorithmically.
    // Initialize it with a capacity of 1024 bytes.
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    let orc = Monster::create(
        &mut builder,
        &MonsterArgs {
            pos: Some(&Vec3::new(1.0f32, 2.0f32, 3.0f32)),
            mana: 150,
            hp: 80,
            ..Default::default()
        },
    );
    builder.finish(orc, None);
    let buf = builder.finished_data().to_vec();
    let monster = SerializedMonster::new(buf).unwrap();
    assert_eq!(monster.get_hp(), 80);
    assert_eq!(monster.get().mana(), 150);
    assert_eq!(monster.get().pos().unwrap().y(), 2.0f32);

    // We cannot pass a non prefixed buffer into a SizePrefixedFlatbufferRetained struct.
    flatbuffers_retained::SizePrefixedFlatbufferRetained::<Monster>::new(
        builder.finished_data().to_vec(),
    )
    .map(|_| false)
    .unwrap_err();
}

#[test]
fn test_size_prefixed() {
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    let orc2 = Monster::create(
        &mut builder,
        &MonsterArgs {
            pos: Some(&Vec3::new(1.0f32, 2.0f32, 3.0f32)),
            mana: 220,
            hp: 81,
            color: Color::Blue,
            ..Default::default()
        },
    );
    builder.finish_size_prefixed(orc2, None);
    let size_prefixed_buf = builder.finished_data().to_vec();
    let monster2 =
        flatbuffers_retained::SizePrefixedFlatbufferRetained::<Monster>::new(size_prefixed_buf)
            .unwrap();

    let monster2_ptr = monster2.get();
    assert_eq!(monster2_ptr.hp(), 81);

    // We cannot pass a size prefixed buffer into a FlatbufferRetained struct.
    flatbuffers_retained::FlatbufferRetained::<Monster>::new(builder.finished_data().to_vec())
        .map(|_| false)
        .unwrap_err();
}

#[test]
fn test_retained() {
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    let orc = Monster::create(
        &mut builder,
        &MonsterArgs {
            pos: Some(&Vec3::new(1.0f32, 2.0f32, 3.0f32)),
            mana: 150,
            hp: 80,
            ..Default::default()
        },
    );
    builder.finish(orc, None);
    let buf = builder.finished_data().to_vec();
    let monster = flatbuffers_retained::Retained::<Monster>::new_unprefixed(buf).unwrap();

    assert_eq!(&builder.finished_data(), &monster.as_ref());

    // Make sure this fails to validate in a size-prefixed way.
    flatbuffers_retained::Retained::<Monster>::new_size_prefixed(builder.finished_data().to_vec())
        .map(|_| false)
        .unwrap_err();

    builder.reset();
    let orc2 = Monster::create(
        &mut builder,
        &MonsterArgs {
            pos: Some(&Vec3::new(1.0f32, 2.0f32, 3.0f32)),
            mana: 220,
            hp: 81,
            color: Color::Blue,
            ..Default::default()
        },
    );
    builder.finish_size_prefixed(orc2, None);
    let size_prefixed_buf = builder.finished_data().to_vec();
    let monster2 =
        flatbuffers_retained::Retained::<Monster>::new_size_prefixed(size_prefixed_buf).unwrap();
    // Make sure this fails to validate in an unprefixed way.
    flatbuffers_retained::Retained::<Monster>::new_unprefixed(builder.finished_data().to_vec())
        .map(|_| false)
        .unwrap_err();

    let monster2_ptr = monster2.get();
    assert_eq!(monster2_ptr.hp(), 81);

    assert_eq!(monster.get().hp(), 80);

    let slice: &[u8] = (&monster2).into();
    assert_eq!(slice, builder.finished_data());
}
