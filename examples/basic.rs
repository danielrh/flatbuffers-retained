extern crate flatbuffers_retained;
extern crate flatbuffers;

mod monster_generated;

pub use monster_generated::my_game::sample::{root_as_monster,
                                             Color, Equipment,
                                             Monster, MonsterArgs,
                                             Vec3,
                                             Weapon, WeaponArgs};

fn main() -> Result<(), flatbuffers::InvalidFlatbuffer> {
    // Build up a serialized buffer algorithmically.
    // Initialize it with a capacity of 1024 bytes.
    let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(1024);
    let orc = Monster::create(&mut builder, &MonsterArgs{
        pos: Some(&Vec3::new(1.0f32, 2.0f32, 3.0f32)),
        mana: 150,
        hp: 80,
        color: Color::Red,
       ..Default::default()
    });
    builder.finish(orc, None);
    let buf = builder.finished_data().to_vec();
    flatbuffers_retained::FlatbufferRetained::<Monster>::new(buf).unwrap()?
}