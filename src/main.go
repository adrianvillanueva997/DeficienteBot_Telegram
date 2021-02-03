package main

import (
	"log"
	"os"
	"strings"

	"adrianvillanueva997/deficienteBot/src/services"
	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api"
	"github.com/joho/godotenv"
)

func main() {
	_ = godotenv.Load()
	bot, err := tgbotapi.NewBotAPI(os.Getenv("botkey"))
	if err != nil {
		log.Panic(err.Error())
	}
	bot.Debug = false

	log.Printf("Bot authorized on account %s", bot.Self.UserName)

	u := tgbotapi.NewUpdate(0)
	u.Timeout = 60

	updates, _ := bot.GetUpdatesChan(u)
	for update := range updates {
		if update.Message == nil {
			continue
		}
		log.Printf("[%s] %s", update.Message.From.UserName, update.Message.Text)
		msg := tgbotapi.NewMessage(update.Message.Chat.ID, update.Message.Text)
		msg.ReplyToMessageID = update.Message.MessageID
		// Bad words check like uwu/owo/:v/:3
		badWords := services.Message(strings.ToLower(update.Message.Text))
		if badWords != nil {
			msg.Text = *badWords
			_, _ = bot.Send(msg)
		}
		//Copypasta checks go here

	}
}
