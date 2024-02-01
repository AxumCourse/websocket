"use client";

import React, { useEffect, useState } from "react";

export default function ProfileView({
  setToken,
  token,
}: {
  setToken: React.Dispatch<React.SetStateAction<string | undefined>>;
  token?: string;
}) {
  const [ws, setWs] = useState<WebSocket | undefined>(undefined);
  const [userProfile, setUserProfile] = useState<JwtClaimsUserData>({
    id: "",
    email: "",
  });

  useEffect(() => {
    const _ws = new WebSocket("ws://127.0.0.1:56789/check");
    _ws.addEventListener("message", (ev: MessageEvent<any>) => {
      console.log("收到消息：", ev.data);
      const claims: JwtClaims = JSON.parse(ev.data);
      setUserProfile(claims.data);
    });
    _ws.addEventListener("close", () => {
      setToken(undefined);
      console.log("关闭连接");
    });
    _ws.addEventListener("open", () => {
      console.log("连接成功", new Date());
    });
    setWs(_ws);

    return () => {
      if (ws && ws?.readyState === WebSocket.OPEN) {
        ws.close();
        setWs(undefined);
      }
    };
  }, []);

  useEffect(() => {
    const t = setInterval(() => {
      if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send(token!);
      }
    }, 1000);

    return () => {
      if (t) {
        clearInterval(t);
      }
    };
  }, [ws]);

  if (!(userProfile && userProfile.id && userProfile.email)) {
    return <>正在获取数据……</>;
  }

  return (
    <>
      <div className="mb-6">当前用户在线</div>
      <ul>
        <li>ID：{userProfile.id}</li>
        <li>Email: {userProfile.email}</li>
      </ul>
    </>
  );
}
