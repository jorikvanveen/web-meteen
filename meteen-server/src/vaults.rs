use std::{collections::HashMap, path::PathBuf};

use meteen_model::Database as MeteenVault;

type VaultIndex = usize;

pub struct Vaults {
    base_path: PathBuf,
    vaults: Vec<MeteenVault>,
    id_map: HashMap<String, VaultIndex>,
}

impl Vaults {
    pub fn new(base_path: PathBuf) -> Vaults {
        Vaults {
            base_path,
            vaults: vec![],
            id_map: HashMap::new(),
        }
    }
    async fn load_vault(id: String) -> MeteenVault {
        todo!()
    }

    async fn save_vault(&self, id: String, vault: &MeteenVault) -> tokio::io::Result<()> {
        let vault_path = self.base_path.join(format!("/{id}.mtvault"));

        todo!()
    }

    pub async fn get_vault(&self, id: String) -> &MeteenVault {
        match self.id_map.get(&id) {
            Some(vault_index) => return self.vaults.get(*vault_index).unwrap(),
            None => {}
        }

        todo!()
    }
}
