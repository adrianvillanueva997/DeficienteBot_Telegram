package routines

import (
	"time"
)

func getCurrentDaynMonth() (int, int, int) {
	t := time.Now()
	month := int(t.Month()) // type time.Month
	day := t.Day()
	weekday := int(t.Weekday())
	return day, month, weekday
}

func CheckEvents() *string {
	day, month, _ := getCurrentDaynMonth()
	var message string
	switch month {
	case 1:
		switch day {
		case 1:
			message = "Felis año nuevo"
		case 30:
			message = "Felicidades @LilNarwhal"
		}
	case 2:
		switch day {
		case 1:
			message = "Chavalotes que ya es febrero"
		case 7:
			message = "Felicidades @Josewe"
		case 14:
			message = "Sam va lentin"
		case 26:
			message = "Felicidades @thedrdvd"
		}
	case 3:
		switch day {
		case 1:
			message = "Chavalotes ya es Marzo"
		case 8:
			message = "Felicidades mujeres"
		}
	case 4:
		switch day {
		case 1:
			message = "En Abril aguas mil"
		case 20:
			message = "Felicidades porreros"

		}
	case 5:
		switch day {
		case 1:
			message = "Hasta el 40 Mayo no te quites el sayo"
		case 4:
			message = "Felicidades @r3dmsr"
		case 6:
			message = "Felicidades @DoctorMckay"
		case 9:
			message = "Felicidades @thexiao77"
		}
	case 6:
		if day == 1 {
			message = "Junio, empieza el veranito"
		}
	case 7:
		switch day {
		case 1:
			message = "Julio, a morir de calor"
		case 8:
			message = "Felicidades @Sanz97xx"
		}
	case 8:
		switch day {
		case 1:
			message = "Agosto, a seguir muriendo de calor"
		case 2:
			message = "Felicidades al más guapo de Asturies @Sauturn"
		}
	case 9:
		switch day {
		case 1:
			message = "Septiembre, fin de las vacaciones"
		case 11:
			message = "Felicidades Torres Gemelas!"
		case 15:
			message = "Feliciades @CecilioGil"
		}
	case 10:
		switch day {
		case 1:
			message = "💀 SpookTober 💀"
		case 5:
			message = "Felicidades al segundo más guapo de Asturies, @davasjoe"
		case 7:
			message = "Felicidades @txc450"
		case 8:
			message = "Felicidades @Naruto"
		case 12:
			message = "🇪🇸 Feliz dia de Españita 🇪🇸"
		case 16:
			message = "https://www.youtube.com/watch?v=KnrKrHhqKyk @DarkTrainer"
		}
	case 11:
		switch day {
		case 1:
			message = "⛔💦 Queda inaugurada la temporada de No Fap November ⛔💦"
		case 20:
			message = "Franco ha muerto"
		}
	case 12:
		switch day {
		case 1:
			message = "Se viene la Navidad, a coger kilos de más! Feliz Diciembre!"
		case 25:
			message = "Feliz navidad!"
		}
	default:
		return nil
	}
	return &message
}

func HappyThursday() *string {
	var message string
	message = "Feliz jueves!"
	return &message

}
