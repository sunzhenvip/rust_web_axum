

fn main() {
    tonic_build::configure()
        .out_dir("src")
        .compile(&["proto/helloworld.proto"], &["proto"])
        .unwrap();
}
