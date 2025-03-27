use std::env;
use cfg::ConfigStruct;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_new_with_default_run_mode() {
        // Ensure RUN_MODE is not set
        env::remove_var("RUN_MODE");

        // Create a new Settings instance
        let settings = ConfigStruct::new();
        assert!(settings.is_ok());
        match settings {
            Ok(cfg) => {
                cfg.broker.host;
                assert_eq!(cfg.storage.segment_size, 1048576);
            },
            Err(e) => {
                eprintln!("error: {:?}", e);
            },
        }
    }
}
