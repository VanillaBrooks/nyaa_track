fn parse(request_result: std::vec::Vec<u8>) {
	println!{"\n\nparse in \n\n{:?}", request_result}
	let s  = String::from_utf8_lossy(&request_result);
	println!{"\n\ns result: \n{:?}", s}
	// let s = match String::from_utf8_lossy(&request_result) {
	// 	Some(v) => v,
	// 	None => panic!("Invalid UTF-8 sequence:"),
	// };
}


fn async_request(url : &'static str) {
	// println!{"in async request"}

	let k = rt::run(rt::lazy( move || {

		// handles https
		let https = HttpsConnector::new(4).expect("TLS initialization failed");
		let client = Client::builder()
			.build::<_, hyper::Body>(https);


		let current_request = Request::get(url)
			.header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:64.0) Gecko/20100101 Firefox/64.0")
			.method("GET")
			.body(hyper::Body::empty())
			.unwrap();

		// println!{"built request: {:?}", current_request}

		client.request(current_request)
		// .map_err(|_| ())

		.and_then(|req| {
			// println!("\n\nResponse code ::::: {}\n\n", req.status());
			req.into_body().concat2()

		}) // and then
		.map(|body| {
			// println!{"\n\nbody {:?} \n\n", body}
			// let k:i32 = body; //hyper::body::chunk::Chunk
			// parse(body.to_vec());



		})

		.map_err(|_| ())

	}) // rt::lazy
	); // rt::run
	// let s : i32 = k;
	// println!{"K {:?}", k.wait()}

}
