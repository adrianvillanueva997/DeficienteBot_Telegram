require('dotenv').config()
const TelegramBot = require('node-telegram-bot-api');
const cron = require('node-cron')

// replace the value below with the Telegram token you receive from @BotFather
// Create a index that uses 'polling' to fetch new updates
const bot = new TelegramBot(process.env.key, {polling: true});
const d = new Date();
var date_format_str = d.getFullYear().toString() + "-" + ((d.getMonth() + 1).toString().length == 2 ? (d.getMonth() + 1).toString() : "0" + (d.getMonth() + 1).toString()) + "-" + (d.getDate().toString().length == 2 ? d.getDate().toString() : "0" + d.getDate().toString()) + " " + (d.getHours().toString().length == 2 ? d.getHours().toString() : "0" + d.getHours().toString()) + ":" + ((parseInt(d.getMinutes() / 5) * 5).toString().length == 2 ? (parseInt(d.getMinutes() / 5) * 5).toString() : "0" + (parseInt(d.getMinutes() / 5) * 5).toString()) + ":00";

console.log("Bot running! " + date_format_str)
// Listen for any kind of message. There are different kinds of
bot.on('message', (msg) => {
    console.log(msg)
    const chatId = msg.chat.id;
    console.log(msg.chat.id);
    const lower_message = msg.text.toLowerCase()
    const pattern_1 = new RegExp("(?<!\\S)(\:v)(?!\\S)")
    const pattern_2 = new RegExp("\\b\\w*uwu|w*owo|(:v)|(:3)|(v:)\\w*\\b")
    const pattern_3 = new RegExp("\\b\\w*dvd\\w*\\b")
    const pattern_4 = new RegExp("\\b\\w*cbt\\w*\\b")
    const dospuntosuve = pattern_1.exec(lower_message)
    const palabrasconcretas = pattern_2.exec(lower_message)
    const javi = pattern_3.exec(lower_message)
    const cbt = pattern_4.exec(lower_message)
    if (javi != null && dospuntosuve != null || javi != null && palabrasconcretas != null) {
        bot.sendMessage(chatId, '@Dvdgg deficiente y pasate a Windows 10', {reply_to_message_id: msg.message_id});
    } else if (dospuntosuve != null || palabrasconcretas != null) {
        bot.sendMessage(chatId, 'Deficiente', {reply_to_message_id: msg.message_id});

    } else if (cbt != null) {
        bot.sendMessage(chatId, 'Cock and ball torture (CBT) is a sexual activity involving application of ' +
            'pain or constriction to the male genitals. This may involve directly painful activities, such as wax play, ' +
            'genital spanking, squeezing, ball-busting, genital flogging, urethral play, tickle torture, erotic electrostimulation or even ' +
            'kicking. The recipient of such activities may receive direct physical pleasure via masochism, ' +
            'or emotional pleasure through erotic humiliation, or knowledge that the play is pleasing to a sadistic dominant. ' +
            'Many of these practices carry significant health risks.', {reply_to_message_id: msg.message_id});
    }
    if (msg.from.id == 300949) {
        const random_number = getRandomInt(0, 1000);
        console.log(random_number)
        if (random_number > 322 && random_number < 420) {
            bot.sendMessage(chatId, 'Deficiente', {reply_to_message_id: msg.message_id});
        }
    }
});

function getRandomInt(min, max) {
    return Math.floor(Math.random() * (max - min)) + min;
}

cron.schedule("0 0 12 10 *", () => {
    bot.sendMessage(-1001063900471, "🇪🇸 Feliz dia de Españita 🇪🇸")
})

cron.schedule("0 0 1 * *", () => {
    const date = new Date()
    const month = date.getMonth()
    switch (month) {
        case 0:
            bot.sendMessage(-1001063900471, "Feliz año nuevo chavalotes")
            break;
        case 1:
            bot.sendMessage(-1001063900471, "Feliz mes de Febrero chavalotes")
            break;
        case 2:
            bot.sendMessage(-1001063900471, "Feliz mes de Marzo chavalotes")
            break;
        case 3:
            bot.sendMessage(-1001063900471, "Feliz mes de Abril chavalotes")
            break;
        case 4:
            bot.sendMessage(-1001063900471, "Feliz mes de Mayo chavalotes")
            break;
        case 5:
            bot.sendMessage(-1001063900471, "Feliz mes de junio chavalotes, que empieza el veranito")
            break;
        case 6:
            bot.sendMessage(-1001063900471, "Feliz mes de Julio chavalotes")
            break;
        case 7:
            bot.sendMessage(-1001063900471, "Feliz mes de Agosto chavalotes, a morir de caloret")
            break;
        case 8:
            bot.sendMessage(-1001063900471, "Feliz mes de Septiembre chavalotes, la vuelta al cole")
            break;
        case 9:
            bot.sendMessage(-1001063900471, "Feliz mes de octubre chavalotes, ")
            break;
        case 10:
            bot.sendMessage(-1001063900471, "Queda inaugurada la temporada de No Fap November")
            break;
        case 11:
            bot.sendMessage(-1001063900471, "Feliz mes de Diciembre chavalotes, abrigaos que ya hace fresco")
            break;
        default:
            break;
    }
})
// cumpleaños javi
cron.schedule("0 0 30 1 *", () => {
    bot.sendMessage(-1001063900471, "@lilnarwhal Deficiente")
})

cron.schedule("0 0 7 2 *", () => {
    bot.sendMessage(-1001063900471, "@joseawe Deficiente")
})

cron.schedule("0 0 26 2 *", () => {
    bot.sendMessage(-1001063900471, "@thedrdvd Deficiente")
})

cron.schedule("0 0 4 5 *", () => {
    bot.sendMessage(-1001063900471, "@r3dmsr Deficiente")
})

cron.schedule("0 0 9 5 *", () => {
    bot.sendMessage(-1001063900471, "@thexiao77 Deficiente")
})

cron.schedule("0 0 8 7 *", () => {
    bot.sendMessage(-1001063900471, "@sanz97xx Deficiente")
})

cron.schedule("0 0 2 8 *", () => {
    bot.sendMessage(-1001063900471, "@sauturn Deficiente")
})

cron.schedule("0 0 7 10 *", () => {
    bot.sendMessage(-1001063900471, "@txc450 Deficiente")
})

cron.schedule("0 0 16 10 *", () => {
    bot.sendMessage(-1001063900471, "@DarkTrainer Deficiente")
})
