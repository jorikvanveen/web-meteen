use std::{collections::HashMap, path::PathBuf};

use meteen_model::Database as MeteenVault;

type VaultIndex = usize;

pub struct Vaults {
    base_path: PathBuf,
    cache: HashMap<String, MeteenVault>,
}

impl Vaults {
    pub fn new(base_path: PathBuf) -> Vaults {
        dbg!(&base_path);
        Vaults {
            base_path: base_path.join("vaults"),
            cache: HashMap::new(),
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

    pub async fn get_vault(&mut self, id: &str) -> tokio::io::Result<&MeteenVault> {
        let is_cached = self.cache.contains_key(id);

        if is_cached {
            return Ok(self.cache.get(id).unwrap());
        }

        let vault = self.load_vault(id).await?;

        self.cache.insert(id.into(), vault);
        Ok(self.cache.get(id).unwrap())
    }

    pub async fn get_vault_mut() -> tokio::io::Result<&mut MeteenVault> {
        todo!()
    }
}
