use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Api, StdResult, Timestamp};
use cw_storage_plus::Item;

use crate::msg::InstantiateMsg;
pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub max_age: Timestamp,
}

impl From<InstantiateMsg> for Config {
    fn from(msg: InstantiateMsg) -> Self {
        Self {
            owner: msg.owner,
            max_age: msg.max_age,
        }
    }
}

impl Config {
    pub fn validate(&self, api: &dyn Api) -> StdResult<()> {
        api.addr_validate(self.owner.as_str())?;
        Ok(())
    }
}
