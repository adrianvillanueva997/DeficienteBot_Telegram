use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tracing::instrument;

const WORDS: [&str; 32] = [
    "cbt",
    "lentejas",
    "colegas",
    "amiga",
    "gimnasio",
    "conciencia",
    "paraguaya",
    "paja",
    "cuerpazo",
    "halloween",
    "niño",
    "ayuso",
    "prepucio",
    "bicho",
    "amogus",
    "china",
    "euskadi",
    "69",
    "spiderman",
    "viernes",
    "anime",
    "hetero",
    "tetas",
    "profe",
    "peruano",
    "amogus",
    "abogado",
    "novia",
    "pene",
    "xd",
    "porno",
    "sexo",
];

/// Find matching words in a string.
#[instrument]
pub async fn find_matching_words(sentence: &str) -> Vec<String> {
    let mut matching_words = Vec::new();
    let split_sentence = sentence.split_whitespace().collect::<Vec<_>>();
    for token in split_sentence {
        for word in WORDS {
            if token.to_lowercase().contains(word) {
                matching_words.push(word.to_string());
            }
        }
    }

    matching_words
}

/// Find copypastas in a string.
#[instrument]
pub async fn find_copypasta(input: &str) -> Vec<String> {
    let words = find_matching_words(input).await;
    words
        .par_iter()
        .map(|word| copypastas(word).to_string())
        .collect()
}

