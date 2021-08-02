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
	if err != nil {
		log.Fatalln(err)
	}
	tmp := "Deficiente"
	return number.Cmp(big.NewInt(int64(69))) == 1, &tmp // cmp values: -1 if x<y | 0 if x==y | +1 if x>y
}
