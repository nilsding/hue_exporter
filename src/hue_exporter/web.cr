require "kemal"

module HueExporter
  module Web
  end
end

get "/" do |x|
  "hue_exporter #{HueExporter::VERSION} -- https://github.com/nilsding/hue_exporter"
end
