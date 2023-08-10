use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CosmosMsg;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Execute { msg: CosmosMsg },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
