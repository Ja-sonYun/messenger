from collections.abc import AsyncGenerator

import aiohttp
from client_tui.models import ChatChunk, ChatContent

SERVER_URL = "http://localhost:8080"


async def chat(channel_id: str, username: str, message: str) -> None:
    url = f"{SERVER_URL}/chats/broadcast/{channel_id}"
    chat = ChatChunk(
        session_id="awldkjw232",
        user_id=username,
        event=ChatContent(
            type="chatContent",
            text=message,
        ),
    )

    async with aiohttp.ClientSession() as session:
        await session.post(url, json=chat.model_dump(by_alias=True))


async def receive_broadcast(
    channel_id: str,
    user_id: str,
) -> AsyncGenerator[ChatChunk, None]:
    url = f"{SERVER_URL}/chats/{channel_id}/{user_id}"
    headers = {"Accept": "text/event-stream"}

    data_prefix = "data: "

    async with aiohttp.ClientSession() as session:
        async with session.get(url, headers=headers) as response:
            async for chunk in response.content.iter_any():
                chunk_str = chunk.decode("utf-8")
                if chunk_str.startswith(data_prefix):
                    stripped_chunk = chunk_str[len(data_prefix) :].strip()
                    if not stripped_chunk.startswith("{"):
                        continue

                    yield ChatChunk.model_validate_json(stripped_chunk)
