use whisp_runtime::environment::Environment;

#[test]
fn test_environment_creation() {
    let env = Environment::new();

    assert!(env.stack.len() == 1);
    assert!(env.stack[0].is_empty());
}
