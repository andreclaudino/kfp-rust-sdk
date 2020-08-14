struct PipelineSpec {
	pub pipeline_id: String,
	pub pipeline_name: String,
	pub workflow_manifest: String,
	pub pipeline_manifest: String,
	pub parameters: Vec<ApiParameter>
}