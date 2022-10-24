#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Order, Response, StdResult,
};
use cosmwasm_storage::{Bucket, ReadonlyBucket};

use crate::config::{Config, CONFIG};
use crate::error::ContractError;
use crate::msg::{CumulativePriceResponse, ExecuteMsg, InstantiateMsg, PriceResponse, QueryMsg};
use crate::twap::TWAP;

const PAIRS: &[u8] = b"pairs";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config::from(msg);
    config.validate(deps.api)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Run {} => {
            let config = CONFIG.load(deps.storage)?;
            let bucket: ReadonlyBucket<Addr> = ReadonlyBucket::new(deps.storage, PAIRS);
            for res in bucket.range(None, None, Order::Ascending) {
                match res {
                    Err(err) => return Err(err.into()),
                    Ok((_, addr)) => {
                        // TODO: Query FIN
                        let price = Decimal::zero();
                        TWAP::from(addr).update(
                            deps.storage,
                            env.block.time,
                            price,
                            config.max_age,
                        )?;
                    }
                }
            }
            Ok(Response::default())
        }
        ExecuteMsg::Register { addr } => todo!(),
        ExecuteMsg::UpdateConfig { owner, max_age } => todo!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Price {
            pair,
            timestamp,
            offset,
        } => {
            let (price, [(t0, v0), (t1, v1)]) =
                TWAP::from(pair).price(deps.storage, timestamp, offset)?;
            Ok(to_binary(&PriceResponse {
                price,
                raw: [
                    CumulativePriceResponse {
                        timestamp: t0,
                        value: v0,
                    },
                    CumulativePriceResponse {
                        timestamp: t1,
                        value: v1,
                    },
                ],
            })?)
        }
        QueryMsg::Pairs { offset } => todo!(),
    }
}

#[cfg(test)]
mod tests {}
