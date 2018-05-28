require "http/client"
require "http/headers"
require "json"

module HueExporter
  class HueClient
    private property config : HueExporter::Configuration
    private property headers : HTTP::Headers?

    def initialize(config)
      @config = config
    end

    # Authorize with the Hue bridge
    def authorize
      return {true, :already_authorized} unless config.username == UNSET_STRING
      response = JSON.parse(post("/api", {"devicetype" => "hue_exporter"}).body)
      if response.first["error"]?
        case response.first["error"]["type"].as_i
        when 101
          return {false, :link_button}
        else
          return {false, :unknown}
        end
      elsif response.first["success"]?
        config.username = response.first["success"]["username"].as_s
      else
        return {false, :unknown}
      end

      {true, :success}
    end

    private def headers
      @headers ||= HTTP::Headers.new.tap do |headers|
        headers["Content-Type"] = "application/json"
      end
    end

    private def post(endpoint, body : Hash)
      HTTP::Client.post(File.join(config.hue_url, endpoint), headers, body.to_json)
    end
  end
end
