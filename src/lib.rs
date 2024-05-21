pub mod rsa {
	fn encrypt(public_key:KeyPublic) {
		
	}
	pub struct KeyPrivate {
		p:u128,
		q:u128
	}
	impl KeyPrivate {
		pub fn new(p:u128,q:u128) -> Self {
			KeyPrivate {p,q}
		}
		pub fn publickey(&self) -> KeyPublic {
			KeyPublic { n: self.p*self.q }
		}
	}
	pub struct KeyPublic {
		n:u128
	}

}