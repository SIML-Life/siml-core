use flatbuffers::FlatBufferBuilder;
use siml_core::generated::perception::*;
use siml_core::generated::message::*;

#[tokio::test]
async fn test_message_envelope_roundtrip() {
    let mut fbb = FlatBufferBuilder::new();

    let pos = fbb.create_vector(&[1, 2, 3]);
    let chemo = fbb.create_vector(&[4u8, 5, 6, 7, 8, 9, 10, 11]);

    let mut perception_builder = PerceptionBuilder::new(&mut fbb);
    perception_builder.add_tick(42);
    perception_builder.add_energy(999);
    perception_builder.add_position(pos);
    perception_builder.add_bitfield(3);
    perception_builder.add_chemosense(chemo);
    perception_builder.add_nutrient(123);
    let perception = perception_builder.finish();
    fbb.finish(perception, None);

    let payload = fbb.finished_data();

    let mut fbb2 = FlatBufferBuilder::new();
    let payload_vec = fbb2.create_vector(payload);
    let mut env_builder = MessageEnvelopeBuilder::new(&mut fbb2);
    env_builder.add_version(1);
    env_builder.add_message_type(MessageType::Perception);
    env_builder.add_agent_id(123); // NEW required field
    env_builder.add_payload(payload_vec);
    let env = env_builder.finish();
    fbb2.finish(env, None);

    let buf = fbb2.finished_data();
    let decoded = root_as_message_envelope(buf).unwrap();

    assert_eq!(decoded.version(), 1);
    assert_eq!(decoded.message_type(), MessageType::Perception);
    assert!(decoded.payload().is_some());

    let perception_buf = decoded.payload().unwrap();
    let decoded_perception = root_as_perception(perception_buf.bytes()).unwrap();
    assert_eq!(decoded_perception.energy(), 999);
    assert_eq!(decoded_perception.nutrient(), 123);
}
