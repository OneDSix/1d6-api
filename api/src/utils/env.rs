//! Based off https://github.com/modrinth/labrinth/blob/master/src/lib.rs

use log::warn;

use crate::AppState;

#[macro_export]
macro_rules! validEnvString {
	($state: expr, $vector: expr, $name: expr) => {
		{
			if check_var::<String>(&$state, &$name) {
				$vector.push($name);
			}
		}
	};
}

#[macro_export]
macro_rules! validEnvU32 {
	($state: expr, $vector: expr, $name: expr) => {
		{
			if check_var::<u32>(&$state, &$name) {
				$vector.push($name);
			}
		}
	};
}

pub fn check_env_vars(state: &AppState) -> Result<(), Vec<&str>> {
    let mut missing_vector = Vec::new();

	/// Checks if a env-var exists, otherwise
    fn check_var<T: std::str::FromStr>(state: &AppState, var: &'static str) -> bool {
        let check = state.secrets.get(var).is_none();
        if check {
            warn!(
                "Variable `{}` missing in dotenv or not of type `{}`",
                var,
                std::any::type_name::<T>()
            );
        }
        check
    }

	validEnvString!(&state, missing_vector, "ANALYTICS_KEY");

	if missing_vector.is_empty() {
		Ok(())
	} else {
		Err(missing_vector)
	}

}

