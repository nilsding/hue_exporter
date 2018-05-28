require "yaml"

module HueExporter
  class Configuration
    YAML.mapping(
      username: String,
      hue_url: {
        type:    String,
        nilable: false,
        default: "http://philips-hue.local",
      }
    )

    def self.load(config_path = default_config_path)
      create_default_config(config_path) unless File.exists?(config_path)
      from_yaml(File.read(config_path))
    end

    def self.default_config_path
      File.expand_path("~/#{CONFIG_FILE_NAME}")
    end

    private def self.create_default_config(config_path)
      puts "Creating empty config at #{config_path.inspect}"
      File.open(config_path, "w") do |f|
        YAML.build(f) do |yaml|
          yaml.mapping do
            yaml.scalar "hue_url"
            yaml.scalar "http://philips-hue.local"
            yaml.scalar "username"
            yaml.scalar UNSET_STRING
          end
        end
      end
    end

    def valid?
      username != UNSET_STRING
    end
  end
end
