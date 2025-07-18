use crate::models::{constants::PREFIX_TOKEN, Token};

pub fn list_tokens(db: &sled::Db) -> sled::Result<()> {
    let mut tokenlist = vec![];
    for entry in db.scan_prefix(format!("{}/", PREFIX_TOKEN)) {
        let (_, val) = entry?;
        let token: Token =
            serde_json::from_slice(&val).map_err(|e| sled::Error::ReportableBug(e.to_string()))?;
        tokenlist.push((token.name, token.symbol));
    }

    for (name, symbol) in tokenlist {
        println!("{} ({})", name, symbol);
    }

    Ok(())
}
