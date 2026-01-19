use std::{collections::HashMap, sync::LazyLock};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tracing::instrument;

static COPYPASTA_MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("cbt", "Cock and ball torture (CBT), penis torture or dick torture is a sexual activity involving application of pain or constriction to the penis or testicles. This may involve directly painful activities, such as genital piercing, wax play, genital spanking, squeezing, ball-busting, genital flogging, urethral play, tickle torture, erotic electrostimulation, kneeing or kicking.");

    m.insert("tetas", "tetas tetitas tetazas tetorras tetotas tetarracas tetacas tetuzas teturras tetungas tetillas bufas bufarras bufarracas bufoncias bakugans mamelungas mamelones melones domingas bubalongas babungas pechugas peras peritas perolas mamellas tetolas gemelas");

    m.insert("vagina", "Tener sexo con una vagina es para maricas absolutos. ¿Sabes lo que hace un hombre de verdad? Como verdadero gangsta que soy, doy y recibo por el culo. Eso es lo que los verdaderos G hacen con sus hermanos. Te diré algo. Puedo ser conocido como Top G en público, pero aquí en mi celda en Rumania, soy conocido como el Bottom G. Así es como rodamos. No puedo pensar en una actividad de menor retorno de la inversión que meter la polla en el coño. Es una completa pérdida de tiempo y energía. Piénsalo por un segundo. Esas putas han cogido literalmente millones de pollas. Simplemente no es lo suficientemente apretado para mí. Pero taladrar el culo de un compañero G, esa es una historia completamente diferente. No solo estás proporcionando un montón de estímulo para tu pene, sino que estás ampliando activamente tu red al mezclarte con personas de ideas afines. Todo forma parte de llegar a la cima. Y ese es, señoras y señores, el secreto para escapar de la Matrix.");

    m.insert("porno","Listo ya está, me vi todos los videos que existieron y van a existir, me era imposible dejarlo y dije ah si? y me puse a ver todos los videos que había a diestro y siniestro me clavé 87 pajas y media y me dolía la cabeza de tanta masacraduría, si me veian por la calle me hubiesen visto completamente de lado y con cara de zombie. Ahora empecé a cosificar todo, tipo cualquier persona que veía me la imaginaba en bolas y la ultima vez que habían garchado, las caras que ponía y tal. Ahora estoy completamente seco y me da asco una mujer (mentira) pero estoy re empachado y desprovisto de energía. Ahora solo quiero ser un monje e irme al tibet.");

    // m.insert("pan"," Pues resulta que he ido a comprar el pan a un moro de estos guays que me cobran 50 centimos por la barra de pan sus cojones ahi y entonces entro y le digo ey que pasa como va alqaeda bien?como va la financiada jajajaaj como soy bromista pues pa reirme y eso porque siempre se lo decia al moro de mi barrio pues coje este moro que era nuevo y me dice asalam juaralam massan o algo asi raro y le digo yo abracadabra abracadabra mesaguaope mechami its cheaaaaaa asi con el tono del cd makina total o los antiguos lo recodais como lo mas duro ajjajaa sa quedao flipando el moro y mañana ire otra vez asi con mi humor gracioso porque tenemos que reinrnos de algo si no te rayas en casa y sin hacer na jajjajajajaja ");
    m.insert("aceitunas"," Ratatatatatatatapumpumpumpum!!! Estaba yo el otro dia comprando en la mercadona cuando de repente abro una lata de aceitunas porque tenia to el picadon y paaaaaaaaaam aparece un genio que me dice loco loco soy el puto genio de las aceitunas, con una pinta de colgao que flipas yo ya pensaba que se me estaba subiendo el tripi y va el pavo y me dice te concedo tres deseosssss. Y yo buabauabauabauabauaabuaaaaaaa obviamente le pedí un kebap mixto sin verdura y me lo estaba comiendo y de dentro me sale otro genio y yo joder joder un genio moruno y me dice que me concede otros 3 deseos y yo joder joder joderrrrrrrrr, me pare a pensar y dije, cuantos deseos me quedan? digo 5, y de pronto aparese una cajera del mercadona to shula y me dice por el culo te la hinco jajajajaajaja y ahora estoy en el hospital con una fractura anal.No vuelvo a comprar aceitunas en la puta vida oye. ");

    m.insert("gimnasio"," Menudo hijo de puta, tienes un cuerpazo del copón. De gordo nada, tu estás fuerte. Menudas espaldas tienes, hijo de la gran puta. Olvídate de adelgazar, potencia ese físico tan espectacular que tienes, hijo de una perra sarnosa. Qué puta envidia me das! Cuida un poco la alimentación y tira de máquinas en el gim. Natación y remo es lo que necesitas para terminar de ponerte del copón. Menudo hijo de la gran puta eres, ya quiesiera yo ese pedazo de cuerpo. Cabronazo, hijo de mil putas, te invidio muchísimo. Insisto, cuida un poco la dieta y machácate en el gim, con ese cuerpo puedes quedarte del copón, cabronazo. Y a los que dicen que estás gordo ni puto caso, son un atajo de maricones insecto-palos que solo les gusta lamer falos. Tu, sin embargo, estás del copón, hijo de la grandísima puta. Qué suerte tenéis algunos con la genética. ");

    m.insert("me cago en tus muertos", "me cago en todos los muertos de tu arbol genialogico y si me apuras tambien en los vivos , puto amorfo de mierda te pillo por la calle y te hundo el pexo a martillazos , enfermo ijodelagran puta , si tienes hijos espero que tengan alguna discapacidad fisica o mental o en su defecto los atropelle un autobus , pero que no mueran que sufran toda su puta vida y si no tienes hijos nunca que sera lo que pasara seguramente dios te bendiga con una gordaka puto follapinos ijodelagranputa , como me entere de que me agregas otra vez con alguna cuenta asi y me entere de quien eres te juro que te busco , cuando te encuentre te voy quitando partes de tu ridiculo cuerpo y me las voy comiendo y mientras me las como las cagare y te haré comer mis putas heces con trozacos de tu piel rebozados y cuando ya te haya destripado completamente y averte hecho comer toda la mierda que suelte de mi precioso y brillante culo ire a por tu hermana y si no tienes hermana ire a por la sudada de tu puta madre y si voy inspirado ire a por las dos , la secuestrare , las metere en una furgoneta , las llevare a una habitacion , las metere el rabo por todos los agujeros de su cuerpo (si , incluidos los de la nariz y orejas) me corre dentro de ellas y esperare 9 meses a que nazcan sus hijas y cuando cumplan 13 años me las follare tambien y si aun asi despues de eso te siguen quedando primas o tias hare lo mismo con ellas y cuando ya este cansado de follarme a toda tu familia de piojosos cojere unas cuantas cadenas las pondre en mi coche y recorrere 300km con toda la tu familia enganchada a ellas y si despues de eso queda alguien vivo , le hecho alcohol para que rabie aun mas de dolor y despues de todo eso ire al hospital cuando ya te hayas recuperado de el destripamiento que te hice te sacare de hay te llevare a la misma habitacion donde me folle a todas las mujeres de tu actual familia y a las que preceden en tu arbol genialogico y mientras te pongo los videos de como me follaba a tu madre te dare minipollazos en la frente hasta que se te quede la marca de mi grande y devastador glande para el resto de tu vida y asi cada vez que te mires en el espejo recordaras esos videos y lo que hice con tu familia , despues de eso te soltare y volvere a ir a por ti a los tres meses , te volvere a meter en la habitacion , pero esta vez nada suave , esta vez cojere tus manos y empezare a meterte agujas entre las uñas hasta que el nivel de dolor te haga desmayarte y te reanimare con un desfibrilador , te bajare los pantalones y los calzoncillos y empezare a darte minimatillazos en tus cojones hasta que poco a poco se vayan deshaciendo y tu escroto quede completamente vano , imagino que despues de eso te desmayaras otra vez , pues volvere a usar el desfibrilador para reanimarte y metere tus pies en un cubo con agua , te pondre pinzas en los pezones , pene y lengua y te dare descargas hasta que vuelvas a desmayarte , cuando lo hagas ya sabes lo que hare.. y volvere y cojere unas tenazas e ire arrancando una a una tus putas uñas pedazo de escoria , despues te tumbare , te pondre un trapo en la cara e ire hechandote en la boca agua poco a poco sin que llegues a ahogarte ... despues me ire y volvere cada a dia para hacerte una tortura diferente , para que cada vez que oyeras mis pasos acercarse a la puerta a horas diferentes cada dia , un miedo que jamas hayas experimentado recorra tu cuerpo y quedarme en la puerta haciendo como que habro hasta que te mees encima , entonces entrare y comenzare... cuando vea que ya no das para mas torturas te dejare que te cures en un hospital y volvere a ir a por ti cuando te recuperes , te cojere y mientras te quemo los ojos con un soplete te daremartillazos en la nuez hasta que mueras pedazo de hijodelagran puta pero no pienses que todo acaba a hay... si la reencarnacion existe , volvere reencarnado en culaquier otra persona y te hare a ti y a toda tu nueva familia todo lo que te hecho en esta vida pero mas lento y sin matarte para que cada vez que vieses una sombra en la noche pienses que soy yo , que te entre una locura impresionante.");

    m.insert("cigarro"," Pam pam!!!!! estaba yo ayer ahiii to locooo por la calle dando un paseo ahi to desfasao, y de repente viene una pava y me dices tienes un cigarro y le digoooo un puro habano pa tu boca putaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa aaaaaaaaaa, y se lo tomo a mal, y se lo dijo a un polisia que habia ahi al laooo y el tio me viene y me diseeeee la violamos entre los dos loco loco locooooooooooo y yo joderrrrrrr, que tiene 15 añossss jajajajaa pero al final no era un polisia, era Braulio. Ahsadishaudhusahuhuahua puto Braulio. ");

    m.insert("oreja"," Os cuento, quede con una golfilla para intimar en su pisito, empezamos normal, nos liamos y demas (ella tenia bastante interes en chuparme la oreja, a un rato me dio hasta mal rollo la cabrona). Bueno, yo no lleve cartera y claro, cuando estabamos ya muy cachondos le dije amablamente si tenia condones, me dijo que no...hice como que me molestaba...ella me dijo que yo siempre suelo llevarlos, le dije que se me olvido la cartera y demas...total que accedio a hacer la marcha atras y yo pensando 'esta es la mia, le hare el spiderman'. Estamos arreando y demas y despues d eun buen rato veo que me voy a correr, saco mi tizona, me corro en mi mano y ella se queda un poco con cara de roto2, y entonces cojo y le tiro la lefada a su cara y le digo 'soy spidermaaaaan' y no me esperaba su reaccion, me empezo a llamar hijo de puta, me dio tortazos como loca :S y empezo a decir que no veia, que se lo meti en el ojo, yo viendo el percal me vesti y me fui, luego me llamo y me dijo que era un hijo de puta, yo le dije que era una broma en plan colegas, pero nada, dice que tuvo que ir al medico pasando verguenza porque tenia el ojo muy irritado... y que como volviera a enviarla algo para quedar me denuncia.");

    m.insert("darth plaeguis", "Did you ever hear the tragedy of Darth Plagueis the wise? No. I thought not, It's No story the jedi would tell you. It's a sith legend. Darth Plagueis was a Dark Lord of the sith. He was so powerful, Yet so wise. He could use the force to influence the medi chlorians to create, Life. He had such a knowledge of the Dark side, He could even keep the ones he cared about, From dying. He could actually, Save the ones he cared about from death? The dark side of the force is a pathway to many abilities some consider to be unnatural. Well what happened to him? Darth Plagueis became so powerful that the only thing he feared was losing his power, Which eventually of course he did. Unfortunately, He taught his apprentice everything he knew. Then his apprentice killed him in his sleep. Ironic, He could save others from death, But not himself. Is it possible to learn this power? Not from a jedi.");
    m.insert("doot", "https://www.youtube.com/watch?v=eVrYbKBrI7o&t");

    m.insert(
        "rusia",
        "
⣿⣿⣿⣿⣻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣵⣿⣿⣿⠿⡟⣛⣧⣿⣯⣿⣝⡻⢿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⠋⠁⣴⣶⣿⣿⣿⣿⣿⣿⣿⣦⣍⢿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⢷⠄⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣏⢼⣿⣿⣿⣿
⢹⣿⣿⢻⠎⠔⣛⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⣿⣿⣿⣿
⢸⣿⣿⠇⡶⠄⣿⣿⠿⠟⡛⠛⠻⣿⡿⠿⠿⣿⣗⢣⣿⣿⣿⣿
⠐⣿⣿⡿⣷⣾⣿⣿⣿⣾⣶⣶⣶⣿⣁⣔⣤⣀⣼⢲⣿⣿⣿⣿
⠄⣿⣿⣿⣿⣾⣟⣿⣿⣿⣿⣿⣿⣿⡿⣿⣿⣿⢟⣾⣿⣿⣿⣿
⠄⣟⣿⣿⣿⡷⣿⣿⣿⣿⣿⣮⣽⠛⢻⣽⣿⡇⣾⣿⣿⣿⣿⣿
⠄⢻⣿⣿⣿⡷⠻⢻⡻⣯⣝⢿⣟⣛⣛⣛⠝⢻⣿⣿⣿⣿⣿⣿
⠄⠸⣿⣿⡟⣹⣦⠄⠋⠻⢿⣶⣶⣶⡾⠃⡂⢾⣿⣿⣿⣿⣿⣿
⠄⠄⠟⠋⠄⢻⣿⣧⣲⡀⡀⠄⠉⠱⣠⣾⡇⠄⠉⠛⢿⣿⣿⣿
⠄⠄⠄⠄⠄⠈⣿⣿⣿⣷⣿⣿⢾⣾⣿⣿⣇⠄⠄⠄⠄⠄⠉⠉
⠄⠄⠄⠄⠄⠄⠸⣿⣿⠟⠃⠄⠄⢈⣻⣿⣿⠄⠄⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⢿⣿⣾⣷⡄⠄⢾⣿⣿⣿⡄⠄⠄⠄⠄⠄⠄
⠄⠄⠄⠄⠄⠄⠄⠸⣿⣿⣿⠃⠄⠈⢿⣿⣿⠄⠄⠄⠄⠄⠄⠄
Мне насрать на твою ёбаную мать",
    );
    m.insert(
        "shrek",
        "
⢀⡴⠑⡄⠀⠀⠀⠀⠀⠀⠀⣀⣀⣤⣤⣤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠸⡇⠀⠿⡀⠀⠀⠀⣀⡴⢿⣿⣿⣿⣿⣿⣿⣿⣷⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠑⢄⣠⠾⠁⣀⣄⡈⠙⣿⣿⣿⣿⣿⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢀⡀⠁⠀⠀⠈⠙⠛⠂⠈⣿⣿⣿⣿⣿⠿⡿⢿⣆⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⢀⡾⣁⣀⠀⠴⠂⠙⣗⡀⠀⢻⣿⣿⠭⢤⣴⣦⣤⣹⠀⠀⠀⢀⢴⣶⣆
⠀⠀⢀⣾⣿⣿⣿⣷⣮⣽⣾⣿⣥⣴⣿⣿⡿⢂⠔⢚⡿⢿⣿⣦⣴⣾⠁⠸⣼⡿
⠀⢀⡞⠁⠙⠻⠿⠟⠉⠀⠛⢹⣿⣿⣿⣿⣿⣌⢤⣼⣿⣾⣿⡟⠉⠀⠀⠀⠀⠀
⠀⣾⣷⣶⠇⠀⠀⣤⣄⣀⡀⠈⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀
⠀⠉⠈⠉⠀⠀⢦⡈⢻⣿⣿⣿⣶⣶⣶⣶⣤⣽⡹⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠉⠲⣽⡻⢿⣿⣿⣿⣿⣿⣿⣷⣜⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣷⣶⣮⣭⣽⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⣀⣀⣈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠇⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠻⠿⠿⠿⠿⠛⠉",
    );
    m.insert("lentejas", "Lo mejor de las maduras, es que puedes comerles el roscón de reyes, mientras te tienen al fuego unas lentejas de puta madre. Yo recuerdo una que conocí en un eroski, y la recuerdo como uno de los mejores polvos de mi vida. Ella me hizo unos callos cojonudisisimos, y mientras los preparaba, yo le daba como un cabrón por el ojete, ya que se había puesto faldita para que fuese haciendo mientras cocinaba. Creo que eyaculé tal cantidad de esperma, que estuve dos horas inconsciente. Menos mal que los callos me dieron fuerza para acabar el día con un par más. Y tenía unos hijos majísimos. Menudos vicios echamos al crash bandicoot 2.");
    m.insert("ayuso" , "Estoy muy jodido, Ayuso me ha arruinado la puta vida, cuando me despierto lo primero que veo es un cartel de Ayuso colgado en mi pared que me pone muy cachondo pero tengo que prepararme para ir al trabajo no sin antes ir al baño para hacerme la paja con Ayuso. En trabajo voy cada dos horas a hacerme la paja con Ayuso y mientras estoy trabajando solo pienso en ella y en lo guapa que es. Cuando por fin llego a mi casa lo primero que hago es hacerme una paja con Ayuso, después me ducho ceno y me vuelvo a hacer una paja con Ayuso, luego voy a ver los mejores discursos de Ayuso y mientras me pongo cachondo al final del día me voy a la cama y me hago un par de pajas más con Ayuso y después me acuesto y me despierto a media noche así que me hago otra paja. En los fin de semanas me levanto con un charco de semen en le pantalón solo de pensar en Ayuso así que me hago otra paja con Ayuso y voy a ducharme, como y me hago una paja con Ayuso, desayuno y me hago una paja con Ayuso, ando y me hago una paja con Ayuso pajas pajas Ayuso pajas Ayuso");
    m.insert("bicho", "SIIUUUUUUUUUUUUU");
    m.insert(
        "china",
        "
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
    );
    m.insert("halloween", "Qué es Jalogüin??");
    m.insert("amiga", "\"amiga\"");
    m.insert("euskadi", "el ojo de");
    m.insert("69", "> nice");
    m.insert("viernes", "viernes");
    m.insert("amogus", "ඞ");
    m.insert(
        "america",
        "
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠛⠋⠉⡉⣉⡛⣛⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⡿⠋⠁⠄⠄⠄⠄⠄⢀⣸⣿⣿⡿⠿⡯⢙⠿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⡿⠄⠄⠄⠄⠄⡀⡀⠄⢀⣀⣉⣉⣉⠁⠐⣶⣶⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⡇⠄⠄⠄⠄⠁⣿⣿⣀⠈⠿⢟⡛⠛⣿⠛⠛⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⡆⠄⠄⠄⠄⠄⠈⠁⠰⣄⣴⡬⢵⣴⣿⣤⣽⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⡇⠄⢀⢄⡀⠄⠄⠄⠄⡉⠻⣿⡿⠁⠘⠛⡿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⡿⠃⠄⠄⠈⠻⠄⠄⠄⠄⢘⣧⣀⠾⠿⠶⠦⢳⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣶⣤⡀⢀⡀⠄⠄⠄⠄⠄⠄⠻⢣⣶⡒⠶⢤⢾⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⡿⠟⠋⠄⢘⣿⣦⡀⠄⠄⠄⠄⠄⠉⠛⠻⠻⠺⣼⣿⠟⠋⠛⠿⣿⣿
⠋⠉⠁⠄⠄⠄⠄⠄⠄⢻⣿⣿⣶⣄⡀⠄⠄⠄⠄⢀⣤⣾⣿⣿⡀⠄⠄⠄⠄⢹
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⢻⣿⣿⣿⣷⡤⠄⠰⡆⠄⠄⠈⠉⠛⠿⢦⣀⡀⡀⠄
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠈⢿⣿⠟⡋⠄⠄⠄⢣⠄⠄⠄⠄⠄⠄⠄⠈⠹⣿⣀
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠘⣷⣿⣿⣷⠄⠄⢺⣇⠄⠄⠄⠄⠄⠄⠄⠄⠸⣿
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠹⣿⣿⡇⠄⠄⠸⣿⡄⠄⠈⠁⠄⠄⠄⠄⠄⣿
⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⢻⣿⡇⠄⠄⠄⢹⣧⠄⠄⠄⠄⠄⠄⠄⠄⠘
Ã̶͉̝̯̙̕ṃ̛͎̥͙̹͓̖͐͢eͦͮͮ̃͏̟̙̗̯̫r̶̟ͩͥ̄̐̓ͧ̒̽ĩ͙̙̬̺͇̥c̲̗̣͊ͨ̂̾̎ͪ͝͠à̠͍̻̭̱̤̙̯͆̀̔̽,̵͚̘ͩ̀ f̧̋͑́҉̼͍̖u͉̰͉̱̬͕͋c̫̙͒͛ͧͤk̵͖͉͈̰͖ͫͥ̋ͪ͞ ÿ̶͔̰̫̭͇͈̻́͊̒e̩͎̖̩̠̳ͣͩ̅̎̓̉͋ͧa̼̺ͮ̕h̙͙̟́̒͛",
    );
    m
});

pub struct CopyPastaMatch {
    pub trigger: String,
    pub response: String,
}

/// Finds copypastas in a string
#[instrument]
pub async fn find_copypasta(input: &str) -> Vec<CopyPastaMatch> {
    COPYPASTA_MAP
        .par_iter()
        .filter_map(|(&trigger, &response)| {
            if input.contains(trigger) {
                Some(CopyPastaMatch {
                    trigger: trigger.to_string(),
                    response: response.to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}
