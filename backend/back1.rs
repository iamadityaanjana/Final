#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fund() {
        let mut state = CrowdfundingESDTTestState::new();
        state.deploy();

        state.fund(FIRST_USER_ADDRESS_EXPR, "1000");
        state.check_deposit(state.first_user_address.clone(), 1_000);
    }

    #[test]
    fn test_status() {
        let mut state = CrowdfundingESDTTestState::new();
        state.deploy();

        state.check_status(Status::FundingPeriod);
    }

    #[test]
    fn test_sc_error() {
        let mut state = CrowdfundingESDTTestState::new();
        state.deploy();

        state.world.sc_call(
            ScCallStep::new()
                .from(FIRST_USER_ADDRESS_EXPR)
                .egld_value("1_000")
                .call(state.crowdfunding_esdt_contract.fund())
                .expect(TxExpect::user_error("str:wrong token")),
        );
        state.world.sc_query(
            ScQueryStep::new()
                .call(
                    state
                        .crowdfunding_esdt_contract
                        .deposit(&state.first_user_address),
                )
                .expect(TxExpect::ok().result("0x")),
        );
    }

    #[test]
    fn test_successful_cf() {
        let mut state = CrowdfundingESDTTestState::new();
        state.deploy();

        // first user fund
        state.fund(FIRST_USER_ADDRESS_EXPR, "1_000");
        state.check_deposit(state.first_user_address.clone(), 1_000);

        // second user fund
        state.fund(SECOND_USER_ADDRESS_EXPR, "1_000");
        state.check_deposit(state.second_user_address.clone(), 1_000);

        // set block timestamp after deadline
        state.set_block_timestamp(CF_DEADLINE + 1);

        // check status successful
        state.check_status(Status::Successful);

        // user try claim
        state.world.sc_call(
            ScCallStep::new()
                .from(FIRST_USER_ADDRESS_EXPR)
                .call(state.crowdfunding_esdt_contract.claim())
                .expect(TxExpect::user_error(
                    "str:only owner can claim successful funding",
                )),
        );

        // owner claim
        state.claim(OWNER_ADDRESS_EXPR);

        state.check_esdt_balance(OWNER_ADDRESS_EXPR, "2_000");
        state.check_esdt_balance(FIRST_USER_ADDRESS_EXPR, "0");
        state.check_esdt_balance(SECOND_USER_ADDRESS_EXPR, "0");
    }

    #[test]
    fn test_failed_cf() {
        let mut state = CrowdfundingESDTTestState::new();
        state.deploy();

        // first user fund
        state.fund(FIRST_USER_ADDRESS_EXPR, "300");
        state.check_deposit(state.first_user_address.clone(), 300u64);

        // second user fund
        state.fund(SECOND_USER_ADDRESS_EXPR, "600");
        state.check_deposit(state.second_user_address.clone(), 600u64);

        // set block timestamp after deadline
        state.set_block_timestamp(CF_DEADLINE + 1);

        // check status failed
        state.check_status(Status::Failed);

        // first user claim
        state.claim(FIRST_USER_ADDRESS_EXPR);

        // second user claim
        state.claim(SECOND_USER_ADDRESS_EXPR);

        state.check_esdt_balance(OWNER_ADDRESS_EXPR, "0");
        state.check_esdt_balance(FIRST_USER_ADDRESS_EXPR, "1_000");
        state.check_esdt_balance(SECOND_USER_ADDRESS_EXPR, "1_000");
    }
}
