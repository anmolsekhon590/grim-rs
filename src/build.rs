use wayland_scanner::{generate_c_interfaces, generate_interfaces, Side};

fn main() {
    let protocol_file = "src/protocols/wlr-screencopy-unstable-v1.xml";

    generate_interfaces(
        protocol_file,
        "src/protocols/generated.rs",
        Side::Client,
    );

    generate_c_interfaces(
        protocol_file,
        "src/protocols/generated_c.rs",
        Side::Client,
    );
}
