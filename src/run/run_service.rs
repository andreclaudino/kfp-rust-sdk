mod client;

use reqwest::{Error, Client};

pub struct RunService {
	pub host: String,
	
	client: reqwest::Client
}

pub impl RunService {

	pub fn new(host: String) -> Result<Run, CreateServiceErrors> {
		RunService{
			host: host,
			client: reqwest::Client::new()
		}
	}
	
	pub fn get_runs(self: &RunService) -> Result<Vec<Run>, Error>{
		self.client.get
	}

	pub fn get_run(self: &RunService) -> Result<Run, Error> {}
	
	pub fn create_run(self: &RunService, body: &Run) -> Result<(), Error>{}

	pub fn delete_run(self: &RunService) -> Result<(), Error> {}
}