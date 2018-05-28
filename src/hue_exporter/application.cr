require "kemal"
require "./web"

module HueExporter
  class Application
    private property config : HueExporter::Configuration
    private property client : HueExporter::HueClient

    def initialize
      @config = HueExporter::Configuration.load
      @@client = @client = HueExporter::HueClient.new(config)
    end

    def self.client
      @@client
    end

    def run
      puts "hue_exporter #{VERSION}"
      unless config.valid?
        puts "Starting first time setup.  This has to be done only once!"
        setup_flow
      end
      puts "Starting web server on port #{HTTP_PORT}..."
      Kemal.run HTTP_PORT
    end

    private def setup_flow
      ok, status = client.authorize
      if !ok
        case status
        when :link_button
          puts "Please push the link button on your Hue bridge and press return."
          STDIN.gets
          return setup_flow
        when :unknown
          puts "Oopsie woopsie uwu"
          exit 1
        end
      end

      config.persist
      puts "Setup complete."
    end
  end
end
