use flatbuffers::FlatBufferBuilder;
use siml_core::generated::action::*;
use siml_core::generated::message::*;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

pub async fn run_env_stub() {
    let mut stream = TcpStream::connect("127.0.0.1:6000").await.unwrap();

    let mut fbb = FlatBufferBuilder::new();
    let direction = fbb.create_vector(&[1, 0, 0]);
    let mut action_builder = ActionBuilder::new(&mut fbb);
    action_builder.add_direction(direction);
    action_builder.add_use_energy(true);
    action_builder.add_emit_signal(1);
    let action = action_builder.finish();
    fbb.finish(action, None);

    let payload = fbb.finished_data();

    // Wrap in Envelope
    let mut fbb2 = FlatBufferBuilder::new();
    let payload_vec = fbb2.create_vector(payload);
    let mut env_builder = MessageEnvelopeBuilder::new(&mut fbb2);
    env_builder.add_version(1);
    env_builder.add_message_type(MessageType::Action);
    env_builder.add_agent_id(123); // NEW required field
    env_builder.add_payload(payload_vec);
    let envelope = env_builder.finish();
    fbb2.finish(envelope, None);

    stream.write_all(fbb2.finished_data()).await.unwrap();
    println!("🌍 Env sent Action");
}
