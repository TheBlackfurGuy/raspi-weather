use yew::{Component, html, Callback, Properties};
use gloo::timers::callback::Interval;

pub struct SettingsComponent {
    interval: Interval,
}

pub enum Msg {
    Update,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    settings_callback: Option<u8>
}

impl Component for SettingsComponent {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let clock_hanlde = {
            let link = ctx.link().clone();
            Interval::new(1000, move || link.send_message(Msg::Update))
        };
        Self {
            interval: clock_hanlde,
        }
    }

    fn update(&mut self, _: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update => true,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"weekday".into(), &"short".into()).unwrap();
        js_sys::Reflect::set(&options, &"year".into(), &"numeric".into()).unwrap();
        js_sys::Reflect::set(&options, &"month".into(), &"short".into()).unwrap();
        js_sys::Reflect::set(&options, &"day".into(), &"numeric".into()).unwrap();
        html!{
            <p style="font-size: 70%">
                { js_sys::Date::new_0().to_locale_time_string("de-DE").to_string() } <br/>
                { js_sys::Date::new_0().to_locale_date_string("de-DE", &options).to_string() }
            </p>
        }
    }
}
