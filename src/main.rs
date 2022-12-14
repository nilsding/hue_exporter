use actix_web::{
    get,
    http::header,
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use anyhow::Result;
use lazy_static::lazy_static;
use prometheus::{Encoder, GaugeVec, IntGaugeVec, TextEncoder};
use std::env;

mod hue_client;
use hue_client::HueClient;

#[macro_use]
extern crate log;

macro_rules! register_sensor_gauge_vec {
    ( $name:expr ) => {
        prometheus::register_gauge_vec!(
            concat!("hue_sensors_", $name),
            concat!("Hue sensor value ", $name),
            &["name", "device_type"]
        )
    };
}
macro_rules! register_sensor_int_gauge_vec {
    ( $name:expr ) => {
        prometheus::register_int_gauge_vec!(
            concat!("hue_sensors_", $name),
            concat!("Hue sensor value ", $name),
            &["name", "device_type"]
        )
    };
}
macro_rules! register_light_int_gauge_vec {
    ( $name:expr ) => {
        prometheus::register_int_gauge_vec!(
            concat!("hue_lights_", $name),
            concat!("Hue light value ", $name),
            &["name", "device_type"]
        )
    };
}

lazy_static! {
    // sensor config metrics -----------------------------------------
    static ref GAUGE_SENSORS_CONFIG_ON: IntGaugeVec = register_sensor_int_gauge_vec!("config_on").unwrap();
    static ref GAUGE_SENSORS_CONFIG_CONFIGURED: IntGaugeVec = register_sensor_int_gauge_vec!("config_configured").unwrap();
    static ref GAUGE_SENSORS_CONFIG_SUNRISEOFFSET: IntGaugeVec = register_sensor_int_gauge_vec!("config_sunriseoffset").unwrap();
    static ref GAUGE_SENSORS_CONFIG_SUNSETOFFSET: IntGaugeVec = register_sensor_int_gauge_vec!("config_sunsetoffset").unwrap();
    static ref GAUGE_SENSORS_CONFIG_REACHABLE: IntGaugeVec = register_sensor_int_gauge_vec!("config_reachable").unwrap();
    static ref GAUGE_SENSORS_CONFIG_BATTERY: IntGaugeVec = register_sensor_int_gauge_vec!("config_battery").unwrap();
    // TODO: string value // static ref GAUGE_SENSORS_CONFIG_ALERT: IntGaugeVec = register_sensor_int_gauge_vec!("config_alert").unwrap();
    static ref GAUGE_SENSORS_CONFIG_LEDINDICATION: IntGaugeVec = register_sensor_int_gauge_vec!("config_ledindication").unwrap();
    static ref GAUGE_SENSORS_CONFIG_USERTEST: IntGaugeVec = register_sensor_int_gauge_vec!("config_usertest").unwrap();
    static ref GAUGE_SENSORS_CONFIG_SENSITIVITY: IntGaugeVec = register_sensor_int_gauge_vec!("config_sensitivity").unwrap();
    static ref GAUGE_SENSORS_CONFIG_SENSITIVITYMAX: IntGaugeVec = register_sensor_int_gauge_vec!("config_sensitivitymax").unwrap();
    static ref GAUGE_SENSORS_CONFIG_THOLDDARK: IntGaugeVec = register_sensor_int_gauge_vec!("config_tholddark").unwrap();
    static ref GAUGE_SENSORS_CONFIG_THOLDOFFSET: IntGaugeVec = register_sensor_int_gauge_vec!("config_tholdoffset").unwrap();

    // sensor state metrics ------------------------------------------
    // TODO: string value // static ref GAUGE_SENSORS_STATE_LASTUPDATED: IntGaugeVec = register_sensor_int_gauge_vec!("state_lastupdated").unwrap();
    static ref GAUGE_SENSORS_STATE_DAYLIGHT: IntGaugeVec = register_sensor_int_gauge_vec!("state_daylight").unwrap();
    static ref GAUGE_SENSORS_STATE_FLAG: IntGaugeVec = register_sensor_int_gauge_vec!("state_flag").unwrap();
    static ref GAUGE_SENSORS_STATE_PRESENCE: IntGaugeVec = register_sensor_int_gauge_vec!("state_presence").unwrap();
    static ref GAUGE_SENSORS_STATE_DARK: IntGaugeVec = register_sensor_int_gauge_vec!("state_dark").unwrap();
    static ref GAUGE_SENSORS_STATE_STATUS: IntGaugeVec = register_sensor_int_gauge_vec!("state_status").unwrap();
    static ref GAUGE_SENSORS_STATE_BUTTONEVENT: IntGaugeVec = register_sensor_int_gauge_vec!("state_buttonevent").unwrap();
    static ref GAUGE_SENSORS_STATE_TEMPERATURE: GaugeVec = register_sensor_gauge_vec!("state_temperature").unwrap();
    static ref GAUGE_SENSORS_STATE_LIGHTLEVEL: IntGaugeVec = register_sensor_int_gauge_vec!("state_lightlevel").unwrap();

    // light state metrics -------------------------------------------
    static ref GAUGE_LIGHTS_STATE_REACHABLE: IntGaugeVec = register_light_int_gauge_vec!("state_reachable").unwrap();
    static ref GAUGE_LIGHTS_STATE_ON: IntGaugeVec = register_light_int_gauge_vec!("state_on").unwrap();
    static ref GAUGE_LIGHTS_STATE_BRI: IntGaugeVec = register_light_int_gauge_vec!("state_bri").unwrap();
    static ref GAUGE_LIGHTS_STATE_HUE: IntGaugeVec = register_light_int_gauge_vec!("state_hue").unwrap();
    static ref GAUGE_LIGHTS_STATE_SAT: IntGaugeVec = register_light_int_gauge_vec!("state_sat").unwrap();
    static ref GAUGE_LIGHTS_STATE_CT: IntGaugeVec = register_light_int_gauge_vec!("state_ct").unwrap();
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
    .insert_header((header::CONTENT_TYPE, "text/html; charset=utf-8"))
    .body(
        format!(
            "<h1>{} {}</h1>\n<ul>\n<li><a href='/metrics'>Metrics</a></li>\n<li><a href=\"{}\">Source code</a></li>\n</ul>\n",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_HOMEPAGE"),
        )
    )
}

#[get("/metrics")]
async fn metrics(hue_client: web::Data<HueClient>) -> impl Responder {
    if let Err(e) = fetch_metrics(&hue_client).await {
        error!("error while fetching metrics: {:?}", e);
        return HttpResponse::InternalServerError().finish();
    }

    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "text/plain; charset=utf-8"))
        .body(String::from_utf8(buffer).unwrap())
}

