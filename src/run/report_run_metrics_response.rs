pub type ReportRunMetricsResponse = Vec<ReportRunMetricsResponseReportRunMetricResult>;

pub struct ReportRunMetricsResponseReportRunMetricResult {
	pub metric_name: String,
	pub metric_node_id: String,
	message: String,
	status: ReportRunMetricsResponseReportRunMetricResultStatus
}

pub enum ReportRunMetricsResponseReportRunMetricResultStatus {
	UNSPECIFIED="UNSPECIFIED",
	OK="OK",
	INVALID_ARGUMENT="INVALID_ARGUMENT",
	DUPLICATE_REPORTING="DUPLICATE_REPORTING",
	INTERNAL_ERROR="INTERNAL_ERROR"
}