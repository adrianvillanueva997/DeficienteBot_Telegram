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
		"Que Dios te bendiga uwu",
		"Un placer, que tengas un bonito jueves precioso",
		"uwu",
		"Disfruta del jueves, nos vemos la semana que viene, fiera",
		"las que tu tienes, fiera, crack, maquina, mastodonte"
	}
	rand.Seed(time.Now().UnixNano())
	return messages[rand.Intn(len(messages))]
}
