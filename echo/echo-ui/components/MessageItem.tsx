import React from "react";

export default function MessageItem({ msg }: { msg: Message }) {
  let css = "";
  switch (msg.type) {
    case "SEND":
      css = "before:content-['发送']  before:bg-cyan-500 ";
      break;
    case "RECV":
      css = "before:content-['接收'] before:bg-green-500 ";
      break;
    case "ERR":
      css = "before:content-['错误'] before:bg-red-500";
      break;
    case "INFO":
      css = "before:content-['提示'] before:bg-gray-500";
      break;
  }
  return (
    <li
      className={`my-3 before:text-[0.65rem] before:mr-2 before:rounded before:p-1 before:text-white ${css}`}
    >
      <span className="text-gray-400 text-sm">{timeFormatter(msg.date)}</span>
      <span className="text-gray-400 text-xs mr-2 before:content-['.']">
        {paddingZero(msg.date.getMilliseconds(), 3)}
      </span>
      <span>{msg.text}</span>
    </li>
  );
}

function timeFormatter(dt: Date) {
  const h = dt.getHours();
  const m = dt.getMinutes();
  const s = dt.getSeconds();

  const [hs, ms, ss] = [h, m, s].map((i) => paddingZero(i, 2));

  return `${hs}:${ms}:${ss}`;
}

function paddingZero(i: number, len: number) {
  const istr = `${i}`;
  if (istr.length >= len) {
    return istr;
  }
  const j = len - istr.length;
  let zeroPrefix = "";
  for (let i = 0; i < j; i++) {
    zeroPrefix += "0";
  }
  return `${zeroPrefix}${istr}`;
}
