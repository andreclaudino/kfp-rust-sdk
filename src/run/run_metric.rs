mod run_metric_format;

struct RunMetric {
	name: String,
	node_id: String,
	number_value: f64,	
	format: RunMetricFormat
}