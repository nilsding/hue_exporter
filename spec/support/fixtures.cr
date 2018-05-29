require "json"

module Fixtures
  BASE_PATH = File.expand_path("../fixtures", __DIR__)

  def self.load(file_name)
    File.read(File.join(BASE_PATH, file_name))
  end

  def self.load_json(file_name)
    JSON.parse(load(File.basename(file_name, ".json") + ".json"))
  end

  def self.load_string_array(file_name)
    load(file_name).strip.split("\n")
  end
end
