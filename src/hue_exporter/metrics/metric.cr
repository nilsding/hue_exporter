require "json"

module HueExporter
  module Metrics
    abstract class Metric
      def extract(response)
        metrics = [] of String
        response.each do |id, hash|
          metrics += extract_metrics(id, hash)
        end
        metrics
      end

      private abstract def metric_name_base

      private abstract def extract_metrics(id, hash)

      private def metric_labels_for(hash)
        "{device_type=#{hash["type"].as_s.inspect},name=#{hash["name"].as_s.inspect}}"
      end

      private def extract_metrics_from_hash(hash, name, &block : (String, Array(JSON::Type) | Bool | Float64 | Hash(String, JSON::Type) | Int64 | String | Nil) -> _)
        metric_labels = metric_labels_for(hash)
        hash[name].as_h.compact_map do |key, value|
          metric_name = [metric_name_base, "#{name}_", key, metric_labels].join

          ok, metric_value = sanitise_value(value) do
            block.call(key, value)
          end
          next unless ok

          [value.nil? ? "#" : "", metric_name, metric_value].join(" ").strip
        end
      end

      private def extract_metrics_from_hash(hash, name)
        extract_metrics_from_hash(hash, name) { }
      end

      # sanitises the value to be used for prometheus.
      private def sanitise_value(value) : Tuple(Bool, Int64 | Float64)
        return {true, value ? 1i64 : 0i64} if value.is_a?(Bool)
        return {true, value} if value.is_a?(Float64 | Int64)
        return {false, 0i64}
      end

      # sanitises the value to be used for prometheus.
      # accepts a block to handle special values
      private def sanitise_value(value) : Tuple(Bool, Int64 | Float64)
        block_value = yield value
        return {true, block_value} if block_value
        sanitise_value(value)
      end
    end
  end
end
