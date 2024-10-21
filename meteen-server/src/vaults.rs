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
        dbg!(&base_path);
        Vaults {
            base_path: base_path.join("vaults"),
            vaults: vec![],
            id_map: HashMap::new(),
        }
    }

    async fn load_vault(&self, id: &str) -> tokio::io::Result<MeteenVault> {
        let vault_path = self.base_path.join(format!("{id}.mtvault"));
        let serialized: Vec<u8> = tokio::fs::read(vault_path).await?;
        let vault: MeteenVault = bincode::deserialize(&serialized)
            .map_err(|_| tokio::io::Error::new(tokio::io::ErrorKind::Other, "Unreadable vault"))?;
        Ok(vault)
    }

    pub async fn save_vault(&self, id: &str, vault: &MeteenVault) -> tokio::io::Result<()> {
        let vault_path = self.base_path.join(format!("{id}.mtvault"));
        dbg!(&vault_path);
        let serialized = bincode::serialize(vault).map_err(|_| {
            tokio::io::Error::new(tokio::io::ErrorKind::Other, "Unserializable vault")
        })?;
        tokio::fs::write(vault_path, serialized).await
    }

    pub async fn delete_vault(&self, id: &str) -> tokio::io::Result<()> {
        let vault_path = self.base_path.join(format!("{id}.mtvault"));
        tokio::fs::remove_file(vault_path).await
    }

    pub async fn get_vault(&self, id: &str) -> &MeteenVault {
        match self.id_map.get(id) {
            Some(vault_index) => return self.vaults.get(*vault_index).unwrap(),
            None => {}
        }

        todo!()
    }
}
