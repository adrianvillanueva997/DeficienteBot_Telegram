require('dotenv').config()
const TelegramBot = require('node-telegram-bot-api');


// replace the value below with the Telegram token you receive from @BotFather
// Create a index that uses 'polling' to fetch new updates
const index = new TelegramBot(process.env.key, {polling: true});

// Listen for any kind of message. There are different kinds of
index.on('message', (msg) => {
    console.log(msg)
    const chatId = msg.chat.id;

    const lower_message = msg.text.toLowerCase()
    const pattern_1 = new RegExp("(?<!\\S)(\:v)(?!\\S)")
    const pattern_2 = new RegExp("\\b\\w*uwu|w*owo|(:v)|(:3)|(v:)\\w*\\b")
    const pattern_3 = new RegExp("\\b\\w*dvd\\w*\\b")
    const dospuntosuve = pattern_1.exec(lower_message)
    const palabrasconcretas = pattern_2.exec(lower_message)
    const javi = pattern_3.exec(lower_message)
    if (javi != null && dospuntosuve != null || javi != null && palabrasconcretas != null) {
        index.sendMessage(chatId, '@Dvdgg deficiente y pasate a Windows 10', {reply_to_message_id: msg.message_id});
    } else if (dospuntosuve != null || palabrasconcretas != null) {
        index.sendMessage(chatId, 'Deficiente', {reply_to_message_id: msg.message_id});

    } else if (javi != null) {
        index.sendMessage(chatId, '@Dvdgg formatea y pasate a Windows 10', {reply_to_message_id: msg.message_id});
    }
    if (msg.from.id == 300949) {
        const random_number = getRandomInt(0, 1000);
        console.log(random_number)
        if (random_number > 322 && random_number < 400) {
            index.sendMessage(chatId, 'Deficiente', {reply_to_message_id: msg.message_id});
        }
    }

});

function getRandomInt(min, max) {
    return Math.floor(Math.random() * (max - min)) + min;
}
