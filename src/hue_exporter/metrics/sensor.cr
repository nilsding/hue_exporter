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
          metric_value : Int64 | Int32 | Float64 | Float32 = 0

          # TODO: refactor this.
          unless value.nil?
            if key == "temperature"
              # special handling for temperature to get a float
              metric_value = value.as_i64 / 100.0
            elsif !value.as_bool?.nil?
              metric_value = value.as_bool ? 1 : 0
            elsif value.as_s?
              next # no support for string values
            else
              metric_value = value.as_i64
            end
          end
          metrics << [value.nil? ? "#" : "", metric_name, metric_value].join(" ").strip
        end

        hash["config"].each do |key, value|
          metric_name = [metric_name_base, "config_", key.as_s, metric_labels].join
          metric_value : Int64 | Int32 | Float64 | Float32 = 0
          unless value.nil?
            if value.as_a?
              next # no support for array values
            elsif value.as_s?
              next # no support for string values
            elsif !value.as_bool?.nil?
              metric_value = value.as_bool ? 1 : 0
            else
              metric_value = value.as_i64
            end
          end
          metrics << [value.nil? ? "#" : "", metric_name, metric_value].join(" ").strip
        end

        metrics
      end
    end
  end
end
