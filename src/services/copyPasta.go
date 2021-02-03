package services

import (
	"fmt"
	"regexp"
)

func pastawords() []string {
	return []string{"cbt", "lentejas"}
}
func pastaCheck(message string) []string {
	copyPastaList := pastawords()
	var checks []string
	for i := 0; i < len(copyPastaList); i++ {
		match, _ := regexp.MatchString(fmt.Sprintf(expr, copyPastaList[i]), message)
		if match {
			checks = append(
				checks,
				copyPastaList[i],
			)
		}
	}
	return checks
}

func PastaText(receivedMessage string) *string {
	// TODO: For now the copy pasta will only work with the first case that matches, I should add support for other pastas.
	matches := pastaCheck(receivedMessage)
	if len(matches) != 0 {
		var message string
		switch matches[0] {
		case "cbt":
			message = "Cock and ball torture (CBT), penis torture or dick torture is a sexual activity involving application of pain or constriction to the penis or testicles. This may involve directly painful activities, such as genital piercing, wax play, genital spanking, squeezing, ball-busting, genital flogging, urethral play, tickle torture, erotic electrostimulation, kneeing or kicking."
		case "lentejas":
			message = "Lo mejor de las maduras, es que puedes comerles el roscón de reyes, mientras te tienen al fuego unas lentejas de puta madre. Yo recuerdo una que conocí en un eroski, y la recuerdo como uno de los mejores polvos de mi vida. Ella me hizo unos callos cojonudisisimos, y mientras los preparaba, yo le daba como un cabrón por el ojete, ya que se había puesto faldita para que fuese haciendo mientras cocinaba. Creo que eyaculé tal cantidad de esperma, que estuve dos horas inconsciente. Menos mal que los callos me dieron fuerza para acabar el día con un par más. Y tenía unos hijos majísimos. Menudos vicios echamos al crash bandicoot 2."
		default:
			return nil
		}
		return &message
	}
	return nil
}
