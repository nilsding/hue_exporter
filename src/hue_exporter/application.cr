module HueExporter
  class Application
    private property config : HueExporter::Configuration

    def initialize
      @config = HueExporter::Configuration.load
    end

    def run
      unless config.valid?
        setup_flow
      end
    end

    private def setup_flow
      puts "First time setup, yay!"
    end
  end
end
