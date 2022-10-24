use cosmwasm_std::{Addr, Decimal, Order, StdResult, Storage, Timestamp, Uint128};
use cosmwasm_storage::{Bucket, ReadonlyBucket};

use crate::ContractError;

pub struct TWAP {
    addr: Addr,
}

impl From<Addr> for TWAP {
    fn from(addr: Addr) -> Self {
        Self { addr }
    }
}

impl TWAP {
    pub fn update(
        &mut self,
        storage: &mut dyn Storage,
        timestamp: Timestamp,
        price: Decimal,
        max_age: Timestamp,
    ) -> StdResult<()> {
        let bucket: Bucket<Decimal> = Bucket::new(storage, self.addr.as_bytes());
        match bucket.range(None, None, Order::Descending).next() {
            Some(Ok((k, entry))) => {
                let prev_seconds = u64::from_be_bytes(k.try_into().unwrap());
                let dt = timestamp.seconds() - prev_seconds;
                let val = entry + (price * Uint128::from(dt));
                bucket.save(&timestamp.seconds().to_be_bytes(), &val)
            }
            _ => bucket.save(&timestamp.seconds().to_be_bytes(), &Decimal::zero()),
        }
    }

    fn prune(&mut self, storage: &mut dyn Storage, max_age: Timestamp) {}

    pub fn price(
        &self,
        storage: &dyn Storage,
        timestamp: Timestamp,
        offset: Timestamp,
    ) -> Result<(Decimal, [(Timestamp, Decimal); 2]), ContractError> {
        let bucket: ReadonlyBucket<Decimal> = ReadonlyBucket::new(storage, self.addr.as_bytes());
        let (t1, v1): (Vec<u8>, Decimal) = bucket
            .range(
                Some(&timestamp.seconds().to_be_bytes()),
                None,
                Order::Descending,
            )
            .next()
            .ok_or(ContractError::NotFound {})??;

        let t1_s = u64::from_be_bytes(t1.try_into().unwrap());

        let offset = timestamp.seconds() - offset.seconds();

        let (t0, v0): (Vec<u8>, Decimal) = bucket
            .range(Some(&offset.to_be_bytes()), None, Order::Descending)
            .next()
            .ok_or(ContractError::NotFound {})??;

        let t0_s = u64::from_be_bytes(t0.try_into().unwrap());

        let price = Decimal::from_ratio(v1 - v0, Uint128::from(t1_s - t0_s));

        Ok((
            price,
            [
                (Timestamp::from_seconds(t0_s), v0),
                (Timestamp::from_seconds(t1_s), v1),
            ],
        ))
    }
}