async fn fetch_metrics(hue_client: &HueClient) -> Result<()> {
    macro_rules! report_metric {
        ( $sensor:ident, $gauge_name:ident, $val:expr ) => {
            $gauge_name
                .with_label_values(&[&$sensor.name, &$sensor.r#type])
                .set($val.into());
        };
    }

    macro_rules! report_optional_metric {
        ( $sensor:ident, $gauge_name:ident, $val:expr ) => {
            if let Some(a) = $val {
                $gauge_name
                    .with_label_values(&[&$sensor.name, &$sensor.r#type])
                    .set(a.into());
            }
        };
    }

    let sensors = hue_client.sensors().await?;
    let lights = hue_client.lights().await?;

    for sensor in sensors.values() {
        // config metrics ------------------------------------------------
        report_metric!(sensor, GAUGE_SENSORS_CONFIG_ON, sensor.config.on);
        report_optional_metric!(
            sensor,
            GAUGE_SENSORS_CONFIG_CONFIGURED,
            sensor.config.configured
        );
        report_optional_metric!(
            sensor,
            GAUGE_SENSORS_CONFIG_SUNRISEOFFSET,
            sensor.config.sunriseoffset
        );
        report_optional_metric!(
            sensor,
            GAUGE_SENSORS_CONFIG_SUNSETOFFSET,
            sensor.config.sunsetoffset
        );
        report_optional_metric!(
            sensor,
            GAUGE_SENSORS_CONFIG_REACHABLE,
            sensor.config.reachable
        );
        report_optional_metric!(sensor, GAUGE_SENSORS_CONFIG_BATTERY, sensor.config.battery);
        // TODO: string value // report_optional_metric!(sensor, GAUGE_SENSORS_CONFIG_ALERT, sensor.config.alert);
        report_optional_metric!(
            sensor,
            GAUGE_SENSORS_CONFIG_LEDINDICATION,
            sensor.config.ledindication
        );
        report_optional_metric!(
            sensor,
            GAUGE_SENSORS_CONFIG_USERTEST,
            sensor.config.usertest
        );
        report_optional_metric!(
            sensor,
            GAUGE_SENSORS_CONFIG_SENSITIVITY,
            sensor.config.sensitivity
        );
        report_optional_metric!(
            sensor,
            GAUGE_SENSORS_CONFIG_SENSITIVITYMAX,
            sensor.config.sensitivitymax
        );
        report_optional_metric!(
            sensor,
            GAUGE_SENSORS_CONFIG_THOLDDARK,
            sensor.config.tholddark
        );
        report_optional_metric!(
            sensor,
            GAUGE_SENSORS_CONFIG_THOLDOFFSET,
            sensor.config.tholdoffset
        );

        // state metrics -------------------------------------------------
        // TODO: string value // report_metric!(sensor, GAUGE_SENSORS_STATE_LASTUPDATED, sensor.state.lastupdated);
        report_optional_metric!(sensor, GAUGE_SENSORS_STATE_DAYLIGHT, sensor.state.daylight);
        report_optional_metric!(sensor, GAUGE_SENSORS_STATE_FLAG, sensor.state.flag);
        report_optional_metric!(sensor, GAUGE_SENSORS_STATE_PRESENCE, sensor.state.presence);
        report_optional_metric!(sensor, GAUGE_SENSORS_STATE_DARK, sensor.state.dark);
        report_optional_metric!(sensor, GAUGE_SENSORS_STATE_STATUS, sensor.state.status);
        report_optional_metric!(
            sensor,
            GAUGE_SENSORS_STATE_BUTTONEVENT,
            sensor.state.buttonevent
        );
        if let Some(temp) = sensor.state.temperature {
            report_metric!(sensor, GAUGE_SENSORS_STATE_TEMPERATURE, temp as f64 / 100.0);
        }
        report_optional_metric!(
            sensor,
            GAUGE_SENSORS_STATE_LIGHTLEVEL,
            sensor.state.lightlevel
        );
    }

    for light in lights.values() {
        // state metrics -------------------------------------------------
        report_metric!(light, GAUGE_LIGHTS_STATE_REACHABLE, light.state.reachable);
        report_metric!(light, GAUGE_LIGHTS_STATE_ON, light.state.on);
        report_optional_metric!(light, GAUGE_LIGHTS_STATE_BRI, light.state.bri);
        report_optional_metric!(light, GAUGE_LIGHTS_STATE_HUE, light.state.hue);
        report_optional_metric!(light, GAUGE_LIGHTS_STATE_SAT, light.state.sat);
        report_optional_metric!(light, GAUGE_LIGHTS_STATE_CT, light.state.ct);
    }

    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();

    info!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let hue_bridge_url =
        env::var("HUE_BRIDGE_URL").unwrap_or_else(|_| "http://philips-hue.local".to_string());
    let hue_token = match env::var("HUE_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            println!("HUE_TOKEN is not set, starting authorization flow...");
            loop {
                match HueClient::authorize(&hue_bridge_url) {
                    Ok(token) => {
                        println!(
                            "Received a token!  Please set the following environment variable:"
                        );
                        println!("\tHUE_TOKEN={}", token);
                        println!("and restart the hue_exporter.");

                        return Ok(());
                    }
                    Err(e) => {
                        println!("{}", e);
                        println!("Press [ENTER] to try again...");
                        read_and_discard_stdin();
                    }
                }
            }
        }
    };

    let hue_client = HueClient::new(hue_token, hue_bridge_url);
    debug!("hue client set to: {:?}", hue_client);

    actix_web::rt::System::new().block_on(async move {
        let server = HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .app_data(Data::new(hue_client.clone()))
                .service(index)
                .service(metrics)
        })
        .bind(env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:9369".to_string()))?
        .run();
        server.await?;

        Ok(())
    })
}

fn read_and_discard_stdin() {
    use std::io::BufRead;

    std::io::stdin().lock().lines().next().unwrap().unwrap();
}
