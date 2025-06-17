<p align="center">
  <img src="./logo.png" alt="bunge-bits logo" />
</p>

# Bunge Bits

Bunge Bits provides convenient summaries of Kenyan National Assembly and Senate seatings, making legislative information more accessible and digestible.

## Motivations

The driving force behind Bunge Bits is to strengthen Kenya's democracy by making legislative processes more transparent and understandable to all citizens. The aim is to bridge the gap between complex government proceedings and the average Kenyan, fostering increased civic engagement and political awareness. By offering concise, easy-to-digest summaries of legislative sessions, I hope to empower citizens with the knowledge they need to participate more fully in their democracy, hold elected officials accountable, and engage in informed discussions about the issues that affect their lives.

Ultimately, Bunge Bits seeks to contribute to a more engaged, informed, and participatory democratic process in Kenya.

## Development Progress

- [x] ytInitialData parser: Parsing logic that parses scraped data from youtube
- [x] Data store: ~Sqlite~ Postgres database bindings for storing, retreivig and modifying stream data
- [x] `yt-dlp` bindings: Bindings to interact with the yt-dlp cli as well as some utilities to interact with video and vtt data
- [x] `ffmpeg` bindings: Fffmpeg bindings to complement the Ytdlp bindings for the purpose of processing audio
- [x] Stream pulse: A cron job that periodically fetches and processes streams
- [x] Stream digest: Functions that make it possible to efficiently process vtt file content in chunks
- [x] LLM Service: A service that interacts with OpenAI's ChatGPT (or any other LLM) to handle summarizing the downloaded audio
- [ ] Web App: The end user interface that will display the summarized content [#3](https://github.com/c12i/bunge-bits/issues/3)

## Blog Posts

- [Building bunge-bits, an AI-Powered Summary Pipeline for the Parliament of Kenya](https://collinsmuriuki.xyz/building-bunge-bits/)
