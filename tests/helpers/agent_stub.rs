use flatbuffers::FlatBufferBuilder;
use siml_core::generated::handshake::{Handshake, HandshakeArgs, Role};
use siml_core::generated::message::{MessageEnvelope, MessageEnvelopeArgs, MessageType};
use siml_core::generated::perception::{Perception, PerceptionArgs};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub async fn run_agent_stub(agent_id: u32) {
    let mut stream = TcpStream::connect("127.0.0.1:6000").await.unwrap();

    // Step 1: Send Handshake
    let mut fbb = FlatBufferBuilder::new();

    let agent_types = ["Worm", "Ant", "Fish", "Spider", "Bee"];
    let chosen_type = agent_types.choose(&mut thread_rng()).unwrap();
    let agent_type = fbb.create_string(chosen_type);

    let args = HandshakeArgs {
        agent_id,
        role: Role::Agent,
        agent_type: Some(agent_type),
    };
    let handshake = Handshake::create(&mut fbb, &args);
    fbb.finish(handshake, None);

    stream.write_all(fbb.finished_data()).await.unwrap();
    if agent_id <= 10 {
        println!("🤝 Agent {agent_id} ({chosen_type}) sent Handshake");
    }

    // Step 2: Send Perception
    let mut fbb = FlatBufferBuilder::new();
    let pos = fbb.create_vector(&[1, 2, 3]);
    let chemo = fbb.create_vector(&[4u8, 5, 6, 7, 8, 9, 10, 11]);
    let args = PerceptionArgs {
        tick: 42,
        energy: 1500,
        position: Some(pos),
        bitfield: 3,
        chemosense: Some(chemo),
        nutrient: 25,
        extra: None,
    };
    let perception = Perception::create(&mut fbb, &args);
    fbb.finish(perception, None);

    let payload = fbb.finished_data();
    let mut fbb2 = FlatBufferBuilder::new();
    let payload_vec = fbb2.create_vector(payload);
    let env_args = MessageEnvelopeArgs {
        version: 1,
        agent_id,
        message_type: MessageType::Perception,
        payload: Some(payload_vec),
    };
    let envelope = MessageEnvelope::create(&mut fbb2, &env_args);
    fbb2.finish(envelope, None);

    stream.write_all(fbb2.finished_data()).await.unwrap();
    if agent_id <= 10 {
        println!("🧠 Agent {} sent Perception", agent_id);
    }
    
}
