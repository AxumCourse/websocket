type MessageType = "SEND" | "RECV" | "ERR" | "INFO";
type Message = {
  type: MessageType;
  id: string;
  text: string;
  date: Date;
};
