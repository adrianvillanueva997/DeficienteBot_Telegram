package services

import (
	"fmt"
	"log"
	"regexp"
	"strconv"
	"strings"
)

const expr = "\\W*((?i)%s(?-i))\\W*"

func badwords() (list []string) {
	x := []string{"uwu", "owo", ":v", ":3", "ewe", "iwi", "awa", "x3"}
	list = x
	return list
}

func comboCheck(message string) int {
	badWordsList := badwords()
	comboCount := 0
	tokens := strings.Split(message, " ")
	for j := 0; j < len(tokens); j++ {
		for i := 0; i < len(badWordsList); i++ {
			match, _ := regexp.MatchString(fmt.Sprintf(expr, badWordsList[i]), tokens[j])
			if match {
				comboCount += 1
			}
		}
	}
	return comboCount
}
func Message(message string) *string {
	comboCount := comboCheck(message)
	var messageToSend string
	if comboCount > 0 {
		log.Println(comboCount)
		if comboCount > 1 {
			messageToSend = fmt.Sprintf("Deficiente x%s", strconv.Itoa(comboCount))
			if comboCount >= 5 {
				messageToSend = fmt.Sprintf("Gilipollas x%s", strconv.Itoa(comboCount))
			}
			if comboCount >= 10 {
				messageToSend = fmt.Sprintf("Subnormal x%s", strconv.Itoa(comboCount))
			}
			if comboCount >= 15 {
				messageToSend = fmt.Sprintf("Mongolo x%s", strconv.Itoa(comboCount))
			}
			if comboCount >= 20 {
				messageToSend = fmt.Sprintf("Me cago en tu puta madre x%s", strconv.Itoa(comboCount))
			}
			if comboCount >= 25 {
				messageToSend = fmt.Sprintf("Te voy a meter un escopetazo en los cojones x%s", strconv.Itoa(comboCount))
			}


			return &messageToSend
		}
		messageToSend = "Deficiente"
		return &messageToSend
	}
	return nil
}
