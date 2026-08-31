package slackblocks

// NewAttachment creates a legacy secondary message attachment.
func NewAttachment() *AttachmentBuilder {
	return newAttachmentBuilder(newBuilder("Attachment", "").withTransform(func(object Object) (Object, error) {
		color, ok := object["color"].(string)
		if !ok || color == "good" || color == "warning" || color == "danger" || (len(color) > 0 && color[0] == '#') {
			return object, nil
		}
		if len(color) == 6 {
			object["color"] = "#" + color
		}
		return object, nil
	}))
}

// NewMessage creates a payload for chat.postMessage and related Web API methods.
func NewMessage() *MessageBuilder {
	builder := newBuilder("Message", "")
	builder.values["mrkdwn"] = true
	builder.values["text"] = ""
	return newMessageBuilder(builder)
}

// NewMessageResponse creates an immediate interaction-response payload.
func NewMessageResponse() *MessageResponseBuilder {
	builder := newBuilder("MessageResponse", "")
	builder.values["mrkdwn"] = true
	builder.values["text"] = ""
	builder.values["replace_original"] = false
	builder.values["response_type"] = "in_channel"
	return newMessageResponseBuilder(builder)
}

// NewWebhookMessage creates an incoming-webhook or response-URL payload.
func NewWebhookMessage() *WebhookMessageBuilder {
	return newWebhookMessageBuilder(newBuilder("WebhookMessage", ""))
}

// NewModal creates a modal view payload.
func NewModal() *ModalBuilder {
	return newModalBuilder(newBuilder("Modal", "modal").
		coerce("title", plainTextLike).
		coerce("close", plainTextLike).
		coerce("submit", plainTextLike))
}

// NewHomeTab creates an App Home tab view payload.
func NewHomeTab() *HomeTabBuilder { return newHomeTabBuilder(newBuilder("HomeTab", "home")) }
