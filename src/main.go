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
		HappyThursday := routines.HappyThursday()
		message := tgbotapi.NewMessage(-1001063900471, *HappyThursday)
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
			twitter := false
			if url != nil {
				twitter = true
				config := tgbotapi.NewDeleteMessage(
					update.Message.Chat.ID,
					update.Message.MessageID,
				)
				_, _ = bot.Send(tgbotapi.NewChatAction(update.Message.Chat.ID, "writing"))
				_, _ = bot.DeleteMessage(config)
				message := fmt.Sprintf("(@%v) \n %v", update.Message.From.UserName, *url)
				msg := tgbotapi.NewMessage(update.Message.Chat.ID, message)
				_, _ = bot.Send(msg)
			}
			if !twitter {
				_, _ = bot.Send(tgbotapi.NewChatAction(update.Message.Chat.ID, "upload_video"))
				if services.CheckWebm(update.Message.Text) {
					services.Downloadwebm(update.Message.Text)
					output := "output.mp4"
					input := "output.webm"
					err = services.ConvertWebMToMP4(input, output)
					if err != nil {
						log.Println(err.Error())
					}
					_, _ = bot.Send(tgbotapi.NewVideoUpload(update.Message.Chat.ID, output))
					_ = os.Remove(output)
					_ = os.Remove(input)
				}
			}
		}

		// Bad words check like uwu/owo/:v/:3
		badWords := services.Message(messageText)
		if badWords != nil {
			msg.Text = *badWords
			_, _ = bot.Send(tgbotapi.NewChatAction(update.Message.Chat.ID, "writing"))
			_, _ = bot.Send(msg)
		}
		// Numerical checks go here
		funnyNumbers := services.NumberText(messageText)
		if funnyNumbers != nil {
			_, _ = bot.Send(tgbotapi.NewChatAction(update.Message.Chat.ID, "writing"))
			msg.Text = *funnyNumbers
			_, _ = bot.Send(msg)
		}

		// Copypasta checks go here
		copyPasta := services.PastaText(messageText)
		if len(copyPasta) != 0 {
			_, _ = bot.Send(tgbotapi.NewChatAction(update.Message.Chat.ID, "writing"))
			for i := 0; i < len(copyPasta); i++ {
				msg.Text = *copyPasta[i]
				_, _ = bot.Send(msg)
			}
		}

		if strings.Contains(messageText, "euskadi") {
			_, _ = bot.Send(tgbotapi.NewChatAction(update.Message.Chat.ID, "writing"))
			msg.Text = "el ojo de"
			_, _ = bot.Send(msg)
		}

		weekday := time.Now().Weekday()
		if int(weekday) == 4 {
			textToCompare := "gracias asuka"
			if messageText == textToCompare {
				_, _ = bot.Send(tgbotapi.NewChatAction(update.Message.Chat.ID, "writing"))
				msg.Text = services.AsukaGreetings()
				_, _ = bot.Send(msg)
			}
		}
	}
}
