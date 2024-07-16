pub fn foo(env: Env) {
    foobarit(
        &env,
        &env.current_contract_address(),
        &ClaimableBalance { claimants },
    );
    println!(&env);
}
