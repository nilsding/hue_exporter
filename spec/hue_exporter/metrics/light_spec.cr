require "../../spec_helper"

describe HueExporter::Metrics::Light do
  it "can be instantiated" do
    HueExporter::Metrics::Light.new
  end

  describe "#extract" do
    instance = HueExporter::Metrics::Light.new
    lights_reply = Fixtures.load_json("hue_bridge_lights_reply")

    it "returns the expected metrics" do
      expected_metrics = Fixtures.load_string_array("lights_metrics.txt")
      instance.extract(lights_reply).should eq expected_metrics
    end
  end
end
