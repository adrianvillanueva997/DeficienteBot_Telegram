package services

import (
	"fmt"
	"regexp"
)

func pastawords() []string {
	return []string{"cbt", "lentejas", "pan", "colegas", "amiga", "gimnasio"}
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
		case "colegas":
			message = "Pues ayer que sali con mis colegas vale y bua estaba con mi morao vale bailando a mi rollo con camiseta de manga corta vale y la verdad es que voy al gimnasio no sigo dietas ni nada de eso porque no me gusta y segundo que no me hace falta aunque reconozco que soy feo vale pero tengo labia entonces cojo vale y eso que se me acaba el cubatica y digo bua loco aora que hago que es pasta y tampoco esta la cosa para ir gastando en bebida vale y cojo y voy a la barra y habia una camarera que estaba muy buena vale y me mira fijamente y le digo sin pensar oye perdona es que se van a pelear y me dice quien??!! y le digo mi poya en tu paladar jaajaaaja se partio el culo riendose mientras le guiñaba el ojo sonriendo vale y le digo como te echo reir enrollate no loca y invitame a un cubatilla ajajaja y se lo dije por decir eh y eso que coje loco y me lo pone jajajaja y me pregunta que de donde soi que nunca me habia visto por ahi pues cojo le explico y eso y le digo que a ver que pasa con su rollo que aber si quedamos fuera sabes y coje y ni corta ni perezosa me dice hoy mismo!bua aqui locos si que flipe ya sabes y bueno pues na se lo dije a los colegas y a la hora de cerrar me espere fuera vale y viene y me dice bueno y a donde vamos y le digo pues nose quieres desayunar algo?puesto que era tarde y me dice dejate de desayunos que a mi me gusta mas pasarla bien buaaaa yo flipando loco pero tenia algo malo todo hay que decirlo y es que era sudamericana vale pero pense en nuestros antepasados en el gran cristobal colon y no podia dejar el liston bajo asi que acabemos en su casa y me imajinaba que era indigena y yo conquistaba su pueblo jajaja alfinal me corri y me fui para casa sabes y con la cabeza alta por nuestras generaciones no descubri america pero si me la folle"
		case "amiga":
			message = "\"amiga\""
		case "gimnasio":
			message = "Llevo bastante tiempo en el gimnasio poniendome mazadete para poder ir reventando pussies por doquier, de momento en 3 años de gimnasio he follado 0 veces pero bueno el camino se hace caminando, almenos he intercambiado unas miradas con una hot latin que tendra como 40 años y usa peluca, yo le he visto anillo de casada pero yo le veo en la cara que quiere dick. \n			Pero bueno al lio porque justo ahi viene el problema, yo normalmente me considero un tio no interesado en los penes ajenos, pero cuando estoy en los vestuarios pues como es normal miras las pollas de los otros para comparartela, por desgracia siempre salgo perdiendo porque mi polla es de sangre, pero aun asi hay verdaderos pollones que solo puedo apreciar durante 1 segundo como mucho, porque sino se me nota mucho, pero es que tengo curiosidad por tocarlos, no soy homo lo digo completamente en serio pero me direis que no nunca habeis tenido la curiosidad de tocar una polla grande que no sea la vuestra. A mi por lo menos si me despierta curiosidad.\n 			Entonces claro veo pedazos de pollas en el gimnasio y me jode porque me imagino que esas cosas penetran a las mujeres y despues voy yo en plan pff, pos ok, a recoger los restos porque depsues de meter semejantes nardos no me jodas. \n			Por eso yo quiero una china virgen, eso es el sumum de la estrechez, que cada vez que se la meta grite cosas tipo \"ooh oni-chan wait wait, you are killing me\" o que me diga que parece que le estoy abriendo las entrañas por dentro y pegarle puñetazos mientras le miro a los ojos y le digo: One-san hoy eres mia\" y besarla. \n			Yo se que esto es dificil porque la sociedad actual esta jodidamente hecha mierda, irme a al china rural es un puto rollo, no se el idioma, nociones basicas tipo Ohayou, ochinchin, hentai, tentacles y poca cosa mas, entonces claro yo no se que hacer, si seguir mirando los penes de otros hombres o no, porque me vienen todas estas cosas a la cabeza y quizas no es muy sano."
		default:
			return nil
		}
		return &message
	}
	return nil
}
