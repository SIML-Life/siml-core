//! src/bin/client.rs
use std::net::TcpStream;
use std::io::Write;
use siml_core::generated::handshake::{HandshakeArgs, Role, Handshake};
use flatbuffers::FlatBufferBuilder;

fn main() {
    // ── 1. build a Handshake FlatBuffer ────────────────────────────────
    let mut fbb = FlatBufferBuilder::new();

    // Anything you put here just has to satisfy the schema:
    let agent_type = fbb.create_string("TestClient");

    let hand = {
        let args = HandshakeArgs {
            agent_id: 42,
            role: Role::Agent,
            agent_type: Some(agent_type),
        };
        Handshake::create(&mut fbb, &args)
    };

    fbb.finish(hand, None);              // <-— root is `Handshake`
    let bytes = fbb.finished_data();     // &[u8]

    // ── 2. connect and shoot the bytes over ────────────────────────────
    let mut stream = TcpStream::connect("127.0.0.1:6000")
        .expect("cannot reach server");
    stream.write_all(bytes).unwrap();

    println!("✅ handshake sent ({} bytes)", bytes.len());
}
