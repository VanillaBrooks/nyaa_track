use std::time::Instant;

use reqwest as request;

// for holding time data
struct Timing{
	total_rt : f64,
	total_runs: u32,
}

impl Timing{
	fn new() -> Timing{
		return Timing{total_rt: 0.0, total_runs: 0}
	}
	fn avg(&self) -> f64{
		if self.total_runs > 0{
			return self.total_rt / (self.total_runs as f64)
		}
		else{
			return 0.0
		}
	}
	fn add(&mut self, start_time: &Instant, end_time: &Instant) {
		let elapsed_time = end_time.duration_since(*start_time);
		self.total_rt += elapsed_time.as_secs() as f64 + (elapsed_time.subsec_nanos() as f64 * 1e-9);
		self.total_runs += 1;

	}
}

// organizing good and bad urls that are being dealt with
struct  GetResults {
	html_data: Vec< String>,
	scraped_urls: Vec<String>,
	failed_urls: Vec< String>,
}

impl GetResults{
	fn new() -> GetResults{
		return GetResults{html_data:Vec::new(), scraped_urls: Vec::new(), failed_urls: Vec::new()};
	}
	fn good_call (&mut self, new_data: String, url_used: String) {
		self.html_data.push(new_data);
		self.scraped_urls.push(url_used)
	}
	fn bad_call(&mut self, bad_url: String) {
		self.failed_urls.push(bad_url);
	}
}

struct UpdateFrequency{
	url: Vec<String>,
	next_update: Vec<std::time::Duration>,
	metadata: Vec<TorrentData>,
}

impl UpdateFrequency{
	fn new() -> UpdateFrequency{
		return UpdateFrequency {url: Vec::new(), next_update: Vec::new(), metadata: Vec::new()}
	}

	fn add_pair(&mut self, new_url: String, update: std::time::Duration){
		self.url.push(new_url);
		self.next_update.push(update);
	}

	fn add_torrent_data(&mut self, seed: u32, leech: u32, snatch: u32, title:String){
		self.metadata.push(TorrentData{seeders:seed, leechers:leech, snatches:snatch, name:title})
	}
}

struct TorrentData {
	seeders: u32,
	leechers: u32,
	snatches: u32,
	name: String,
}


fn sync_request (urls : Vec<&str>, client: &request::Client , mut results_container: GetResults) -> GetResults {

	for url in urls{
		let body = client.get(url).send();

		match body.ok() {
			Some(mut x) => {
				match x.text(){
					Ok(data) => results_container.good_call(data, url.to_string()),
					Err(_)   => results_container.bad_call(url.to_string())
				}
			},
			None =>{
				results_container.bad_call(url.to_string())
			},
		}
	}
	return results_container

}

fn parse_html <'a>(html_vector: &Vec<String>, url_vector: &'a Vec<String>, prev_counts: UpdateFrequency) -> UpdateFrequency{

	for index in 0..html_vector.len(){
		let currnet_html = &html_vector[index];
		let current_url = &url_vector[index];


		// scrape the data here
	}


	return UpdateFrequency::new() // placehodler
}


fn main() {
	let avg = Timing::new();
	let client = request::Client::new();
	let mut results_container = GetResults::new();

	let urls_to_check = vec!["https://nyaa.pantsu.cat/", "https://nyaa.si"];
	// get magnet link data
	let results_struct = sync_request(urls_to_check, &client, results_container);

	let previous_frequencies = UpdateFrequency::new();
	let previous_frequencies = parse_html(&results_struct.html_data , &results_struct.scraped_urls, previous_frequencies);

	// cycle through the older torrents and find which ones need to be scraped again

}
