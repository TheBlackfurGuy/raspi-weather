use yew::{html, Component, Properties};

pub struct HumidityComponent;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub humidity: u8,
}

impl Component for HumidityComponent {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let percent = ctx.props().humidity;
        html!(
            <svg width="200px" height="200px">
                <polygon points="20,80 80,80, 50,20" stroke="black" stroke-width="5" fill={get_color_from_percent(&percent)}/>
                <text x="40" y="73" style="font-size: 50px;">{ "!" }</text>
                <text x="80" y="40" style="font-size: 25px;">{get_text_from_percent(&percent)}</text>
                <text x="80" y="70" style="font-size: 20px;">{ "Feuchtigkeit" }</text>
                <text x="50" y="150" style="font-size: 60px;">{ format!("{} %", &percent) }</text>
            </svg>
        )
    }
}

fn get_color_from_percent(percent: &u8) -> String {
    match percent {
        71..=100 => "red".to_string(),
        51..=70 => "yellow".to_string(),
        31..=50 => "green".to_string(),
        21..=30 => "yellow".to_string(),
        11..=20 => "red".to_string(),
        0..=10 => "blue".to_string(),
        101..=u8::MAX => "blue".to_string(),
    }
}

fn get_text_from_percent(percent: &u8) -> String {
    match percent {
        91..=100 => "Hohe".to_string(),
        81..=90 => "Mittlere".to_string(),
        61..=80 => "Niedrige".to_string(),
        0..=60 => "BROKEN".to_string(),
        101..=u8::MAX => "BROKEN".to_string(),
    }
}
