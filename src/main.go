package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"time"

	"adrianvillanueva997/deficienteBot/src/metrics"
	"adrianvillanueva997/deficienteBot/src/routines"
	"adrianvillanueva997/deficienteBot/src/services"

	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api"
	"github.com/joho/godotenv"
	"github.com/robfig/cron/v3"
)

func main() {
	_ = godotenv.Load()
	bot, err := tgbotapi.NewBotAPI(os.Getenv("telegram_bot"))
	if err != nil {
		log.Panic(err.Error())
	}
	bot.Debug = false

	log.Printf("Bot authorized on account %s", bot.Self.UserName)

	u := tgbotapi.NewUpdate(0)
	u.Timeout = 60
	updates, _ := bot.GetUpdatesChan(u)

	// Birthdays go here
	events := cron.New()
	thursday := cron.New()
	go metrics.Metrics()
	_, err = events.AddFunc("00 0 * * *", func() {
		event := routines.CheckEvents()
		if event != nil {
			message := tgbotapi.NewMessage(-1001063900471, *event)
			_, _ = bot.Send(message)
		}
	})
	if err != nil {
		log.Println((err.Error()))
	}

	// Happy thursday goes here
	_, err = thursday.AddFunc("0 0 * * 4", func() {
		thursday_message := routines.HappyThursday()
		message := tgbotapi.NewMessage(-1001063900471, *thursday_message)
		_, _ = bot.Send(message)
	})

	go events.Start()
	go thursday.Start()
	if err != nil {
		log.Println(err.Error())
	}
	for update := range updates {
		if update.Message == nil {
			continue
		}
		//tgbotapi.NewDeleteMessage(update.Message.Chat.ID, update.Message.MessageID)
		msg := tgbotapi.NewMessage(update.Message.Chat.ID, update.Message.Text)
		msg.ReplyToMessageID = update.Message.MessageID
		messageText := strings.ToLower(update.Message.Text)

		// twitter to vxtwitter goes here
		if services.Is_url(update.Message.Text) {
			url := services.Update_vx_twitter(update.Message.Text)
			if url != nil {
				config := tgbotapi.NewDeleteMessage(
					update.Message.Chat.ID,
					update.Message.MessageID,
				)
				_, _ = bot.DeleteMessage(config)
				message := fmt.Sprintf("(@%v) \n %v", update.Message.From.UserName, *url)
				msg := tgbotapi.NewMessage(update.Message.Chat.ID, message)
				_, _ = bot.Send(msg)
			}
		}
		// Bad words check like uwu/owo/:v/:3
		badWords := services.Message(strings.ToLower(messageText))
		if badWords != nil {
			msg.Text = *badWords
			_, _ = bot.Send(msg)
		}
		// Numerical checks go here
		funnyNumbers := services.NumberText(messageText)
		if funnyNumbers != nil {
			msg.Text = *funnyNumbers
			_, _ = bot.Send(msg)
		}

		// Copypasta checks go here
		copyPasta := services.PastaText(messageText)
		if len(copyPasta) != 0 {
			for i := 0; i < len(copyPasta); i++ {
				msg.Text = *copyPasta[i]
				_, _ = bot.Send(msg)
			}
		}

		// Javi checks go here
		if update.Message.Chat.ID == 447988998 {
			javi, deficiente := services.CheckJavi()
			if javi {
				msg.Text = *deficiente
				_, _ = bot.Send(msg)
			}
		}
		weekday := time.Now().Weekday()
		if int(weekday) == 4 {
			textToCompare := "gracias asuka"
			if messageText == textToCompare {
				msg.Text = "De nada maquina, que tengas un feliz jueves"
				_, _ = bot.Send(msg)
			}
		}
	}
}
