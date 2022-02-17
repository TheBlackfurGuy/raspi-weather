use yew::prelude::*;

mod components;

use components::temperature::TemperatureComponent;
use components::weather::WeatherComponent;
use components::wind_angle::WindAngleComponent;
use components::settings::SettingsComponent;
use components::co2::CO2Component;
use components::humidity::HumidityComponent;

pub enum Msg {
    Update,
    Settings,
    Increment,
}

pub struct App {
    temperature: f32,
    weather: bool,
    settings: bool,
    wind_angle: i16,
    co2: u16,
    humidity: u8,
}

#[allow(unused_variables)]
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            weather: false,
            settings: false,
            temperature: 0.0,
            wind_angle: 0,
            co2: 800,
            humidity: 50,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update => {
                self.weather = !self.weather;
                true
            }
            Msg::Settings => {
                self.settings = !self.settings;
                true
            }
            Msg::Increment => {
                self.temperature += 1.0;
                self.wind_angle += 5;
                self.humidity += 10;
                self.co2 += 100;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.settings {
            true => {
                html!(
                    <>
                        <div>
                            <button class="button" onclick={ctx.link().callback(|_| Msg::Update)}>
                                { "Toggle Weather" }
                            </button>

                            <button class="button" onClick="window.location.reload();">
                                { "Reload" }
                            </button>

                            <button class="button" onclick={ctx.link().callback(|_| Msg::Settings)}>
                                { "Exit Settings" }
                            </button>

                            <button class="button" onclick={ctx.link().callback(|_| Msg::Increment)}>
                                { "Increment"}
                            </button>
                        </div>
                        <style>
                            { "body {" }
                            { "background-image: url(" }
                            { match self.weather {
                                false => "sunny.jpg",
                                true => "thunder.jpg",
                            } }
                            { ");" }
                            { "}" }
                        </style>
                    </>
                )
            }
            false => {
                html!(
                    <>
                        <div class="grid-wrapper">
                            <div>
                                <WindAngleComponent wind_angle={self.wind_angle} />
                            </div>
                            <div>
                                <TemperatureComponent temperature={self.temperature}/>
                            </div>
                            <div>
                                { "C" }
                            </div>
                            <div>
                                <CO2Component co2level={self.co2} />
                            </div>
                            <div>
                                { "E" }
                            </div>
                            <div>
                                <HumidityComponent humidity={self.humidity} />
                            </div>
                            <div>
                                { "G" }
                            </div>
                            <div>
                                <SettingsComponent settings_callback={ctx.link().callback(|_| Msg::Settings)}/>
                            </div>
                        </div>
                        <style>
                            { ".grid-wrapper {" }
                            { "display: grid;" }
                            { "grid-template-columns: repeat(4, 1fr);" }
                            { "grid-template-rows: repeat(2, 1fr);" }
                            { "height: 100vh;" }
                            { "}" }
                            { "body {"}
                            { "background-image: url(" }
                            { match self.weather {
                                false => "sunny.jpg",
                                true => "thunder.jpg",
                            } }
                            { ");" }
                            { "margin: 0;"}
                            { "}" }
                        </style>
                    </>
                )
            }
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
