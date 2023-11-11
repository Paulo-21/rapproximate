#[inline(always)]
fn solve() {
    let in_degree = af.inDegree(argument);
	let out_degree = af.outDegree(argument);
	let res = out_degree >= threshold * in_degree;
}

