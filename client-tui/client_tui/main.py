from client_tui.api_requests import chat, receive_broadcast
from client_tui.models import ChatContent, LeftClient, NewClient
from textual import on, work
from textual.app import App, ComposeResult
from textual.containers import VerticalScroll
from textual.widgets import Button, Input, Log


class MessageInput(Input):
    ...


class UsernameInput(Input):
    ...


class ChannelInput(Input):
    ...


class ChatApp(App):
    def compose(self) -> ComposeResult:
        yield ChannelInput(placeholder="channel", id="channel")
        yield UsernameInput(placeholder="username")
        yield Button("Join", id="join")

        with VerticalScroll(id="chat-log"):
            yield Log()

        yield MessageInput(placeholder="Message")
        yield Button("Send", id="send")

    @on(Button.Pressed, "#join")
    def on_channel_changed(self) -> None:
        self.receive_messages(
            self.query_one(ChannelInput).value,
            self.query_one(UsernameInput).value,
        )

    @on(Button.Pressed, "#send")
    async def on_button_pressed(self) -> None:
        channel = self.query_one(ChannelInput)
        message = self.query_one(MessageInput)
        username = self.query_one(UsernameInput)

        if message.value and username.value:
            await chat(channel.value, username.value, message.value)

            # Reset message
            message.value = ""

    @work(exclusive=True)
    async def receive_messages(self, channel_id: str, user_id: str) -> None:
        async for chunk in receive_broadcast(channel_id, user_id=user_id):
            log = self.query_one(Log)
            match chunk.event:
                case ChatContent() as chat_content:
                    log.write_line(f"[{chunk.user_id}] : {chat_content.text}")
                case NewClient():
                    log.write_line(f"{chunk.user_id} has joined the chat")
                case LeftClient():
                    log.write_line(f"{chunk.user_id} has left the chat")


if __name__ == "__main__":
    ChatApp().run()
