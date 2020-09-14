pub fn weighted_random<T: Copy>(list: Vec<(T, f64)>, random: f64) -> T {
	let mut total = 0.0;
	for (_, weight) in list.iter() {
		total += weight;
	}
	if !(total > 0.0) {
		return list[0].0;
	}
	let result = total * random;
	let mut total = 0.0;
	for (id, weight) in list.iter() {
		total += weight;
		if total > result {
			return *id;
		}
	}
	panic!("This shouldn't happen unless x * (1.0 - epsilon) == x, which I don't think should be possible, but haven't been able to find a source for that");
}
