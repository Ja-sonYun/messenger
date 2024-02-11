from pydantic import BaseModel as _BaseModel
from pydantic import ConfigDict
from pydantic.alias_generators import to_camel


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
