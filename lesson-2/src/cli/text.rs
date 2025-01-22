use clap::{command, Parser};

#[derive(Debug, Parser)]
pub enum TextSubcommands {
    #[command(about = "Sign a text with a private/session key and return a signature")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signature with a public/session key")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a random blake3 key or ed25519 key pair")]
    Generate(KeyGenerateOpts),
}

#[derive(Debug, Parser)]
struct  TextSignOpts {

}



#[derive(Debug, Parser)]
struct  TextVerifyOpts {

}





#[derive(Debug, Parser)]
struct  KeyGenerateOpts {

}
