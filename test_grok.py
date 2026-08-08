import os
from openai import OpenAI

client = OpenAI(
    api_key=os.environ.get("XAI_API_KEY"),
    base_url="https://api.x.ai/v1",
)

completion = client.chat.completions.create(
    model="grok-4.5",
    messages=[
        {"role": "system", "content": "You are a tech lead assistant."},
        {"role": "user", "content": "Hola Grok, confirma que estás activo."},
    ],
)

print(completion.choices[0].message.content)
