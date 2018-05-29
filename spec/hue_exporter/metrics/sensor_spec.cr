require "../../spec_helper"

describe HueExporter::Metrics::Sensor do
  it "can be instantiated" do
    HueExporter::Metrics::Sensor.new
  end

  describe "#extract" do
    instance = HueExporter::Metrics::Sensor.new
    sensors_reply = Fixtures.load_json("hue_bridge_sensors_reply")

    it "returns the expected metrics" do
      expected_metrics = Fixtures.load_string_array("sensors_metrics.txt")
      instance.extract(sensors_reply).should eq expected_metrics
    end
  end
end
