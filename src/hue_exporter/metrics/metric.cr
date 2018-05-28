module HueExporter
  module Metrics
    abstract class Metric
      abstract def extract(response)
    end
  end
end
