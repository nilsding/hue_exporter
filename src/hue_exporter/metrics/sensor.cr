module HueExporter
  module Metrics
    class Sensor < Metric
      def extract(response)
        metrics = [] of String
        response.each do |id, hash|
          metrics += extract_metrics(id, hash)
        end
        metrics
      end

      def extract_metrics(id, hash)
        metrics = [] of String
        metric_name_base = "hue_sensor_"
        metric_labels = "{device_type=#{hash["type"].as_s.downcase}, name=#{hash["name"].as_s.inspect}}"

        hash["state"].each do |key, value|
          next if key == "lastupdated"
          metric_name = [metric_name_base, "state_", key.as_s, metric_labels].join
          metric_value : Int64 | Int32 | Float64 | Float32 = 0
          # TODO: refactor this.
          unless value.nil?
            if key == "temperature"
              metric_value = value.as_i64 / 100.0
            elsif value.as_bool? && value.as_bool?.is_a?(Bool)
              metric_value = value.as_bool ? 1 : 0
            end
          end
          metrics << [metric_name, metric_value].join(" ")
        end

        metrics
      end
    end
  end
end
