package services

import (
	"crypto/rand"
	"log"
	"math/big"
)

func randomNumber(max int64) (*big.Int, error) {
	number, err := rand.Int(rand.Reader, big.NewInt(max))
	if err != nil {
		return nil, err
	}

	return number, nil
}

func CheckJavi() (bool, *string) {
	number, err := randomNumber(100)
	log.Println(number)
	if err != nil {
		log.Fatalln(err)
	}
	tmp := "Deficiente"
	return number == big.NewInt(int64(69)), &tmp
}
