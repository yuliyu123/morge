use std::path::Path;

pub fn is_existed(path: &String) -> bool {
    Path::new(path).exists()
}

pub fn is_contract_existed(contract: String) -> bool {
    let contract_vec = contract.split(":").collect::<Vec<&str>>();
    is_existed(&contract_vec[0].into())
}
