use clap::{Parser, Subcommand};
use md5::Digest;
use std::io::Read;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Arhidia
#[derive(Debug, Parser)]
#[command(name = "arhidia", version, author, about = "Quick'n'dirty MD5 file hasher", long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: CliCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum CliCommands {
    /// Hash files
    Hash {
        /// Files to hash
        // #[arg(required)]
        files: Vec<PathBuf>,
        /// Hash every file till the specified depth in 4k blocks
        #[arg(short, long, default_value = "0")]
        blocks: usize,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        CliCommands::Hash { files, blocks } => {
            println!(
                "Hashing files: {:?} with depth: {} 4K blocks",
                files, blocks
            );
            let mut allfiles = Vec::new();
            for dirs in files {
                for entry in WalkDir::new(dirs)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_file())
                {
                    allfiles.push(entry.path().to_path_buf());
                }
            }
            for filename in allfiles {
                let metadata = std::fs::metadata(&filename).unwrap();
                let size = metadata.len();
                let file = std::fs::File::open(&filename).unwrap();

                let mut reader = std::io::BufReader::new(file);
                let mut hasher = md5::Md5::new();
                let mut buffer = [0; 4096];
                let mut read_blocks = 0;
                let mut read_bytes: u64 = 0;

                // Compute md5 hash of file using streaming reader
                loop {
                    let n = reader.read(&mut buffer).unwrap();
                    if (n == 0) || (blocks > 0 && read_blocks >= blocks) {
                        break;
                    }
                    hasher.update(&buffer[..n]);
                    read_blocks += 1;
                    read_bytes += n as u64;
                }
                let hash = hasher.finalize();
                println!("{:?} {:x} {:?} {:?}", size, hash, read_bytes, filename);
            }
        }
    }
}
