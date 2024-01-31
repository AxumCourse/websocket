"use client";

import Box from "@/components/Box";
import MessageItem from "@/components/MessageItem";

import { newInfoMsg, newMsg } from "@/utils/message";
import React, { useCallback, useEffect, useState } from "react";

export default function HomeView() {
  const [isConnected, setConnected] = useState(false);
  const [url, setUrl] = useState("ws://127.0.0.1:56789/ws");
  const [msgs, setMsgs] = useState<Message[]>([]);
  const [ws, setWs] = useState<WebSocket | undefined>(undefined);
  const [lastMsg, setLastMsg] = useState<Message | undefined>(undefined);
  const [msgForSend, setMsgForSend] = useState("");
  const [isKeep, setKeep] = useState(true);

  useEffect(() => {
    if (lastMsg) {
      setMsgs((prev) => prev?.concat({ ...lastMsg }));
    }
  }, [lastMsg]);

  const connectHandler = () => {
    const _ws = new WebSocket(url);
    _ws.addEventListener("message", (ev: MessageEvent<any>) => {
      setLastMsg(newMsg("RECV", ev.data as string));
    });
    _ws.addEventListener("open", () => {
      setLastMsg(newInfoMsg("已连接"));
      setConnected(true);
    });
    _ws.addEventListener("close", () => {
      setLastMsg(newInfoMsg("已断开连接"));
      setConnected(false);
    });
    setWs(_ws);
  };

  const sendHandler = (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
    if (msgForSend) {
      setLastMsg(newMsg("SEND", msgForSend));
      ws?.send(msgForSend);
      if (!isKeep) {
        setMsgForSend("");
      }
    }
  };
  return (
    <>
      <Box>
        <h1 className="text-lg font-bold">ECHO</h1>
      </Box>

      <Box>
        <div className="grid grid-cols-12 items-center">
          <div className="col-span-1 flex flex-col justify-start items-center">
            {isConnected ? (
              <>
                {/* 已连接 */}
                <span className="relative flex h-3 w-3">
                  <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
                  <span className="relative inline-flex rounded-full h-3 w-3 bg-green-500"></span>
                </span>
              </>
            ) : (
              <>
                {/* 未连接 */}
                <span className="relative flex h-3 w-3">
                  <span className="relative inline-flex rounded-full h-3 w-3 bg-red-500"></span>
                </span>
              </>
            )}
          </div>
          <div className="col-span-9">
            <input
              className="border block w-full rounded px-3 py-1"
              readOnly={isConnected}
              value={url}
              onChange={(e) => {
                useCallback(() => setUrl(e.target.value), []);
              }}
            />
          </div>
          <div className="col-span-2 flex flex-col justify-start items-center">
            {isConnected ? (
              <button
                className="border bg-amber-600 text-white px-3 py-1 rounded"
                onClick={(e) => ws?.close()}
              >
                断开
              </button>
            ) : (
              <button
                className="border bg-cyan-600 text-white px-3 py-1 rounded"
                onClick={connectHandler}
              >
                连接
              </button>
            )}
          </div>
        </div>
      </Box>
      <Box>
        <ul className="h-[30rem] overflow-y-auto">
          {msgs.map((m) => (
            <MessageItem key={m.id} msg={m} />
          ))}
        </ul>
      </Box>
      <Box>
        <div className="flex flex-col gap-y-2">
          <div>
            <textarea
              className="border block w-full p-4 rounded"
              rows={1}
              value={msgForSend}
              onChange={(e) => {
                setMsgForSend(e.target.value.trim());
              }}
            ></textarea>
          </div>
          <div className="flex justify-end items-center gap-x-2">
            <label>
              <input
                type="checkbox"
                checked={isKeep}
                onChange={(e) => setKeep(e.target.checked)}
              />{" "}
              保留上次发送的内容
            </label>
            <button
              className="border bg-blue-600 text-white px-3 py-2 rounded disabled:bg-blue-600/50"
              disabled={!(isConnected && msgForSend)}
              onClick={sendHandler}
            >
              发送
            </button>
          </div>
        </div>
      </Box>

      <footer className="text-center text-xs text-gray-400">
        &copy; 2024{" "}
        <a href="https://axum.rs" target="_blank">
          AXUM.RS
        </a>
      </footer>
    </>
  );
}
