import json

import httpx
import sseclient
from rich import print

url = "http://localhost:8080/chats/"
headers = {"Accept": "text/event-stream"}


def stream_events(url, headers):
    """Get a streaming response for the given event feed using httpx."""
    with httpx.stream("GET", url, headers=headers) as response:
        yield from response.iter_bytes()


client = sseclient.SSEClient(stream_events(url, headers))

for event in client.events():
    print(event.data)
