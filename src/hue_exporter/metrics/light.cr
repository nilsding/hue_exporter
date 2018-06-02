require "./metric"

module HueExporter
  module Metrics
    class Light < Metric
      private def metric_name_base
        "hue_light_"
      end

      private def extract_metrics(id, hash)
        metrics = [] of String
        metric_labels = metric_labels_for(hash)

        metrics += extract_metrics_from_hash(hash, "state") do |key, value|
          # special handling for temperature to get a float
          if key == "temperature" && value.is_a? Int64
            value / 100.0
          end
        end

        metrics += extract_metrics_from_hash(hash, "config")

        metrics
      end
    end
  end
end
