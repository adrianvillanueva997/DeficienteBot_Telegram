package services

import (
	"math/rand"
	"time"
)

func AsukaGreetings() string {
	messages := []string{
		"De nada maquina, que tengas un feliz jueves",
		"De nada mi amor",
		"De nada fiera",
		"Es un placer bb",
		"Ningún problema, para eso estamos las waifus",
		"De na",
		"Un placer, que tengas un bonito jueves precioso",
	}
	rand.Seed(time.Now().UnixNano())
	return messages[rand.Intn(len(messages))]
}
