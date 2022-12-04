extern crate flatbuffers;
extern crate flatbuffers_retained;

mod monster_generated;

pub use monster_generated::my_game::sample::{
    root_as_monster, Color, Equipment, Monster, MonsterArgs, Vec3, Weapon, WeaponArgs,
};

fn main() -> Result<(), flatbuffers::InvalidFlatbuffer> {
    let mut builder = flatbuffers::FlatBufferBuilder::new();
    let orc = Monster::create(
        &mut builder,
        &MonsterArgs {
            pos: Some(&Vec3::new(1.0f32, 2.0f32, 3.0f32)),
            mana: 150,
            hp: 80,
            color: Color::Red,
            ..Default::default()
        },
    );
    builder.finish(orc, None);
    let buf = builder.finished_data().to_vec();
    let monster = flatbuffers_retained::FlatbufferRetained::<Monster>::new(buf)?;

    assert_eq!(monster.get().hp(), 80);
    assert_eq!(monster.get().mana(), 150);
    assert_eq!(monster.get().pos().unwrap().y(), 2.0f32);

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
        flatbuffers_retained::SizePrefixedFlatbufferRetained::<Monster>::new(size_prefixed_buf)?;

    let monster2_ptr = monster2.get();
    assert_eq!(monster2_ptr.hp(), 81);

    Ok(())
}
