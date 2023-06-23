package services

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

const expr = "\\W*((?i)%s(?-i))\\W*"

func badwords() []string {
	return []string{"uwu", "owo", ":v"}
}

func comboCheck(message string) int {
	badWordsList := badwords()
	comboCount := 0
	tokens := strings.Split(message, " ")
	for j := 0; j < len(tokens); j++ {
		for i := 0; i < len(badWordsList); i++ {
			match, _ := regexp.MatchString(fmt.Sprintf(expr, badWordsList[i]), tokens[j])
			if match {
				comboCount++
			}
		}
	}
	return comboCount
}

func Message(message string) *string {
	comboCount := comboCheck(message)
	var messageToSend string

	switch {
	case comboCount > 25:
		messageToSend = fmt.Sprintf("Te voy a meter un escopetazo en los cojones x%s", strconv.Itoa(comboCount))
	case comboCount > 20:
		messageToSend = fmt.Sprintf("Me cago en tu puta madre x%s", strconv.Itoa(comboCount))
	case comboCount > 15:
		messageToSend = fmt.Sprintf("Mongolo x%s", strconv.Itoa(comboCount))
	case comboCount > 10:
		messageToSend = fmt.Sprintf("Subnormal x%s", strconv.Itoa(comboCount))
	case comboCount >= 5:
		messageToSend = fmt.Sprintf("Gilipollas x%s", strconv.Itoa(comboCount))
	case comboCount > 0:
		messageToSend = fmt.Sprintf("Deficiente", strconv.Itoa(comboCount))
	default:
		return nil
	}

	return &messageToSend
}