/// Return the copypasta for a given word.
fn copypastas(word: &str) -> &str {
    match word {
        "cbt" => "Cock and ball torture (CBT), penis torture or dick torture is a sexual activity involving application of pain or constriction to the penis or testicles. This may involve directly painful activities, such as genital piercing, wax play, genital spanking, squeezing, ball-busting, genital flogging, urethral play, tickle torture, erotic electrostimulation, kneeing or kicking.",
        "lentejas" => "Lo mejor de las maduras, es que puedes comerles el roscón de reyes, mientras te tienen al fuego unas lentejas de puta madre. Yo recuerdo una que conocí en un eroski, y la recuerdo como uno de los mejores polvos de mi vida. Ella me hizo unos callos cojonudisisimos, y mientras los preparaba, yo le daba como un cabrón por el ojete, ya que se había puesto faldita para que fuese haciendo mientras cocinaba. Creo que eyaculé tal cantidad de esperma, que estuve dos horas inconsciente. Menos mal que los callos me dieron fuerza para acabar el día con un par más. Y tenía unos hijos majísimos. Menudos vicios echamos al crash bandicoot 2.",
        "colegas" => "Pues ayer que sali con mis colegas vale y bua estaba con mi morao vale bailando a mi rollo con camiseta de manga corta vale y la verdad es que voy al gimnasio no sigo dietas ni nada de eso porque no me gusta y segundo que no me hace falta aunque reconozco que soy feo vale pero tengo labia entonces cojo vale y eso que se me acaba el cubatica y digo bua loco aora que hago que es pasta y tampoco esta la cosa para ir gastando en bebida vale y cojo y voy a la barra y habia una camarera que estaba muy buena vale y me mira fijamente y le digo sin pensar oye perdona es que se van a pelear y me dice quien??!! y le digo mi poya en tu paladar jaajaaaja se partio el culo riendose mientras le guiñaba el ojo sonriendo vale y le digo como te echo reir enrollate no loca y invitame a un cubatilla ajajaja y se lo dije por decir eh y eso que coje loco y me lo pone jajajaja y me pregunta que de donde soi que nunca me habia visto por ahi pues cojo le explico y eso y le digo que a ver que pasa con su rollo que aber si quedamos fuera sabes y coje y ni corta ni perezosa me dice hoy mismo!bua aqui locos si que flipe ya sabes y bueno pues na se lo dije a los colegas y a la hora de cerrar me espere fuera vale y viene y me dice bueno y a donde vamos y le digo pues nose quieres desayunar algo?puesto que era tarde y me dice dejate de desayunos que a mi me gusta mas pasarla bien buaaaa yo flipando loco pero tenia algo malo todo hay que decirlo y es que era sudamericana vale pero pense en nuestros antepasados en el gran cristobal colon y no podia dejar el liston bajo asi que acabemos en su casa y me imajinaba que era indigena y yo conquistaba su pueblo jajaja alfinal me corri y me fui para casa sabes y con la cabeza alta por nuestras generaciones no descubri america pero si me la folle",
        "conciencia" => "he perdido la conciencia varias veces por el alcohol...el coma pues fue en un san juan en una moraga, pfff ni me acuerdo lo que bebí compré una botella de wisky y algo de cerveza...pero es que no sólo me bebí mi parte, por lo visto con la gente que íba había pillado cantidades bestia, y sobraba por todos lados, todo el mundo me invitaba así que ni puta idea de lo que bebí, eso con 17 años...casi me follo una de 23, pero mi sentido aracnido de folla modelos me saltó, y conforme amanecia se le veía mejor la cara pero el cuerpazo lo seguía teniendo...no sé cómo llegué a casa, saludé a quién me encontré por los pasillos, me senté en una silla....y caí de cabeza al suelo, y ahí me quedé dormidito, hasta que consiguieron despertarme.",
        "paraguaya" => 	"Yo conocí a una paraguaya a la que le gustaba decirme 'hasme por detrás'. \n Estábamos en el sofá de la salita dándole y me vino un olor a mierda bastante sospechoso. Cabe apuntar que iba fumadísimo y deduje que le estaba percutiendo el ano. Aguanté porque si esa chica hablaba español era porque alguien antes aguantó un viaje muy largo en un barco lleno de mierda y sin quejarse. Así que apreté los dientes y justiqué que mi polla oliese a una pocilga con problemas de cañerías porque ella acababa de salir de currar y con el calentón nos liamos. Pero el mundo se me vino abajo cuando se giró y con ojos guaranís me dijo 'ahora hasme por detrás'",
        "paja" => "Yo me hice una paja en el baño de un avión. \n Estaba en un vuelo de 12 horas y me aburría. Me fui al baño y me la casqué. \n Cuando salí, había una cola de 5 personas esperando para entrar. \n Me sentí como un campeón.",
        "ayuso" => "Estoy muy jodido, Ayuso me ha arruinado la puta vida, cuando me despierto lo primero que veo es un cartel de Ayuso colgado en mi pared que me pone muy cachondo pero tengo que prepararme para ir al trabajo no sin antes ir al baño para hacerme la paja con Ayuso. En trabajo voy cada dos horas a hacerme la paja con Ayuso y mientras estoy trabajando solo pienso en ella y en lo guapa que es. Cuando por fin llego a mi casa lo primero que hago es hacerme una paja con Ayuso, después me ducho ceno y me vuelvo a hacer una paja con Ayuso, luego voy a ver los mejores discursos de Ayuso y mientras me pongo cachondo al final del día me voy a la cama y me hago un par de pajas más con Ayuso y después me acuesto y me despierto a media noche así que me hago otra paja. En los fin de semanas me levanto con un charco de semen en le pantalón solo de pensar en Ayuso así que me hago otra paja con Ayuso y voy a ducharme, como y me hago una paja con Ayuso, desayuno y me hago una paja con Ayuso, ando y me hago una paja con Ayuso pajas pajas Ayuso pajas Ayuso",
        "niño" => "Le cojes y les dices escúchame tío, que yo no soy un niño, tío, yo no soy un niño, tío, yo no estoy pa que me hagan perder el tiempo. ¿Me entiendes? Que es lo que me hace perder tu gente. Ya está primo, que si eres tan bravo tío, cuando quieras quedas conmigo, hermano. Y yo sí sé como funcionan hermano. Aquí hay walthers, hay 38s, hermano, hay 9mm, hay lo que quieras, compadre. Yo no sé qué tú te estás pensando que es esto, compadre, THIS IS THE JUNGLE, NIGGA. Escúchame, es que, es que ya me está tocando la polla, tío, estás tonterías, de tu coro de niños pequeños, tío, que escúchame, que venga, primo, que venga aquí a la calle Aguilón nº9, primo. Que venga ya tu colega si es tan bravo, que ayer me tuvo ahí esperando, compadre, hasta la 1 y media de la noche, hermano, y no vino ni Dios. ¿Qué pasa con tu rollo, sois tan bravos, primo? Pues si sois tan bravos, vente, vente de verdad, hermano. Estuve ayer con mi colega, primo y es que menos mal que no te encontré, chivato de mierda, porque estuve con mi colega dando vueltas por ahí con el coche, con la pistola, maricón. ¿Tú con quién te piensas que estás hablando, pipudo? ¿Eh? Ts. Y es que has tenido hasta suerte, maricón, te ha venido hasta bien, el que no vinieras, porque es que te hubiera metido un tiro en la rodilla, payaso, que eres un payaso.",
        "prepucio" => "Intentaron circuncidarme, pero mi prepucio solo volvió a fortalecerse. Desde entonces, me han circuncidado cada 6 meses. Mi prepucio ahora es más fuerte que el acero. Siempre que estoy en peligro, lo coloco sobre mi cuerpo como una capa exterior. Es completamente a prueba de balas, ignífugo, impermeable y extremadamente liviano. Tengo planes de venderlo como un material muy raro y muy resistente y ganar millones. Los puentes se harán con vigas de prepucio y las unidades de policía usarán chalecos de prepucio. Viviré en mi casa de prepucio y me bañaré en mi riqueza. Soy el hombre prepucio.",
        "bicho" => "SIIUUUUUUUUUUUUU",
        "china" => "
⣿⣿⣿⣿⣿⠟⠋⠄⠄⠄⠄⠄⠄⠄⢁⠈⢻⢿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⠃⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠈⡀⠭⢿⣿⣿⣿⣿
⣿⣿⣿⣿⡟⠄⢀⣾⣿⣿⣿⣷⣶⣿⣷⣶⣶⡆⠄⠄⠄⣿⣿⣿⣿
⣿⣿⣿⣿⡇⢀⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠄⠄⢸⣿⣿⣿⣿
⣿⣿⣿⣿⣇⣼⣿⣿⠿⠶⠙⣿⡟⠡⣴⣿⣽⣿⣧⠄⢸⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣾⣿⣿⣟⣭⣾⣿⣷⣶⣶⣴⣶⣿⣿⢄⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⡟⣩⣿⣿⣿⡏⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣹⡋⠘⠷⣦⣀⣠⡶⠁⠈⠁⠄⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣍⠃⣴⣶⡔⠒⠄⣠⢀⠄⠄⠄⡨⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣦⡘⠿⣷⣿⠿⠟⠃⠄⠄⣠⡇⠈⠻⣿⣿⣿⣿
⣿⣿⣿⣿⡿⠟⠋⢁⣷⣠⠄⠄⠄⠄⣀⣠⣾⡟⠄⠄⠄⠄⠉⠙⠻
⡿⠟⠋⠁⠄⠄⠄⢸⣿⣿⡯⢓⣴⣾⣿⣿⡟⠄⠄⠄⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⣿⡟⣷⠄⠹⣿⣿⣿⡿⠁⠄⠄⠄⠄⠄⠄⠄⠄
为党争光! Glory to the CCP!",
        "halloween" => "Qué es Jalogüin??",
        "amiga" => "\"amiga\"",
        "euskadi" => "el ojo de",
        "69" => "> nice",
        "spiderman" => " Os cuento, quede con una golfilla para intimar en su pisito, empezamos normal, nos liamos y demas (ella tenia bastante interes en chuparme la oreja, a un rato me dio hasta mal rollo la cabrona). Bueno, yo no lleve cartera y claro, cuando estabamos ya muy cachondos le dije amablamente si tenia condones, me dijo que no...hice como que me molestaba...ella me dijo que yo siempre suelo llevarlos, le dije que se me olvido la cartera y demas...total que accedio a hacer la marcha atras y yo pensando \"esta es la mia, le hare el spiderman\". Estamos arreando y demas y despues d eun buen rato veo que me voy a correr, saco mi tizona, me corro en mi mano y ella se queda un poco con cara de roto2, y entonces cojo y le tiro la lefada a su cara y le digo \"soy spidermaaaaan\" y no me esperaba su reaccion, me empezo a llamar hijo de puta, me dio tortazos como loca :S y empezo a decir que no veia, que se lo meti en el ojo, yo viendo el percal me vesti y me fui, luego me llamo y me dijo que era un hijo de puta, yo le dije que era una broma en plan colegas, pero nada, dice que tuvo que ir al medico pasando verguenza porque tenia el ojo muy irritado... y que como volviera a enviarla algo para quedar me denuncia.",
        "viernes" => "viernes",
        // "anime" => "El anime es de otakus y pringaos que no tienen nada más que hacer que sucumbir al escapismo de dibujos de personajes en 2D. Los consumidores de anime NO tienen derechos y debería recaer sobre ellos todo el peso de la ley. Si tienes avatar de anime tu opinión no solo no cuenta, si no que voy a pensar automáticamente lo contrario de lo que me digas. En definitiva el anime es basura y fumarlo es de idiotas.",
        "anime" => "No les pasa que algunos hábitos del anime se les queda?😥😥😱😱😰😓😰😓 ayer estaba comprando tortillas y una señora no se movía de la fila🙄 y le grité SHINEE😣😠‼️ me di cuenta de lo que había hecho y le dije 本当にごめんなさ (gomenasai para los que no saben🙄) y me agaché para pedirle perdón, creo que nací en el país incorrecto😅😂😂",
        "hetero" => "omgg, eres hetero???😍😍😳 Siempre quise un amigo😅 hetero🤣, yo tengo un conocido hetero🧐, telo presento alomejor y se gustan pq son 😳😳heteros😳😳fifa, fútbol🤭🤭, violencia intrafamiliar😘😘, 😏😏misoginia, 🤗🤗peluches de Stich, golpear paredes 😳",
        "tetas" => "Tetitas, tetazas, tetorras, tetotas, tetarracas, tetacas, tetuzas, teturras, tetungas, tetillas, bufas, bufarras, bufarracas, bufoncias, mamelungas, mamelones, melones, domingas, bubalongas, babongas, pechugas, peras, peritas, perolas, mamellas, tetolas, gemelas, maracas, bazucas, petacas.",
        "profe" => "Hola profe, perdón por no asistir a clases, me sucedió un problema. No sé preocupe, no fue algo grande. Bueno, si tanto insiste se lo explico, el pez de mi abuela se estaba ahogando, por lo tanto esta lo sacó de la pecera, y pues el pez se murió, se lo mató el gato. Por lo tanto, mi abuela le hizo un funeral, bueno, en realidad no hizo un funeral, el pez era algo grande, sabes? se veía rico, entonces, nos comimos al pez. Pero pasó algo raro, mi primo comenzó a convulsionar, no sé por qué. Mi primo se tiró a la piscina y empezó a nadar, se convirtió en un pez, creo. Lo íbamos a llevar al medico, pero al final no lo pudimos hacer por qué se murió, tranquila, esta vez no nos comimos al muerto. Bueno, no sabíamos donde dejar el cadáver de mi primo, entonces mi abuela propuso una idea \"si no hay cuerpo no hay muerto\" por lo tanto nos comimos a mi primo (he de decir que sabe mal, normal, estaba quemado). Bueno, enterramos los huesos para fingir que eran de un cavernicola. Funcionó, vino un arqueólogo, este se dio cuenta de que eran falsos, lo sobornamos al arqueólogo y ahora mi primo se encuentra en el museo y nosotros con mucho dinero. Por lo tanto, quiero decirle que si no me aprueba la materia, me la comeré (a usted, no a la materia) y la pondré en un museo y ganaré mucho dinero.",
        "peruano" => "Malditos peruanos, los odio, son como venezolanos pero con fetiches raros con aves. Estoy seguro de que no soy la única persona que piensa esto, odio a mi abuela peruana, a la vez la quiero, pero deseo su muerte, todo es su culpa. Maldigo el día en el que la sangre peruana se metió dentro de mi cuerpo y me infectó de por vida. Estaría dispuesto a hacerme innumerables transfusiones de sangre solo para eliminar este veneno que reside dentro de mi. Si eres peruano y estás leyendo esto, no te preocupes, prometo que algún día encontraré la cura a esta grave enfermedad.",
        "abogado" => "El que tengo aqui colgado xd",
        "amogus" => "ඞ",
        "novia" => "Salir con una mujer es 50% gay porque te gusta una mujer sin embargo haces cosas de mujeres porque pasas tiempo con ella. Por el contrario uno podría pensar que salir con un hombre es gay pero no es solo 50% gay porque te gusta un hombre pero haces cosas de hombre cuando estás con un hombre. Salir con un trapito es el culmine de las relaciones gays porque no solo te gusta un hombre sino que hace cosas femeninas haciéndote 110% gay, en cambio salir con una tomboy es 100% heterosexual y lo más hetero que podrías hacer porque te gusta una mujer y cuando estas con ella haces cosas de hombres. En conclusión prefiero a los perros callejeros",
        "pene" => "Pene, pichula, pirulin, tula, Dedo sin uña, Tampon de carne, Corneta, Falo, Casco aleman, Callampa, Microfono, Nutria, Palo, Palo rico, Prieta del mal, Chupete de cuero, platano, El niño, Cabeza de habba, Darth vader, Chino tuerto, Puñal de carne, Cabeza de griffo, El mani, Cheesito, Wañaño, Guanaco, Pirulin, Pico de tucan, Coyac, Salchicha, Cogote de ganso, Cuchufli,El pelado, El pate, La coronta, El lapiz, Vitamina P, Cogote de jirafa, La perforadora, Pilin, Manguera, Sable lacer, Zanahoria pelada, Vaginometro, La que escupe, La colorada, Flauta dulce,Cabeza de gato, Cacahuate, El olimpo, Arrollado de primavera, Julio martines, Vin disel, La rocka, El dedo 21, Picota, Palanca de cambio, Pistola de quaker, Pistola, Uvre, El que llora,Boca de pescadito, El huachalomo, El fusil, La cosa, El venoso, El siclope, El mocoso, El fabrizio, El fideo, Pepino, Teta del juicio, El cavernoso, La puntita, El tubo, El amigo de manuelaEl puro de carne, Guaton con leche, Chulapi, Narigon, El explorador, El amante de tu hermana, La que se enchufa, El que se interna, El regalon, El flecha, Cabro chico, La sin hueso,Caramelo, Aparato, El pedazo, El ciclope sentimental, Anaconda, Bin bin, Brazo de niño, Cachano, El cabezon, El chorizo, El guevara, El loco, Flipper, Macana, Machete, Monda,Pajarito, Paloma, Paquete, Paraiso, Perinola, Pipi, Piringallo, Piripicho, Pirulo, Pitillo de cuero, Polla, Trompa de elefante, Trozo, Reata, Chile, Cola, Picha, Bicho, Picho, Chimbo, Turca,Garrote, Paloma, Garcha, Chota, Goma, Ripio, Ñema, Cañón, el pueblo, guaripolo, el que te conte, matasapos, este otro, inyeccion de carne, penicillin, Capsula, el olimpo, el puro de carne,Copi, el que entra y sale, pitufo, mocofono, el moquiensefalo, pichulazo, el bebé sin dientes, Amigo, Arrollado de vena, Cabeza de pala, Cabeza de papa,Cacho de paragua, Chacal de la zorra, Chino con beatle, Chuto, Cogote de pavo, Copi, Diuca, Escupe sémola, Estaca, Guañañen, Guarifaifa, Iñi piñ, La que cuelga, La que flota, La que preña, El llora espeso, Loly, Longaniza, Maceta, Mangarria, Manguaco, Mazo, Meon, Nabo, Ñafle, Ñuflo, Obelisco de carne, Pajaritom, Palo del amor, Pata de chancho,Pata guacha, Pelao con 2 garrafas, Penca, Piniwin, Presa, Saca caca, Socio, Swancito, Taladro de carne, Tercera pierna, Tilín, Tolín, Tranca, Tripa viva, Tronco, Troncoso, Vengador calvo,Vienesa, chimpandolfo, Chorra, Cipote, Cimbel, Cuca, Manubrio, Minda, Minga, Nabo, Nardo, Paquete, Picha, Babosa, Taco de vena, Chuzo, Este, Tranca, Canelón, Manguapo, Güevopalo, Camote,El willy, La ñonga, Mazacuata, Penelope, Elsa berico, El piolin, la maciza, La abuela elastica, El raton con capucha, El encapuchao.",
        "xd" => "Cada vez que algún internauta escribe su xd al final de una frase me dan ganas de darle una trompada, es que exageran y lo hacen siempre como si no pudieran vivir sin eso ¿No sé cansan? Maduren. Lo mismo pasa con esa carita :V Es una de las aberraciones más feas que tienen los hispanoamericanos cuando se expresan en lo chats o comentarios de internet.",
        "porno" => "La pornografía se apodero de mí. Ya no me masturbo por placer, sino por rutina o por aburrimiento. Por mi adicción ya no me es tan fácil satisfacerme con cualquier vídeo. Más de una vez he llegado a buscar cosas de las que no me enorgullezco. El tiempo empleado en satisfacer mis ansias sexuales es incalculable. A día de hoy, después de haber desperdiciado tanto tiempo valioso, me arrepiento de haber tenido curiosidad cuando era un niño. Parece que al final, la curiosidad sí mato al gato...",
        "sexo" => "Sexo Coito Meneo Hacer el amor Cuchiplancheo Sinrespeto Sindistancia El delicioso El acto El frutifantastico El frutidelicioso Intercambio de energías Momento horno Acto fogoso El metisaca El conocimiento El cuchicuchi El acompañame por mi cargador El poder Garchar Coshar Correr con chanclas Contacto El empanizamiento La clavada Boom boom Ir a pescar Hacerle como el auto de Jesse pinkman Momento hoteweels El muele carne El pecaminoso Echar un caldo Echar un palito Palenque La fusión Despeinar a la cotorra Mojar el churro El brinca brinca El ñiki ñiki Ir tras bambalinas Comerte el frijol Tronar el cacahuate Ponerle crema a la empanada Un parchís",
        _ => ""

    }
}
