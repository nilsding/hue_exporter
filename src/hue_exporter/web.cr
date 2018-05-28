require "kemal"
require "./metrics/*"

module HueExporter
  module Web
    def self.sensor_metrics(client)
      metrics = ["# sensors"] of String
      ok, response = client.sensors
      unless ok
        metrics << "# got error number #{response} :-("
        return metrics.join("\n")
      end

      if response.is_a? Int32
        metrics << "# response = #{response} ???"
        return metrics.join("\n")
      end

      metrics += HueExporter::Metrics::Sensor.new.extract(response)

      metrics.join("\n")
    end
  end
end

before_get do |context|
  context.response.headers["Content-Type"] = "text/plain"
end

get "/" do |context|
  "hue_exporter #{HueExporter::VERSION} -- https://github.com/nilsding/hue_exporter\n"
end

get "/metrics" do |context|
  client = HueExporter::Application.client
  if client.nil?
    "# unknown\n"
  else
    [
      HueExporter::Web.sensor_metrics(client),
      "\n",
    ].join("\n")
  end
end
