
lazy_static::lazy_static! {
	static ref ALLOWED_CDNS: Vec<String> = vec![
		"example.com".to_string(),
		"rust-lang.org".to_string(),
		"docs.rs".to_string()
	];
}

pub fn check_url() {
	for url in ALLOWED_CDNS.iter() {

	}
}



