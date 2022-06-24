use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;

#[allow(dead_code)]
pub fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[allow(dead_code)]
pub fn create_dir(dir: &Path) -> io::Result<()> {
    match fs::create_dir(dir) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn is_existed(path: &String) -> bool {
    Path::new(path).exists()
}

pub fn is_contract_existed(contract: String) -> bool {
    println!("contract: {}", contract);
    tracing::info!("add contract:  {}", contract);

    let contract_vec = contract.split(":").collect::<Vec<&str>>();
    is_existed(&contract_vec[0].into())
}
