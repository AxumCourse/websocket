import { nanoid } from "nanoid";

export const newMsg = (type: MessageType, text: string): Message => ({
  id: nanoid(),
  type,
  text,
  date: new Date(),
});

export const newErrMsg = (e: Error): Message => newMsg("ERR", e.message);

export const newInfoMsg = (text: string): Message => newMsg("INFO", text);
