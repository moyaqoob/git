
use std::{fmt::format, fs};
use anywhere::Context;
use clap::{Parser, Subcommand,command};
use std::io::{BufReader, prelude::*};
use std::io;
use flate2::read::ZlibDecoder;


#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug,Subcommand)]
enum Command {
    Init,
    CatFile {
        #[clap(short='p')]
        pretty_print:bool,
        object_hash:String,
    },
}

fn main(){
    let args= Args::parse();

    println!("Logs from your program will appear here!");

    match args.command{
         Command::Init => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
         }
         Command::CatFile {pretty_print,object_hash}=>{
            let mut f = std::fs::File::open(format!(
                ".git/objects/{}/{}",
                &object_hash[2..],
                &object_hash[..2]
            )).context("open in .git/objects");
            let mut z = ZlibDecoder::new(f);
            let mut z = BufReader::new(z);
            z.read_until(0,&mut buf)
                    .context("read header from .git/objects")


         }
    }
}