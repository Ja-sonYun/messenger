import requests
from pydantic import BaseModel as _BaseModel
from pydantic import ConfigDict
from pydantic.alias_generators import to_camel
from rich import print


class BaseModel(_BaseModel):
    model_config = ConfigDict(
        alias_generator=to_camel,
        populate_by_name=True,
    )


class ChatContent(BaseModel):
    text: str


class ChatChunk(BaseModel):
    session_id: str
    user_id: str
    content: ChatContent


SERVER_URL = "http://localhost:8080"


def main():
    url = f"{SERVER_URL}/chats/broadcast"
    chat = ChatChunk(
        session_id="awldkjw232",
        user_id="awldkj",
        content=ChatContent(text="2d1"),
    )
    print(chat.model_dump(by_alias=True))
    response = requests.post(url, json=chat.model_dump(by_alias=True))
    print(url)
    print(response)
    print(response.text)


main()
