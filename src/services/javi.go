package services

import "math/rand"

func randomNumber(min int, max int) int {
	return (rand.Intn(max - min)) + min
}

func CheckJavi() (bool, *string) {
	number := randomNumber(0, 100)
	tmp := "Deficiente"
	return number == 69, &tmp
}
