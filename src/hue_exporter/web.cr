require "kemal"
require "./metrics/*"

module HueExporter
  module Web
    macro metric_methods(*types)
      {% for type in types %}
        def self.{{ type }}_metrics(client)
          metrics = ["# {{ type }}s"] of String
          ok, response = client.{{ type }}s
          unless ok
            metrics << "# got error number #{response} :-("
            return metrics.join("\n")
          end

          if response.is_a? Int32
            metrics << "# response = #{response} ???"
            return metrics.join("\n")
          end

          metrics += HueExporter::Metrics::{{ type.id.capitalize }}.new.extract(response)

          metrics.join("\n")
        end
      {% end %}
    end

    metric_methods sensor, light
  end
end

before_get do |context|
  context.response.headers["Content-Type"] = "text/plain; charset=utf-8"
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
      "",
      HueExporter::Web.light_metrics(client),
      "\n",
    ].join("\n")
  end
end
