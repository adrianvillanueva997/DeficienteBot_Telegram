use super::utils::get_todays_date;

pub fn special_event() -> &'static str {
    let (day, month, _year) = get_todays_date();
    match month {
        1 => match day {
            1 => "Feliz año nuevo!",
            30 => "Felicidades @LilNarwhal",
            _ => "",
        },
        2 => match day {
            1 => "Febrero",
            7 => "Felicidades @JoseAwe",
            14 => "Sam va lentin",
            26 => "Felicidades @thedrdvd",
            _ => "",
        },
        3 => match day {
            1 => "Mazo",
            8 => "Felicidades mujeres",
            _ => "",
        },
        4 => match day {
            1 => "Abril",
            20 => "Felicidades porreros",
            _ => "",
        },
        5 => match day {
            1 => "Mayo",
            4 => "Felicidades @r3dmsr",
            6 => "Felicidades @DoctorMckay",
            9 => "Felicidades @thexiao77",
            _ => "",
        },
        6 => match day {
            1 => "Junio",
            _ => "",
        },
        7 => match day {
            1 => "Julio",
            8 => "Felicidades @Sanz97xx",
            _ => "",
        },
        8 => match day {
            1 => "Agosto",
            2 => "Felicidades al más guapo de Asturies @Sauturn",
            _ => "",
        },
        9 => match day {
            1 => "Septiembre",
            11 => "Felicidades Torres Gemelas",
            15 => "Feliciades @CecilioGil",
            _ => "",
        },
        10 => match day {
            1 => "💀 SpookTober 💀",
            5 => "Felicidades al segundo más guapo de Asturies, @davasjoe",
            7 => "Felicidades @txc450",
            8 => "Felicidades @Naruto",
            12 => "🇪🇸 Feliz dia de Españita 🇪🇸",
            16 => "https://www.youtube.com/watch?v=KnrKrHhqKyk @DarkTrainer",
            _ => "",
        },
        11 => match day {
            1 => "⛔💦 Queda inaugurada la temporada de No Fap November ⛔💦",
            20 => "Felicidades Franco",
            _ => "",
        },
        12 => match day {
            1 => "💦 Queda inaugurada la temporada de Destroy Dick December 💦",
            25 => "Feliz navidad!",
            28 => "https://www.youtube.com/watch?v=xfr64zoBTAQ",
            _ => "",
        },
        _ => "",
    }
}
