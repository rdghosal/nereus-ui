import { WebSocket } from "ws";

const ws = new WebSocket("ws://127.0.0.1:8080/ws/");
ws.on("open", function () {
  ws.send("class Test(pydantic.BaseModel):\n\n    id: int");
});
ws.on("message", function (msg) {
  console.log(msg.toString());
});
