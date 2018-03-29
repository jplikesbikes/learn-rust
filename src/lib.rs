pub mod db {

	/// This is the create function
	/// It should print hello world and return 5
	/// ```
	/// use learn_rust::db;
	///
	/// let x = db::create();
	/// assert_eq!(5, x);
	/// ```
	pub fn create() -> i32 {
		println!("hello world");
		5
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn first_test() {
		assert!(true);
	}
}
