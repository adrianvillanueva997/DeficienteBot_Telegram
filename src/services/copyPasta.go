package services

import (
	"fmt"
	"regexp"
)

func pastawords() []string {
	return []string{"cbt", "lentejas", "pan", "colegas", "amiga",
		"gimnasio", "paseo", "conciencia", "paraguaya",
		"paja", "cuerpazo", "halloween", "niño", "ayuso"}
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

func PastaText(receivedMessage string) []*string {
	matches := pastaCheck(receivedMessage)
	var copypastas []*string
	for i := 0; i < len(matches); i++ {
		switch matches[i] {
		case "cbt":
			copypastas = append(copypastas, stringToPointer("Cock and ball torture (CBT), penis torture or dick torture is a sexual activity involving application of pain or constriction to the penis or testicles. This may involve directly painful activities, such as genital piercing, wax play, genital spanking, squeezing, ball-busting, genital flogging, urethral play, tickle torture, erotic electrostimulation, kneeing or kicking."))
		case "lentejas":
			copypastas = append(copypastas, stringToPointer("Lo mejor de las maduras, es que puedes comerles el roscón de reyes, mientras te tienen al fuego unas lentejas de puta madre. Yo recuerdo una que conocí en un eroski, y la recuerdo como uno de los mejores polvos de mi vida. Ella me hizo unos callos cojonudisisimos, y mientras los preparaba, yo le daba como un cabrón por el ojete, ya que se había puesto faldita para que fuese haciendo mientras cocinaba. Creo que eyaculé tal cantidad de esperma, que estuve dos horas inconsciente. Menos mal que los callos me dieron fuerza para acabar el día con un par más. Y tenía unos hijos majísimos. Menudos vicios echamos al crash bandicoot 2."))
		case "colegas":
			copypastas = append(copypastas, stringToPointer("Pues ayer que sali con mis colegas vale y bua estaba con mi morao vale bailando a mi rollo con camiseta de manga corta vale y la verdad es que voy al gimnasio no sigo dietas ni nada de eso porque no me gusta y segundo que no me hace falta aunque reconozco que soy feo vale pero tengo labia entonces cojo vale y eso que se me acaba el cubatica y digo bua loco aora que hago que es pasta y tampoco esta la cosa para ir gastando en bebida vale y cojo y voy a la barra y habia una camarera que estaba muy buena vale y me mira fijamente y le digo sin pensar oye perdona es que se van a pelear y me dice quien??!! y le digo mi poya en tu paladar jaajaaaja se partio el culo riendose mientras le guiñaba el ojo sonriendo vale y le digo como te echo reir enrollate no loca y invitame a un cubatilla ajajaja y se lo dije por decir eh y eso que coje loco y me lo pone jajajaja y me pregunta que de donde soi que nunca me habia visto por ahi pues cojo le explico y eso y le digo que a ver que pasa con su rollo que aber si quedamos fuera sabes y coje y ni corta ni perezosa me dice hoy mismo!bua aqui locos si que flipe ya sabes y bueno pues na se lo dije a los colegas y a la hora de cerrar me espere fuera vale y viene y me dice bueno y a donde vamos y le digo pues nose quieres desayunar algo?puesto que era tarde y me dice dejate de desayunos que a mi me gusta mas pasarla bien buaaaa yo flipando loco pero tenia algo malo todo hay que decirlo y es que era sudamericana vale pero pense en nuestros antepasados en el gran cristobal colon y no podia dejar el liston bajo asi que acabemos en su casa y me imajinaba que era indigena y yo conquistaba su pueblo jajaja alfinal me corri y me fui para casa sabes y con la cabeza alta por nuestras generaciones no descubri america pero si me la folle"))
		case "amiga":
			copypastas = append(copypastas, stringToPointer("\"amiga\""))
		case "gimnasio":
			copypastas = append(copypastas, stringToPointer("Llevo bastante tiempo en el gimnasio poniendome mazadete para poder ir reventando pussies por doquier, de momento en 3 años de gimnasio he follado 0 veces pero bueno el camino se hace caminando, almenos he intercambiado unas miradas con una hot latin que tendra como 40 años y usa peluca, yo le he visto anillo de casada pero yo le veo en la cara que quiere dick. \n			Pero bueno al lio porque justo ahi viene el problema, yo normalmente me considero un tio no interesado en los penes ajenos, pero cuando estoy en los vestuarios pues como es normal miras las pollas de los otros para comparartela, por desgracia siempre salgo perdiendo porque mi polla es de sangre, pero aun asi hay verdaderos pollones que solo puedo apreciar durante 1 segundo como mucho, porque sino se me nota mucho, pero es que tengo curiosidad por tocarlos, no soy homo lo digo completamente en serio pero me direis que no nunca habeis tenido la curiosidad de tocar una polla grande que no sea la vuestra. A mi por lo menos si me despierta curiosidad.\n 			Entonces claro veo pedazos de pollas en el gimnasio y me jode porque me imagino que esas cosas penetran a las mujeres y despues voy yo en plan pff, pos ok, a recoger los restos porque depsues de meter semejantes nardos no me jodas. \n			Por eso yo quiero una china virgen, eso es el sumum de la estrechez, que cada vez que se la meta grite cosas tipo \"ooh oni-chan wait wait, you are killing me\" o que me diga que parece que le estoy abriendo las entrañas por dentro y pegarle puñetazos mientras le miro a los ojos y le digo: One-san hoy eres mia\" y besarla. \n			Yo se que esto es dificil porque la sociedad actual esta jodidamente hecha mierda, irme a al china rural es un puto rollo, no se el idioma, nociones basicas tipo Ohayou, ochinchin, hentai, tentacles y poca cosa mas, entonces claro yo no se que hacer, si seguir mirando los penes de otros hombres o no, porque me vienen todas estas cosas a la cabeza y quizas no es muy sano."))
		case "paseo":
			copypastas = append(copypastas, stringToPointer("Pam pam!!!!! estaba yo ayer ahiii to locooo por la calle dando un paseo ahi to desfasao, y de repente viene una pava y me dices tienes un cigarro y le digoooo un puro habano pa tu boca putaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa aaaaaaaaaa, y se lo tomo a mal, y se lo dijo a un polisia que habia ahi al laooo y el tio me viene y me diseeeee la violamos entre los dos loco loco locooooooooooo y yo joderrrrrrr, que tiene 15 añossss jajajajaa pero al final no era un polisia, era Braulio. Ahsadishaudhusahuhuahua puto Braulio. \n Me fui con el de cañas y al final estuvimos ahi to locos tomando mdma hasta que vino una señora y nos dijo payoooos que eso es to malo tomar de esto que es mucho mejor, y nos dio una sopica de ajo to rica jajajajaajahauuahauajaj puta vieja, nos robo la droja y se fue con sus amigas al bingo a ponerse to morá jashajaj. \n Arriquitaun taun taun!!!!"))
		case "conciencia":
			copypastas = append(copypastas, stringToPointer("he perdido la conciencia varias veces por el alcohol...el coma pues fue en un san juan en una moraga, pfff ni me acuerdo lo que bebí compré una botella de wisky y algo de cerveza...pero es que no sólo me bebí mi parte, por lo visto con la gente que íba había pillado cantidades bestia, y sobraba por todos lados, todo el mundo me invitaba así que ni puta idea de lo que bebí, eso con 17 años...casi me follo una de 23, pero mi sentido aracnido de folla modelos me saltó, y conforme amanecia se le veía mejor la cara pero el cuerpazo lo seguía teniendo...no sé cómo llegué a casa, saludé a quién me encontré por los pasillos, me senté en una silla....y caí de cabeza al suelo, y ahí me quedé dormidito, hasta que consiguieron despertarme."))
		case "paraguaya":
			copypastas = append(copypastas, stringToPointer("Yo conocí a una paraguaya a la que le gustaba decirme 'hasme por detrás'. \n Estábamos en el sofá de la salita dándole y me vino un olor a mierda bastante sospechoso. Cabe apuntar que iba fumadísimo y deduje que le estaba percutiendo el ano. Aguanté porque si esa chica hablaba español era porque alguien antes aguantó un viaje muy largo en un barco lleno de mierda y sin quejarse. Así que apreté los dientes y justiqué que mi polla oliese a una pocilga con problemas de cañerías porque ella acababa de salir de currar y con el calentón nos liamos. Pero el mundo se me vino abajo cuando se giró y con ojos guaranís me dijo 'ahora hasme por detrás'"))
		case "paja":
			copypastas = append(copypastas, stringToPointer("Estaba en mi cuarto tendido en la cama y me dio por tocarme (cosas de esa edad) estaba ya to tieso cuando me baje los pantalones y empeze a cascarmela, no habia peligro, mi hermano estaba en su habitacion y mi madre no estaba, pues estoy ahi dandole y de repente se abre la puerta y me ve mi madre dandole a la zambomba, me quedo en blanco y no se me ocurre otra cosa que hacer que moverme, levantandome y acostandome asi seguido como si estuviera poseido gritando: QUE ME HE VUELTO LOCOOO, QUE ME HE VUELTO LOCOOO!!! y mi madre asustada y todo y yo seguia asi vino mi hermano me vio y descojonandose abrio la puerta y en la calle riendose tirado en el suelo con lo que se enteró mi primo que vivia al lao al oir la risa mi hermano se lo contó, mi madre con cara de y yo haciendome el poseido, de eso no se a vuelto a hablar nunca."))
		case "cuerpazo":
			copypastas = append(copypastas, stringToPointer("Menudo hijo de puta, tienes un cuerpazo del copón. De gordo nada, tu estás fuerte. Menudas espaldas tienes, hijo de la gran puta. Olvídate de adelgazar, potencia ese físico tan espectacular que tienes, hijo de una perra sarnosa. Qué puta envidia me das! Cuida un poco la alimentación y tira de máquinas en el gim. Natación y remo es lo que necesitas para terminar de ponerte del copón. \n Menudo hijo de la gran puta eres, ya quiesiera yo ese pedazo de cuerpo. Cabronazo, hijo de mil putas, te invidio muchísimo. Insisto, cuida un poco la dieta y machácate en el gim, con ese cuerpo puedes quedarte del copón, cabronazo. Y a los que dicen que estás gordo ni puto caso, son un atajo de maricones insecto-palos que solo les gusta lamer falos. \n Tu, sin embargo, estás del copón, hijo de la grandísima puta. Qué suerte tenéis algunos con la genética."))
		case "halloween":
			copypastas = append(copypastas, stringToPointer("Qué es Jalogüin?"))
		case "niño":
			copypastas = append(copypastas, stringToPointer("Le cojes y les dices escúchame tío, que yo no soy un niño, tío, yo no soy un niño, tío, yo no estoy pa que me hagan perder el tiempo. ¿Me entiendes? Que es lo que me hace perder tu gente. Ya está primo, que si eres tan bravo tío, cuando quieras quedas conmigo, hermano. Y yo sí sé como funcionan hermano. Aquí hay walthers, hay 38s, hermano, hay 9mm, hay lo que quieras, compadre. Yo no sé qué tú te estás pensando que es esto, compadre, THIS IS THE JUNGLE, NIGGA. Escúchame, es que, es que ya me está tocando la polla, tío, estás tonterías, de tu coro de niños pequeños, tío, que escúchame, que venga, primo, que venga aquí a la calle Aguilón nº9, primo. Que venga ya tu colega si es tan bravo, que ayer me tuvo ahí esperando, compadre, hasta la 1 y media de la noche, hermano, y no vino ni Dios. ¿Qué pasa con tu rollo, sois tan bravos, primo? Pues si sois tan bravos, vente, vente de verdad, hermano. Estuve ayer con mi colega, primo y es que menos mal que no te encontré, chivato de mierda, porque estuve con mi colega dando vueltas por ahí con el coche, con la pistola, maricón. ¿Tú con quién te piensas que estás hablando, pipudo? ¿Eh? Ts. Y es que has tenido hasta suerte, maricón, te ha venido hasta bien, el que no vinieras, porque es que te hubiera metido un tiro en la rodilla, payaso, que eres un payaso."))
		case "ayuso":
			copypastas = append(copypastas, stringToPointer("Estoy muy jodido, Ayuso me ha arruinado la puta vida, cuando me despierto lo primero que veo es un cartel de Ayuso colgado en mi pared que me pone muy cachondo pero tengo que prepararme para ir al trabajo no sin antes ir al baño para hacerme la paja con Ayuso. En trabajo voy cada dos horas a hacerme la paja con Ayuso y mientras estoy trabajando solo pienso en ella y en lo guapa que es. Cuando por fin llego a mi casa lo primero que hago es hacerme una paja con Ayuso, después me ducho ceno y me vuelvo a hacer una paja con Ayuso, luego voy a ver los mejores discursos de Ayuso y mientras me pongo cachondo al final del día me voy a la cama y me hago un par de pajas más con Ayuso y después me acuesto y me despierto a media noche así que me hago otra paja. En los fin de semanas me levanto con un charco de semen en le pantalón solo de pensar en Ayuso así que me hago otra paja con Ayuso y voy a ducharme, como y me hago una paja con Ayuso, desayuno y me hago una paja con Ayuso, ando y me hago una paja con Ayuso pajas pajas Ayuso pajas Ayuso"))
		default:
		}
	}

	return copypastas
}

func stringToPointer(text string) *string {
	return &text
}
