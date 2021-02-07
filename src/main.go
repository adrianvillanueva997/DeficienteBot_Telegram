package main

import (
	"log"
	"os"
	"strings"

	"adrianvillanueva997/deficienteBot/src/routines"
	"adrianvillanueva997/deficienteBot/src/services"
	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api"
	"github.com/joho/godotenv"
	"github.com/robfig/cron/v3"
)

func main() {
	_ = godotenv.Load()
	bot, err := tgbotapi.NewBotAPI(os.Getenv("key"))
	if err != nil {
		log.Panic(err.Error())
	}
	bot.Debug = false

	log.Printf("Bot authorized on account %s", bot.Self.UserName)

	u := tgbotapi.NewUpdate(0)
	u.Timeout = 60
	updates, _ := bot.GetUpdatesChan(u)

	// Routines startup goes here
	job := cron.New()
	_, err = job.AddFunc("0 00 * * *", func() {
		event := routines.CheckEvents()
		if event != nil {
			message := tgbotapi.NewMessage(-1001063900471, *event)
			_, _ = bot.Send(message)
		}
	})
	go job.Start()
	if err != nil {
		log.Println(err.Error())
	}
	for update := range updates {
		if update.Message == nil {
			continue
		}
		msg := tgbotapi.NewMessage(update.Message.Chat.ID, update.Message.Text)
		msg.ReplyToMessageID = update.Message.MessageID
		messageText := strings.ToLower(update.Message.Text)
		// Bad words check like uwu/owo/:v/:3
		badWords := services.Message(strings.ToLower(messageText))
		if badWords != nil {
			msg.Text = *badWords
			_, _ = bot.Send(msg)
		}
		// Copypasta checks go here
		copyPasta := services.PastaText(messageText)
		if copyPasta != nil {
			msg.Text = *copyPasta
			_, _ = bot.Send(msg)
		}
		// Javi checks go here
		if update.Message.Chat.ID == 300949 {
			javi, deficiente := services.CheckJavi()
			if javi {
				msg.Text = *deficiente
				_, _ = bot.Send(msg)
			}
		}
	}
}
