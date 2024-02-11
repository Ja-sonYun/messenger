from client_tui.api_requests import chat, receive_broadcast
from textual import work
from textual.app import App, ComposeResult
from textual.containers import VerticalScroll
from textual.widgets import Button, Input, Log


class MessageInput(Input):
    ...


class UsernameInput(Input):
    ...


class ChatApp(App):
    def on_mount(self) -> None:
        self.receive_messages()

    def compose(self) -> ComposeResult:
        yield UsernameInput(placeholder="username")

        with VerticalScroll(id="chat-log"):
            yield Log()

        yield MessageInput(placeholder="Message")
        yield Button("Send")

    async def on_button_pressed(self, _: Button.Pressed) -> None:
        message = self.query_one(MessageInput)
        username = self.query_one(UsernameInput)

        if message.value and username.value:
            await chat(username.value, message.value)

            # Reset message
            message.value = ""

    @work(exclusive=True)
    async def receive_messages(self) -> None:
        async for chunk in receive_broadcast():
            log = self.query_one(Log)
            log.write_line(f"[{chunk.user_id}] : {chunk.content.text}")


if __name__ == "__main__":
    ChatApp().run()
