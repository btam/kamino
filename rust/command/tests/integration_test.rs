use command::{command, CommandArgs};

#[test]
#[should_panic]
fn test_constant_backoff() {
    command!(
        ["cat", "dummy"],
        CommandArgs {
            retries: 1,
            ..Default::default()
        }
    )
    .expect("Should panic");
}
