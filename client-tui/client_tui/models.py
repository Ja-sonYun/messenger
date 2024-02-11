from typing import Annotated, Literal

from pydantic import BaseModel as _BaseModel
from pydantic import ConfigDict, Field
from pydantic.alias_generators import to_camel


class BaseModel(_BaseModel):
    model_config = ConfigDict(
        alias_generator=to_camel,
        populate_by_name=True,
    )


class ChatContent(BaseModel):
    type: Literal["chatContent"]
    text: str


class NewClient(BaseModel):
    type: Literal["newClient"]


class LeftClient(BaseModel):
    type: Literal["leftClient"]


class ChatChunk(BaseModel):
    session_id: str
    user_id: str
    event: Annotated[
        ChatContent | NewClient | LeftClient,
        Field(discriminator="type"),
    ]
