package routines

import (
	"log"
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
	day, month, weekday := getCurrentDaynMonth()
	var message string
	switch month {
	case 1:
		switch day {
		case 1:
			message = "Feliz año nuevo!"
			return &message
		case 30:
			message = "Felicidades @LilNarwhal"
			return &message
		}
	case 2:
		switch day {
		case 1:
			message = "Chavalotes que ya es febrero"
			return &message
		case 7:
			message = "Felicidades @Josewe"
			return &message
		case 14:
			message = "Sam va lentin"
			return &message
		case 26:
			message = "Felicidades @thedrdvd"
			return &message
		}
	case 3:
		if day == 1 {
			message = "Chavalotes ya es Marzo"
			return &message
		}
	case 4:
		if day == 1 {
			message = "En Abril aguas mil"
			return &message
		}
	case 5:
		switch day {
		case 1:
			message = "Hasta el 40 Mayo no te quites el sayo"
			return &message
		case 4:
			message = "Felicidades @r3dmsr"
			return &message
		case 9:
			message = "Felicidades @thexiao77"
			return &message
		}
	case 6:
		if day == 1 {
			message = "Junio, empieza el veranito"
			return &message
		}
	case 7:
		switch day {
		case 1:
			message = "Julio, a morir de calor"
			return &message
		case 8:
			message = "Felicidades @Sanz97xx"
			return &message
		}
	case 8:
		switch day {
		case 1:
			message = "Agosto, a seguir muriendo de calor"
			return &message
		case 8:
			message = "Felicidades @Sauturn"
			return &message
		}
	case 9:
		if day == 1 {
			message = "Septiembre, fin de las vacaciones"
			return &message
		}
	case 10:
		switch day {
		case 1:
			message = "Ya es Octubre"
			return &message
		case 10:
			message = "Felicidades @txc450"
			return &message
		case 12:
			message = "🇪🇸 Feliz dia de Españita 🇪🇸"
			return &message
		case 16:
			message = "https://www.youtube.com/watch?v=KnrKrHhqKyk @DarkTrainer"
			return &message
		}
	case 11:
		switch day {
		case 1:
			message = "Ya es Noviembre!"
			return &message
		case 20:
			message = "https://www.youtube.com/watch?v=RS_4HosMJuE"
			return &message
		}
	case 12:
		switch day {
		case 1:
			message = "Feliz mes de Diciembre!"
			return &message
		case 25:
			message = "Feliz navidad!"
			return &message
		}
	}
	log.Println(message, weekday)
	if weekday == 4 {
		message = "Feliz jueves!"
		return &message
	}
	return nil
}
