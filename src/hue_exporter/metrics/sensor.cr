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
        metric_labels = "{device_type=#{hash["type"].as_s.inspect},name=#{hash["name"].as_s.inspect}}"

        hash["state"].each do |key, value|
          metric_name = [metric_name_base, "state_", key.as_s, metric_labels].join

          ok, metric_value = sanitise_value(value) do
            # special handling for temperature to get a float
            if key == "temperature"
              value.as_i64 / 100.0
            end
          end
          next unless ok

          metrics << [value.nil? ? "#" : "", metric_name, metric_value].join(" ").strip
        end

        hash["config"].each do |key, value|
          metric_name = [metric_name_base, "config_", key.as_s, metric_labels].join

          ok, metric_value = sanitise_value(value)
          next unless ok

          metrics << [value.nil? ? "#" : "", metric_name, metric_value].join(" ").strip
        end

        metrics
      end

      # overloading for blocks
      private def sanitise_value(value) : Tuple(Bool, Int64 | Int32 | Float64 | Float32)
        # block for special handling
        block_value = yield value
        return {true, block_value} if block_value
        sanitise_value(value)
      end

      # sanitises the JSON::Any value to be used for prometheus
      private def sanitise_value(value) : Tuple(Bool, Int64 | Int32 | Float64 | Float32)
        # no support for nil, array, and string values
        return {false, 0} if value.nil? || value.as_a? || value.as_s?
        return {true, value.as_bool ? 1 : 0} if !value.as_bool?.nil?
        return {true, value.as_f} if value.as_f?
        return {true, value.as_f32} if value.as_f32?
        {true, value.as_i64}
      end
    end
  end
end
