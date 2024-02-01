"use client";

import FullMask from "@/components/FullMask";
import LoginForm from "@/components/LoginForm";
import React, { useState } from "react";
import ProfileView from "./Profile";

export default function HomeView() {
  const [token, setToken] = useState<string | undefined>(undefined);

  if (!token) {
    return (
      <>
        {/* 未登录 */}
        <FullMask>
          <LoginForm setToken={setToken} />
        </FullMask>
      </>
    );
  }
  return (
    <>
      {/* 已登录 */}
      <ProfileView token={token} setToken={setToken} />
    </>
  );
}
