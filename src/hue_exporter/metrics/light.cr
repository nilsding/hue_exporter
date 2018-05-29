require "./metric"

module HueExporter
  module Metrics
    class Light < Metric
      def extract(response)
        metrics = [] of String
        response.each do |id, hash|
          metrics += extract_metrics(id, hash)
        end
        metrics
      end

      def extract_metrics(id, hash)
        metrics = [] of String
        metric_name_base = "hue_light_"
        metric_labels = "{device_type=#{hash["type"].as_s.inspect},name=#{hash["name"].as_s.inspect}}"

        metrics += hash["state"].as_h.compact_map do |key, value|
          metric_name = [metric_name_base, "state_", key, metric_labels].join

          ok, metric_value = sanitise_value(value) do
            # special handling for temperature to get a float
            if key == "temperature" && value.is_a?(Int64)
              value / 100.0
            end
          end
          next unless ok

          [value.nil? ? "#" : "", metric_name, metric_value].join(" ").strip
        end

        metrics += hash["config"].as_h.compact_map do |key, value|
          metric_name = [metric_name_base, "config_", key, metric_labels].join

          ok, metric_value = sanitise_value(value)
          next unless ok

          [value.nil? ? "#" : "", metric_name, metric_value].join(" ").strip
        end

        metrics
      end

      # overloading for blocks
      private def sanitise_value(value) : Tuple(Bool, Int64 | Float64)
        # block for special handling
        block_value = yield value
        return {true, block_value} if block_value
        sanitise_value(value)
      end

      # sanitises the value to be used for prometheus
      private def sanitise_value(value) : Tuple(Bool, Int64 | Float64)
        return {true, value ? 1i64 : 0i64} if value.is_a?(Bool)
        return {true, value} if value.is_a?(Float64 | Int64)
        return {false, 0i64}
      end
    end
  end
end
