package services

import (
	"fmt"
	"regexp"
	"strconv"
)

const expr = "\\W*((?i)%s(?-i))\\W*"


func badwords() (list []string) {
	x := []string{"uwu", "owo", ":v", ":3", "ewe", "iwi", "awa"}
	list = x
	return list
}

func comboCheck(message string) (int) {
	badWordsList := badwords()
	combo := false
	comboCount := 0
	for i := 0; i < len(badWordsList); i++ {
		match, _ := regexp.MatchString(fmt.Sprintf(expr, badWordsList[i]), message)
		if match {
			combo = true
		} else {
			combo = false
		}
		if combo {
			comboCount += 1
		}
	}
	return comboCount
}
func Message(message string) *string {
	comboCount := comboCheck(message)
	var messageToSend string
	if comboCount > 0 {
		if comboCount > 1 {
			messageToSend = fmt.Sprintf("Deficiente x%s", strconv.Itoa(comboCount))
			return &messageToSend
		}
		messageToSend = "Deficiente"
		return &messageToSend
	}
	return nil
}
