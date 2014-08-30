fn main() {
	for x in range(0i,40i) {
		if (x % 15) == 0 {
			println!("fizzbuzz");
		} else if (x % 3) == 0 {
			println!("fizz");
		} else if (x % 5) == 0 {
			println!("buzz");
		} else {
			println!("{:d}", x);
		}
	}
}
