use chrono::DateTime;
use reqwest::{Error};


pub struct Run {
	
	pub id: Option(String),
	name: Option(String),
	description: Option(String),
	service_accoount: Option(String),
	status: Option(String),
	error: Option(String),
	
	created_at: DateTime,
	scheduled_at: DateTime,
	finished_at: DateTime,
	
	storage_state: RunStorageState,
	pipeline_spec: PipelineSpec,
	resource_references: Vec<ResourceReference>,
	metrics: Vec<RunMetric>
}

pub impl Run {
	pub fn archive(self: &Run) -> Result<ApiStatus, Error> {}
	pub fn restore(self: &Run) -> Result<ApiStatus, Error> {}
	pub fn retry(self: &Run) -> Result<ApiStatus, Error> {}
	pub fn terminate(self: &Run) -> Result<ApiStatus, Error> {}
	pub fn report_metrics(self: &Run) -> Result<ReportRunMetricsResponse, Error> {}
	pub fn get_artifacts(self: &Run, node_id: String, artifact_name: String) -> Resultt<ReadArtifactResponse, Error> {}
}