pub mod rsa {
	fn update_step(a: &mut i128, old_a: &mut i128, quotient: i128) {
		let temp = *a;
		*a = *old_a - quotient * temp;
		*old_a = temp;
	}
	pub fn extended_euclidean_algorithm(a: i128, b: i128) -> (i128, i128, i128) {
		let (mut old_r, mut rem) = (a, b);
		let (mut old_s, mut coeff_s) = (1, 0);
		let (mut old_t, mut coeff_t) = (0, 1);
	
		while rem != 0 {
			let quotient = old_r / rem;
	
			update_step(&mut rem, &mut old_r, quotient);
			update_step(&mut coeff_s, &mut old_s, quotient);
			update_step(&mut coeff_t, &mut old_t, quotient);
		}
	
		(old_r, old_s, old_t)
	}
	
    

	pub fn get_rsa_keypair(p:i128,q:i128) -> (KeyPrivate,KeyPublic) {
		let n = p*q;
		let r = (p-1)*(q-1);
		const e: i128 = 65537;

		
	}	
	pub struct KeyPrivate {
		pub decryptkey: i128
	}
	impl KeyPrivate {
		fn decrypt(&self) {

		}
	}
	pub struct KeyPublic {
		n:u128
	}
	impl KeyPublic {
		fn encrypt(&self) {
		
		}
	}
	#[test]
	fn test_rsa() {

	}

}